use crate::extract_resource::ExtractResourcePlugin;
use crate::mesh::RenderMesh;
use crate::render_asset::{prepare_assets, RenderAssets};
use crate::render_resource::{
    CachedRenderPipelineId, PipelineCache, SpecializedMeshPipeline, SpecializedMeshPipelines,
};
use crate::sync_world::{MainEntity, MainEntityHashSet};
use crate::view::{RenderVisibleEntities, VisibleEntities};
use crate::{Extract, ExtractSchedule, Render, RenderApp, RenderSet};
use bevy_app::{App, First, Plugin, PostUpdate};
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::component::Component;
use bevy_ecs::entity::{Entity, EntityHashMap, EntityHashSet};
use bevy_ecs::query::{QueryFilter, QueryItem, ReadOnlyQueryData, With};
use bevy_ecs::schedule::{IntoSystemConfigs, IntoSystemSetConfigs};
use bevy_ecs::system::lifetimeless::Read;
use bevy_ecs::system::{
    Commands, Local, Query, Res, ResMut, Resource, StaticSystemParam, SystemParam, SystemParamItem,
};
use bevy_render_macros::ExtractResource;
use bevy_utils::{tracing::error, HashMap};
use core::marker::PhantomData;
use std::any::TypeId;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};

pub struct SpecializationPlugin<M, NSQF, VK, VC>(PhantomData<(M, NSQF, VK, VC)>);

impl<M, NSQF, VK, VC> Default for SpecializationPlugin<M, NSQF, VK, VC>
where
    M: Sync + Send + 'static,
    NSQF: QueryFilter + Sync + Send + 'static,
    VK: SpecializedViewKey,
    VC: Component,
{
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<M, NSQF, VK, VC> Plugin for SpecializationPlugin<M, NSQF, VK, VC>
where
    M: Sync + Send + 'static,
    NSQF: QueryFilter + Sync + Send + 'static,
    VK: SpecializedViewKey,
    VC: Component,
{
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<ExtractResourcePlugin<ViewKeyCache<VK>>>() {
            app.add_plugins(ExtractResourcePlugin::<ViewKeyCache<VK>>::default())
                .init_resource::<ViewKeyCache<VK>>();
        };

        app
            .add_systems(First, clear_needs_specialization::<NSQF>)
            .add_systems(
                PostUpdate,
                (
                    check_entity_needs_specialization::<NSQF>,
                    check_views_need_specialization::<VK, VC>,
                ),
            );
    }

    fn finish(&self, app: &mut App) {
        if let Some(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app
                .init_resource::<SpecializedMaterialPipelineCache<M>>()
                .init_resource::<EntitiesToSpecialize<M>>()
                .add_systems(ExtractSchedule, extract_needs_specialization::<M>);
        }
    }
}

#[derive(Component)]
pub struct NeedsSpecialization;

#[derive(Resource)]
pub struct EntitiesToSpecialize<M> {
    entities: MainEntityHashSet,
    marker: PhantomData<M>,
}

impl<M> Default for EntitiesToSpecialize<M> {
    fn default() -> Self {
        Self {
            entities: MainEntityHashSet::default(),
            marker: PhantomData,
        }
    }
}

impl<M> Deref for EntitiesToSpecialize<M> {
    type Target = MainEntityHashSet;

    fn deref(&self) -> &Self::Target {
        &self.entities
    }
}

impl<M> DerefMut for EntitiesToSpecialize<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entities
    }
}

pub fn check_entity_needs_specialization<NSQF>(
    mut commands: Commands,
    query: Query<Entity, NSQF>,
) where
    NSQF: QueryFilter,
{
    commands.insert_or_spawn_batch(
        EntityHashSet::from_iter(query.iter())
            .into_iter()
            .map(|entity| (entity, NeedsSpecialization)),
    );
}

fn clear_needs_specialization<M>(
    mut commands: Commands,
    query: Query<Entity, With<NeedsSpecialization>>,
) {
    for entity in &query {
        commands.entity(entity).remove::<NeedsSpecialization>();
    }
}

fn extract_needs_specialization<M>(
    mut entities_to_specialize: ResMut<EntitiesToSpecialize<M>>,
    query: Extract<Query<Entity, With<NeedsSpecialization>>>,
)
where
    M: Sync + Send + 'static,
{
    entities_to_specialize.extend(query.iter().map(MainEntity::from));
}

#[derive(Resource, Deref, DerefMut, ExtractResource, Clone)]
pub struct ViewKeyCache<VK>(EntityHashMap<VK>)
where
    VK: SpecializedViewKey;

impl<VK> Default for ViewKeyCache<VK>
where
    VK: SpecializedViewKey,
{
    fn default() -> Self {
        Self(EntityHashMap::default())
    }
}

pub trait SpecializedViewKey: Clone + PartialEq + Send + Sync + 'static {
    type ViewQueryData: ReadOnlyQueryData + 'static;

    fn get_view_key<'w>(view_query: QueryItem<'w, Self::ViewQueryData>) -> Self;
}

pub fn check_views_need_specialization<VK, VC>(
    mut commands: Commands,
    mut view_key_cache: ResMut<ViewKeyCache<VK>>,
    views: Query<(Entity, &VisibleEntities, VK::ViewQueryData)>,
) where
    VK: SpecializedViewKey,
    VC: Component,
{
    for (view_entity, visible_entities, view_query) in &views {
        let view_key = VK::get_view_key(view_query);

        if let Some(current_key) = view_key_cache.get_mut(&view_entity) {
            if *current_key != view_key {
                *current_key = view_key;
                let batch = visible_entities
                    .iter(TypeId::of::<VC>())
                    .copied()
                    .map(|entity| (entity, NeedsSpecialization))
                    .collect::<Vec<_>>();
                commands.insert_or_spawn_batch(batch);
            }
        } else {
            view_key_cache.insert(view_entity, view_key);
            let batch = visible_entities
                .iter(TypeId::of::<VC>())
                .copied()
                .map(|entity| (entity, NeedsSpecialization))
                .collect::<Vec<_>>();
            commands.insert_or_spawn_batch(batch);
        }
    }
}

#[derive(Resource)]
pub struct SpecializedMaterialPipelineCache<M> {
    map: HashMap<(Entity, MainEntity), CachedRenderPipelineId>,
    marker: PhantomData<M>,
}

impl<M> Default for SpecializedMaterialPipelineCache<M> {
    fn default() -> Self {
        Self {
            map: HashMap::default(),
            marker: PhantomData,
        }
    }
}

impl<M> Deref for SpecializedMaterialPipelineCache<M> {
    type Target = HashMap<(Entity, MainEntity), CachedRenderPipelineId>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl<M> DerefMut for SpecializedMaterialPipelineCache<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

// pub trait SpecializedMaterial: CheckNeedsSpecialization + Send + Sync + 'static {
//     type Param: SystemParam + 'static;
//     type Pipeline: SpecializedMeshPipeline<Key: Send + Sync + 'static> + Resource;
//     type ViewKey: SpecializedViewKey + PartialEq + Send + Sync + 'static;
//     type VisibilityClass: Component;
//
//     fn should_specialize_view(view_entity: Entity, param: &SystemParamItem<Self::Param>) -> bool;
//     fn get_mesh_and_pipeline_key<'w>(
//         entity: (Entity, MainEntity),
//         view_key: &Self::ViewKey,
//         param: &'w SystemParamItem<Self::Param>,
//     ) -> Option<(
//         &'w RenderMesh,
//         <Self::Pipeline as SpecializedMeshPipeline>::Key,
//     )>;
// }
//
// #[allow(clippy::too_many_arguments)]
// fn specialize_pipelines<M>(
//     param: StaticSystemParam<M::Param>,
//     mut entities_to_specialize: ResMut<EntitiesToSpecialize<M>>,
//     pipeline: Res<M::Pipeline>,
//     mut pipelines: ResMut<SpecializedMeshPipelines<M::Pipeline>>,
//     pipeline_cache: Res<PipelineCache>,
//     mut specialized_pipeline_cache: ResMut<SpecializedMaterialPipelineCache<M>>,
//     view_key_cache: Res<ViewKeyCache<M::ViewKey>>,
//     views: Query<(Entity, &MainEntity, &RenderVisibleEntities)>,
// ) where
//     M: SpecializedMaterial,
// {
//     let param = param.into_inner();
//
//     for (view_entity, main_view_entity, visible_entities) in &views {
//         if !M::should_specialize_view(view_entity, &param) {
//             continue;
//         }
//         let Some(view_key) = view_key_cache.get(&main_view_entity.id()) else {
//             continue;
//         };
//
//         for (render_entity, visible_entity) in visible_entities.iter::<M::VisibilityClass>() {
//             if entities_to_specialize.entities.contains(visible_entity) {
//                 let Some((mesh, pipeline_key)) = M::get_mesh_and_pipeline_key(
//                     (*render_entity, *visible_entity),
//                     &view_key,
//                     &param,
//                 ) else {
//                     continue;
//                 };
//                 let pipeline_id =
//                     pipelines.specialize(&pipeline_cache, &pipeline, pipeline_key, &mesh.layout);
//
//                 let pipeline_id = match pipeline_id {
//                     Ok(id) => id,
//                     Err(err) => {
//                         error!("{}", err);
//                         continue;
//                     }
//                 };
//
//                 specialized_pipeline_cache.insert((view_entity, *visible_entity), pipeline_id);
//                 entities_to_specialize.remove(visible_entity);
//             }
//         }
//     }
// }

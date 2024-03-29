use bevy::prelude::{Query, With, Or, Entity, Commands, DespawnRecursiveExt, Res, State};

use crate::{lib::tools::markers::PlayerParentMarker, PlayerState};

use super::components::{GltfFileMarker, MainSceneMarker};

pub fn unload(
    mut commands: Commands,
    query: Query<Entity, Or<(With<GltfFileMarker>,With<MainSceneMarker>, With<PlayerParentMarker>)>>,
) {
    
    for i in query.iter() {
        commands.entity(i).despawn_recursive();
    }
}
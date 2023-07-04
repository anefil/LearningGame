use std::path::Path;

use bevy::{prelude::{Query, With, Res, Input, KeyCode, Commands, NodeBundle, BuildChildren, TextBundle, Color, AssetServer, EventReader, ResMut}, ui::{Style, Size, Val, JustifyContent}, window::FileDragAndDrop, scene::Scene};

use super::{player_extensions::PlayerSettings, markers::{PlayerParentMarker, AddingObjectUiMarker}};


// pub fn object_dialogue_window(
//     mut commands: Commands,
//     mut player_query: Query<&mut PlayerSettings, With<PlayerParentMarker>>,
//     asset_server: Res<AssetServer>,
//     keys: Res<Input<KeyCode>>
// ) {
//     if keys.just_pressed(KeyCode::A) && 
//         (keys.pressed(KeyCode::LShift) || keys.pressed(KeyCode::RShift)) {
        
//         commands.spawn(NodeBundle {
//                 style: Style {
//                     size: Size::width(Val::Percent(70.0)),
//                     justify_content: JustifyContent::SpaceBetween,
//                     ..Default::default()
//                 },
//                 ..Default::default()
//         })
//         .insert(AddingObjectUiMarker)
//         .with_children(|p| {
//             p.spawn(TextBundle::from_section(
//                 "Example",
//                 bevy::text::TextStyle { 
//                     font: asset_server.load("fonts/FiraSans-Bold.ttf"), 
//                     font_size: 30.0, 
//                     color: Color::WHITE 
//                 }
//             ));
//         });
//     }
// }

// pub fn dnd_gltf_scene(
//     // mut commands: Commands,
//     mut dnd_evr: EventReader<FileDragAndDrop>,
//     mut asset_server: ResMut<AssetServer>
// ) {
    
//     for ev in dnd_evr.iter() {
//         println!("{:?}",ev);
//         let FileDragAndDrop::DroppedFile { window, path_buf } = ev else {
//             continue;
//         };
//         asset_server.load::<Scene, &Path>(path_buf.as_path());
//     }
// }
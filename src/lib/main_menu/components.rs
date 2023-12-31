use std::ffi::OsString;

use bevy::prelude::{Component, Resource, Color};

use crate::lib::tools::transition::TransitionMarker;

#[derive(Component)]
pub struct MainMenuMarker;


#[derive(Component)]
pub struct Level(pub OsString);

#[derive(Component)]
pub struct ButtonColors(pub Color,pub Color, pub Color);


#[derive(Clone, Copy)]
pub enum MainMenuVariants {
    Main,
    Settings,
    Levels
}

impl MainMenuVariants {
    pub fn position(&self,) -> (f32,f32) {
        match *self {
            MainMenuVariants::Main => (0.0,0.0),
            MainMenuVariants::Settings => (1.0,0.0),
            MainMenuVariants::Levels => (0.0,1.0),
        }
    }
}

pub enum MainMenuButtonEnum {
    MainMenu,
    StartGame,
    Settings,
    Exit,
}

pub enum SettingsButtonEnum {
    VolumeUp(u64),
    VolumeDown(u64),
}

#[derive(Component)]
pub struct MainMenuButtonMarker(pub MainMenuButtonEnum);

#[derive(Component)]
pub struct SettingsButtonMarker(pub SettingsButtonEnum);

#[derive(Component)]
pub struct SettingsVolumeLabel;

#[derive(Component)]
pub struct RootNode;

use std::{time::Duration, fs::{File, self}, io::BufReader, path::PathBuf, env};

use bevy::{
    prelude::{
        AssetServer, BuildChildren, Button, ButtonBundle, Camera2dBundle, Changed, Color, Commands,
        Component, DespawnRecursiveExt, Entity, ImageBundle, NodeBundle, Query, Res,
        ResMut, State, TextBundle, With, Without, EventWriter, EventReader
    },
    text::{ TextStyle, TextSection, BreakLineOn},
    time::Time,
    ui::{
        AlignItems, BackgroundColor, FlexWrap, FlexDirection, Interaction, JustifyContent, PositionType,
        Style, UiRect, Val, AlignSelf, UiImage
    },
    input::mouse::MouseWheel
};
use bevy_kira_audio::{Audio, AudioControl};


use crate::{
    lib::tools::{consts::{styles, font_names, self}, transition::TransitionMarker, resources::{MainMenuResource, AllSettings}, events::LoadLevel, config::LevelSchema, self},
    GameState, 
};

use super::components::{
    MainMenuButtonEnum, MainMenuButtonMarker, MainMenuMarker, RootNode, MainMenuVariants, Level, SettingsButtonMarker, SettingsButtonEnum, SettingsLabel, ButtonColors, QuickFixImageComponentMarker
};

pub fn button_interactivity(
    mut main_menu_res: ResMut<MainMenuResource>,
    mut roots: Query<&mut Style, With<RootNode>>,
    mut main_menu_buttons: Query<
        (&Interaction, &mut BackgroundColor, &MainMenuButtonMarker, Option<&ButtonColors>),
        (Changed<Interaction>, With<Button>, Without<SettingsButtonMarker>),
    >,
    mut settings_buttons: Query< // TODO: move it to another function
        (&Interaction, &mut BackgroundColor, &SettingsButtonMarker, Option<&ButtonColors>),
        (Changed<Interaction>, With<Button>, Without<MainMenuButtonMarker>),
    >,
    audio: ResMut<Audio>,
    mut player: ResMut<AllSettings>,
    mut text: Query<(&mut bevy::text::Text, &SettingsLabel),(Without<MainMenuButtonMarker>,Without<SettingsButtonMarker>)>,
    state: Res<State<GameState>>,
    // mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
) {
    if state.get() == &GameState::MainMenu { // TODO: why is it here again?
        for (interaction, mut color, button_marker, colors) in &mut main_menu_buttons {
            if *interaction == Interaction::Pressed {
                match &button_marker.0 {
                    MainMenuButtonEnum::MainMenu => {
                        if main_menu_res.current_position == MainMenuVariants::Settings {
                            let path = if let Ok(manifest_dir) = env::var("BEVY_ASSET_ROOT") {
                                PathBuf::from(manifest_dir)
                            } else if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
                                PathBuf::from(manifest_dir)
                            } else {
                                env::current_exe()
                                    .map(|path| {
                                        path.parent()
                                            .map(|exe_parent_path| exe_parent_path.to_owned())
                                            .unwrap()
                                    })
                                    .unwrap()
                            };
                        
                            let path = path.join("assets/data");
                            let path = path.join("settings.json");
                            let res = fs::write(path, serde_json::to_string(player.as_ref()).expect("couldn't serialize settings into settings.json"));
                            match res {
                                Ok(_) => {},
                                Err(err) => println!("{}",err)
                            }
                        }
                        main_menu_res.transition_proccess.started = true;
                        main_menu_res.next_position = MainMenuVariants::Main;
                        main_menu_res.transition_proccess.timer.reset();
                    }
                    MainMenuButtonEnum::StartGame => {
                        main_menu_res.transition_proccess.started = true;
                        main_menu_res.next_position = MainMenuVariants::Levels;
                        main_menu_res.transition_proccess.timer.reset();
                    }
                    MainMenuButtonEnum::Settings => {
                        main_menu_res.transition_proccess.started = true;
                        main_menu_res.next_position = MainMenuVariants::Settings;
                        main_menu_res.transition_proccess.timer.reset();
                    }
                    MainMenuButtonEnum::About => {
                        main_menu_res.transition_proccess.started = true;
                        main_menu_res.next_position = MainMenuVariants::About;
                        main_menu_res.transition_proccess.timer.reset();
                    },
                    MainMenuButtonEnum::Exit => {

                    }
                } 
            }
            match *interaction {
                Interaction::Pressed => {
                    color.0 = colors.and_then(|x| Some(x.2)).or(Some(styles::button::BUTTON_ACTIVE.into())).unwrap();
                }
                Interaction::Hovered => {
                    color.0 = colors.and_then(|x| Some(x.1)).or(Some(styles::button::BUTTON_HOVER.into())).unwrap();
                }
                Interaction::None => {
                    color.0 = colors.and_then(|x| Some(x.0)).or(Some(styles::button::BUTTON_DEFAULT.into())).unwrap();
                }
            }
        }

        for (interaction, mut color, button_marker,_) in &mut settings_buttons {
            match (&button_marker.0, *interaction) {
                (SettingsButtonEnum::VolumeDown(v), Interaction::Pressed) => {
                    player.volume -= (*v as f64) / 100.;
                    player.volume = (player.volume*100.).round()/100.;
                    audio.set_volume(player.volume);
                    for mut t in &mut text {
                        if t.1.0 == super::components::SettingsLabelEnum::Volume {
                            t.0.sections[0].value = format!("{:.2}",player.volume);
                        }
                    }
                }
                (SettingsButtonEnum::VolumeUp(v), Interaction::Pressed) => {
                    player.volume += (*v as f64) / 100.;
                    player.volume = (player.volume*100.).round()/100.;
                    audio.set_volume(player.volume);
                    for mut t in &mut text {
                        if t.1.0 == super::components::SettingsLabelEnum::Volume {
                            t.0.sections[0].value = format!("{:.2}",player.volume);
                        }
                    }
                }
                (SettingsButtonEnum::FovChange(v), Interaction::Pressed) => {
                    player.fov += *v as f32;
                    player.fov = player.fov.clamp(50.,120.);
                    for mut t in &mut text {
                        if t.1.0 == super::components::SettingsLabelEnum::Fov {
                            t.0.sections[0].value = format!("{:.0}",player.fov);
                        }
                    }
                }
                (SettingsButtonEnum::VolumeUp(_), _) => {},
                (SettingsButtonEnum::VolumeDown(_), _) => {},
                (SettingsButtonEnum::FovChange(_), _) => {},
            }
            match *interaction {
                Interaction::Pressed => {
                    *color = styles::button::SETTINGS_BUTTON_ACTIVE.into();
                }
                Interaction::Hovered => {
                    *color = styles::button::SETTINGS_BUTTON_HOVER.into();
                }
                Interaction::None => {
                    *color = styles::button::SETTINGS_BUTTON_DEFAULT.into();
                }
            }
        }

        if main_menu_res.transition_proccess.started { // todo move into another function
            let p;
            if main_menu_res.transition_proccess.tick(time.delta()) {
                main_menu_res.transition_proccess.started = false;
                main_menu_res.current_position = main_menu_res.next_position;
                p = 100.;
            } else {
                p = main_menu_res.transition_proccess.ease_in_out().unwrap() * 100.;
            }
            
                let a = main_menu_res.current_position.position();
                let b = main_menu_res.next_position.position();

                

            roots.for_each_mut(|mut el| {
                el.left = Val::Percent(- (100.- p) * a.0 - p * b.0);
                el.top = Val::Percent(- (100. - p) * a.1 - p * b.1);
            });
        }
    }
}

pub fn level_interactivity(
    // mut main_menu_res: ResMut<MainMenuResource>,
    // mut roots: Query<&mut Style, With<RootNode>>,
    mut button_interaction: Query<
        (&Interaction, &mut BackgroundColor, &Level),
        (Changed<Interaction>, With<Button>),
    >,
    // state: Res<State<GameState>>,
    // mut next_state: ResMut<NextState<GameState>>,
    mut ev: EventWriter<LoadLevel>
) {
    for (interaction, mut color, button_marker) in &mut button_interaction {
         
        match *interaction {
            Interaction::Pressed => {
                *color = styles::button::BUTTON_ACTIVE.into();
                ev.send(LoadLevel { string: button_marker.0.clone() });
            }
            Interaction::Hovered => {
                *color = styles::button::BUTTON_HOVER.into();
            }
            Interaction::None => {
                *color = styles::button::BUTTON_DEFAULT.into();
            }
        }
    }
}



#[derive(Component)]
pub struct MainMenuImageMarker;

pub fn prepare_main_menu(mut commands: Commands, asset_server: Res<AssetServer>, settings: Res<AllSettings>) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(MainMenuMarker);

    commands.insert_resource(MainMenuResource {
        current_position: super::components::MainMenuVariants::Main,
        next_position: super::components::MainMenuVariants::Main,
        transition_proccess: TransitionMarker::new(false, Duration::from_secs_f32(1.8))
    });

    let mut level_paths: Vec<(PathBuf,LevelSchema)> = vec![];

    {
        // use std::{env, path::{PathBuf}};

        let path = if let Ok(manifest_dir) = env::var("BEVY_ASSET_ROOT") {
            PathBuf::from(manifest_dir)
        } else if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
            PathBuf::from(manifest_dir)
        } else {
            env::current_exe()
                .map(|path| {
                    path.parent()
                        .map(|exe_parent_path| exe_parent_path.to_owned())
                        .unwrap()
                })
                .unwrap()
        };

        let path = path.join("assets/levels");


        for p in path.read_dir().unwrap() {
            if let Ok(entry) = p {
                if entry.path().is_dir() && entry.path().read_dir().unwrap().find(|p| p.as_ref().is_ok_and(|p| {p.file_name().eq("main.gltf")})).is_some() {
                    if let Ok(file) = File::open(entry.path().join("config.json")) {
                        let reader = BufReader::new(file);
                        let mut u = serde_json::from_reader::<_,LevelSchema>(reader).unwrap();
                        if u.prioritize == 0 {
                            u.prioritize = u32::MAX;
                        }
                        level_paths.push((entry.path(),u));
                    } else {
                        level_paths.push((entry.path(),LevelSchema {
                            name: entry.path().to_string_lossy().to_owned().to_string(),
                            version: 1,
                            prioritize: u32::MAX
                        }));
                    }
                }
            }
        }

        level_paths.sort_by(|a,b| a.1.prioritize.cmp(&b.1.prioritize));

        for el in level_paths.iter() {
            println!("{}: {}", el.1.name, el.1.prioritize);
        }

        // let _ = "a";


    }

    let button_style = Style {
        width: Val::Percent(100.),
        // height: Val::Px(200.0),
        // border: bevy::ui::UiRect::all(Val::Px(2.0)),
        padding: UiRect::all(Val::Px(10.)),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..Default::default()
    };

    let button_text_style = TextStyle {
        font: asset_server.load(font_names::NOTO_SANS_MEDIUM),
        font_size: 40.0,
        color: Color::BLACK,
    };

    let main_screen_bundle = NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            position_type: PositionType::Absolute,
            padding: UiRect::left(Val::Px(30.)),
            ..Default::default()
        },
        ..Default::default()
    };

    let main_screen_bg = ImageBundle {
        image: bevy::ui::UiImage {
            texture: asset_server.load("internal/splash/main_screen.png"),
            ..Default::default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            min_width: Val::Vw(100.),
            min_height: Val::Vh(100.),
            width: Val::Px(0.01),
            height: Val::Px(0.01),
            aspect_ratio: Some(1920. / 1080.),
            left: Val::Px(0.),
            top: Val::Px(0.),
            ..Default::default()
        },
        ..Default::default()
    };

    let mut text_heading = TextBundle::from_section(
        "Віртуальна гра 'Подорож з фізикою'",
        TextStyle {
            font_size: 64.,
            color: Color::BLACK,
            font: asset_server.load(font_names::NOTO_SANS_EX_BOLD),
        },
    );
    text_heading.style.margin = UiRect::vertical(Val::Px(90.));

    let main_screen_button_group_node = NodeBundle {
        style: Style {
            // width: Val::Percent(100.),
            // height: Val::Auto,
            align_items: AlignItems::Start,
            flex_direction: FlexDirection::Column,
            // row_gap: Val::Px(20.),
            // padding: UiRect::horizontal(Val::Px(15.)),
            // margin: UiRect::left(Val::Px(60.)),
            ..Default::default()
        },
        background_color: consts::styles::button::TRANSPARENT_WHITE.into(),
        ..Default::default()
    };

    let main_screen_button_group_delimeter_node = NodeBundle {
        style: Style {
            width: Val::Percent(70.),
            height: Val::Px(2.),
            margin: UiRect::horizontal(Val::Percent(15.)),
            ..Default::default()
        },
        background_color: Color::rgba(0.0,0.0,0.0,0.6).into(),
        ..Default::default()
    };

    let goto_level_picker = (
        ButtonBundle {
            style: button_style.clone(),
            ..Default::default()
        },
        MainMenuButtonMarker(MainMenuButtonEnum::StartGame)
    );

    let goto_level_picker_text = TextBundle::from_section(
        "Почати",
        button_text_style.clone()
    );

    let goto_about_picker = (
        ButtonBundle {
            style: button_style.clone(),
            ..Default::default()
        },
        MainMenuButtonMarker(MainMenuButtonEnum::About)
    );

    let goto_about_picker_text = TextBundle::from_section(
        "Посібник користувача",
        button_text_style.clone()
    );

    let goto_settings_picker = (
        ButtonBundle {
            style: button_style.clone(),
            ..Default::default()
        },
        MainMenuButtonMarker(MainMenuButtonEnum::Settings)
    );

    let goto_settings_picker_text = TextBundle::from_section(
        "Налаштування",
        button_text_style.clone()
    );

    let settings_node = NodeBundle { 
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            padding: UiRect::horizontal(Val::Percent(20.)),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceEvenly,
            margin: UiRect::left(Val::Vw(100.)),
            position_type: PositionType::Absolute,
            ..Default::default()
        },
        background_color: styles::button::BUTTON_ACTIVE.into(),
        ..Default::default()
    };

    let help_node = NodeBundle { 
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            padding: UiRect::horizontal(Val::Percent(20.)),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceEvenly,
            margin: UiRect::new(Val::Vw(100.),Val::ZERO,Val::Vh(100.),Val::ZERO),
            position_type: PositionType::Absolute,
            ..Default::default()
        },
        background_color: styles::button::BUTTON_ACTIVE.into(),
        ..Default::default()
    };

    let goto_back_main = (
        ButtonBundle {
            style: Style {
                padding: UiRect::axes(Val::Px(20.), Val::Px(10.)),
                ..Default::default()
            },
            background_color: BackgroundColor(Color::WHITE),
            ..Default::default()
        },
        MainMenuButtonMarker(MainMenuButtonEnum::MainMenu),
        ButtonColors(styles::button::SETTINGS_BUTTON_DEFAULT,styles::button::SETTINGS_BUTTON_HOVER,styles::button::SETTINGS_BUTTON_ACTIVE)
    );

    let goto_back_main_2 = (
        ButtonBundle {
            style: Style {
                padding: UiRect::axes(Val::Px(20.), Val::Px(10.)),
                ..Default::default()
            },
            background_color: BackgroundColor(Color::WHITE),
            ..Default::default()
        },
        MainMenuButtonMarker(MainMenuButtonEnum::MainMenu),
        ButtonColors(styles::button::SETTINGS_BUTTON_DEFAULT,styles::button::SETTINGS_BUTTON_HOVER,styles::button::SETTINGS_BUTTON_ACTIVE)
    );

    commands // MAIN SCREEN
        .spawn(main_screen_bundle)
        .insert((RootNode,MainMenuMarker))
        .with_children(|parent| {
            parent.spawn(main_screen_bg);
            parent.spawn(text_heading);

            parent
                .spawn(main_screen_button_group_node)
                .with_children(|parent| {
                    parent
                        .spawn(goto_level_picker)
                        .with_children(|p| {
                            p.spawn(goto_level_picker_text);
                        });

                    parent
                        .spawn(main_screen_button_group_delimeter_node.clone());

                    parent
                        .spawn(goto_about_picker)
                        .with_children(|p| {
                            p.spawn(goto_about_picker_text);
                        });

                    parent
                        .spawn(main_screen_button_group_delimeter_node.clone());

                    parent
                        .spawn(goto_settings_picker)
                        .with_children(|p| {
                            p.spawn(goto_settings_picker_text);
                        });
                });
        });

    commands // SETTINGS
        .spawn(settings_node)
        .insert((RootNode,MainMenuMarker))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: bevy::ui::UiImage {
                    texture: asset_server.load("internal/splash/main_screen_blur.png"),
                    ..Default::default()
                },
                style: Style {
                    position_type: PositionType::Absolute,
                    min_width: Val::Vw(100.),
                    min_height: Val::Vh(100.),
                    width: Val::Px(0.01),
                    height: Val::Px(0.01),
                    aspect_ratio: Some(1920. / 1080.),
                    left: Val::Px(0.),
                    top: Val::Px(0.),
                    ..Default::default()
                },
                ..Default::default()
            });

            parent
                .spawn(goto_back_main)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Назад", button_text_style.clone()));
                });

            parent.spawn(NodeBundle {
                style: Style {
                    border: UiRect::all(Val::Px(4.)),
                    ..Default::default()
                },
                border_color: bevy::ui::BorderColor(Color::BLACK),
                ..Default::default()
            }).with_children(|parent| {
                let mut volume_label = TextBundle::from_section("Гучність: ", button_text_style.clone());
                volume_label.style.padding = UiRect::all(Val::Px(10.));
                let mut volume_value_label = TextBundle::from_section(format!("{}",settings.volume), button_text_style.clone());
                volume_value_label.style.padding = UiRect::all(Val::Px(10.));

                parent
                    .spawn(volume_label)
                    .insert(BackgroundColor(Color::WHITE));
                parent.spawn(bevy::ui::node_bundles::ButtonBundle {
                    style: Style {
                        width: Val::Px(40.),
                        height: Val::Px(40.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(10.)),
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::WHITE),
                    ..Default::default()
                })
                .insert(SettingsButtonMarker(super::components::SettingsButtonEnum::VolumeDown(2)))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("-",button_text_style.clone()));
                });
                parent
                    .spawn(volume_value_label)
                    .insert(BackgroundColor(Color::WHITE))
                    .insert(SettingsLabel(super::components::SettingsLabelEnum::Volume));

                parent.spawn(bevy::ui::node_bundles::ButtonBundle {
                    style: Style {
                        width: Val::Px(40.),
                        height: Val::Px(40.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(10.)),
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::WHITE),
                    ..Default::default()
                })
                .insert(SettingsButtonMarker(super::components::SettingsButtonEnum::VolumeUp(2)))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("+",button_text_style.clone()));
                });
            });

            parent.spawn(NodeBundle {
                style: Style {
                    border: UiRect::all(Val::Px(4.)),
                    ..Default::default()
                },
                border_color: bevy::ui::BorderColor(Color::BLACK),
                ..Default::default()
            }).with_children(|parent| {
                let mut fov_label = TextBundle::from_section("Кут огляду:", button_text_style.clone());
                fov_label.style.padding = UiRect::all(Val::Px(10.));
                let mut fov_value_label = TextBundle::from_section(format!("{}",settings.fov), button_text_style.clone());
                fov_value_label.style.padding = UiRect::all(Val::Px(10.));

                parent
                    .spawn(fov_label)
                    .insert(BackgroundColor(Color::WHITE));
                parent.spawn(bevy::ui::node_bundles::ButtonBundle {
                    style: Style {
                        width: Val::Px(40.),
                        height: Val::Px(40.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(10.)),
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::WHITE),
                    ..Default::default()
                })
                .insert(SettingsButtonMarker(super::components::SettingsButtonEnum::FovChange(-2)))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("-",button_text_style.clone()));
                });
                parent
                    .spawn(fov_value_label)
                    .insert(BackgroundColor(Color::WHITE))
                    .insert(SettingsLabel(super::components::SettingsLabelEnum::Fov));

                parent.spawn(bevy::ui::node_bundles::ButtonBundle {
                    style: Style {
                        width: Val::Px(40.),
                        height: Val::Px(40.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(10.)),
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::WHITE),
                    ..Default::default()
                })
                .insert(SettingsButtonMarker(super::components::SettingsButtonEnum::FovChange(2)))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("+",button_text_style.clone()));
                });
            });
        });

        commands // HELP
        .spawn(help_node)
        .insert((RootNode,MainMenuMarker))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: bevy::ui::UiImage {
                    texture: asset_server.load("internal/splash/main_screen_blur.png"),
                    ..Default::default()
                },
                style: Style {
                    position_type: PositionType::Absolute,
                    min_width: Val::Vw(100.),
                    min_height: Val::Vh(100.),
                    width: Val::Px(0.01),
                    height: Val::Px(0.01),
                    aspect_ratio: Some(1920. / 1080.),
                    left: Val::Px(0.),
                    top: Val::Px(0.),
                    ..Default::default()
                },
                ..Default::default()
            });

            parent
                .spawn(goto_back_main_2)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Назад", button_text_style.clone()));
                });

            let style = TextStyle {
                font: asset_server.load(font_names::NOTO_SANS_SM_BOLD),
                font_size: 32.,
                color: Color::BLACK
            };

            let hint_style = TextStyle {
                font: asset_server.load(font_names::NOTO_SANS_BLACK_I),
                font_size: 32.,
                color: Color::RED
            };

            parent
                .spawn(TextBundle::from_sections(
                    [
                        TextSection::new("Вітаємо Вас, шановний гравцю, у чарівному світі фізики! За допомогою цієї \
                                            віртуальної лабораторії ви зможете провести різноманітні експерименти і \
                                            перевірити власні знання. \n", style.clone()),
                        TextSection::new("Щоб усішно керувати простором, пропонуємо вивчити кнопки управління: \n - Кнопка ", style.clone()),
                        TextSection::new("W/A/S/D", hint_style.clone()),
                        TextSection::new(" викориутстовується для пересування у просторі \n - ", style.clone()),
                        TextSection::new("Миша", hint_style.clone()),
                        TextSection::new(" викориутстовується для повороту камери \n - Кнопка ", style.clone()),
                        TextSection::new("Space", hint_style.clone()),
                        TextSection::new(" викориутстовується для стрибка \n - Кнопка  ", style.clone()),
                        TextSection::new("E", hint_style.clone()),
                        TextSection::new(" викориутстовується для взаємодії з об'єктами \n - Кнопка ", style.clone()),
                        TextSection::new("Escape", hint_style.clone()),
                        TextSection::new(" викориутстовується для виходу на головний екран \n", style.clone()),
                    ]
                )).insert(BackgroundColor(tools::consts::styles::button::LESS_TRANSPARENT_WHITE));
            
            parent.spawn(ImageBundle {
                image: UiImage { texture: asset_server.load("internal/textures/keyboard-layout.png"), flip_x: false, flip_y: false },
                ..Default::default()
            });
        });

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                padding: UiRect::all(Val::Px(60.)),
                display: bevy::ui::Display::Flex,
                flex_direction: FlexDirection::Column,
                margin: UiRect::top(Val::Vh(100.)),
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            background_color: styles::button::BUTTON_ACTIVE.into(),
            ..Default::default()
        })
        .insert(RootNode)
        .insert(MainMenuMarker)
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: bevy::ui::UiImage {
                    texture: asset_server.load("internal/splash/main_screen_blur.png"),
                    ..Default::default()
                },
                style: Style {
                    position_type: PositionType::Absolute,
                    min_width: Val::Vw(100.),
                    min_height: Val::Vh(100.),
                    width: Val::Px(0.01),
                    height: Val::Px(0.01),
                    aspect_ratio: Some(1920. / 1080.),
                    left: Val::Px(0.),
                    top: Val::Px(0.),
                    ..Default::default()
                },
                ..Default::default()
            });

            let mut text_heading = TextBundle::from_section(
                "Список лекцій",
                TextStyle {
                    font_size: 58.,
                    color: Color::BLACK,
                    font: asset_server.load(font_names::NOTO_SANS_BOLD),
                },
            );
            text_heading.style.margin = UiRect::vertical(Val::Px(35.));
            text_heading.style.align_self = AlignSelf::Center;
            parent.spawn(text_heading);

            parent.spawn((NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    display: bevy::ui::Display::Flex,
                    flex_wrap: FlexWrap::Wrap,
                    // grid_auto_flow: bevy::ui::GridAutoFlow::Row,
                    // grid_template_columns: vec![GridTrack::flex(1.0),GridTrack::flex(1.0),GridTrack::flex(1.0),GridTrack::flex(1.0)],
                    // grid_template_rows: vec![GridTrack::flex(1.0),GridTrack::flex(1.0),GridTrack::flex(1.0)],
                    // row_gap: Val::Px(20.),
                    // column_gap: Val::Px(20.),
                    ..Default::default()
                },
                background_color: BackgroundColor(consts::styles::button::TRANSPARENT_WHITE),
                ..Default::default()
            }, QuickFixImageComponentMarker)).with_children(|parent| {
                let text_style = TextStyle {
                    font_size: 32.,
                    color: Color::BLACK,
                    font: asset_server.load(font_names::NOTO_SANS_SM_BOLD),
                };

                for p in level_paths {
                    
                    parent.spawn(ButtonBundle {
                        style: Style {
                            width: Val::Percent(25.),
                            flex_grow: 0.,
                            flex_shrink: 0.,
                            // min_height: Val::Px(150.),
                            flex_direction: FlexDirection::Column,
                            ..Default::default()
                        },
                        background_color: BackgroundColor(Color::ORANGE_RED),
                        ..Default::default()
                    }).insert(Level(p.0.components().last().unwrap().as_os_str().to_os_string())).with_children(|el| {
                        el.spawn(ImageBundle {
                            image: UiImage::new(asset_server.load(
                                if p.0.clone().join("preview.png").exists() {
                                    p.0.clone().join("preview.png")
                                } else {
                                    "internal/splash/preview.png".into()
                                })),
                            style: Style {
                                width: Val::Percent(100.),
                                height: Val::Auto,
                                aspect_ratio: Some(1.),
                                ..Default::default()
                            },
                            ..Default::default()
                        });

                        let mut text = TextBundle::from_section(p.1.name, text_style.clone());
                        text.style.justify_content = JustifyContent::Center;
                        text.style.width = Val::Percent(80.);
                        text.style.margin = UiRect {
                            left: Val::Percent(10.),
                            right: Val::Percent(10.),
                            top: Val::Px(20.),
                            ..Default::default()
                        };
                        text.text.alignment = bevy::text::TextAlignment::Center;
                        text.text.linebreak_behavior = BreakLineOn::AnyCharacter;
                        el.spawn(text);
                    });
                    println!("{}",p.0.display());
                }
            });
        });
}

pub fn destroy_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuMarker>>) {
    for main_menu_entity in &query {
        commands.entity(main_menu_entity).despawn_recursive();
    }
    commands.remove_resource::<MainMenuResource>();
}


pub fn fix_images(
    mut scroll_evr: EventReader<MouseWheel>, 
    main_menu_res: Res<MainMenuResource>,
    mut query: Query<&mut Style, With<QuickFixImageComponentMarker>>) {
    use bevy::input::mouse::MouseScrollUnit;

    if main_menu_res.current_position == MainMenuVariants::Levels { 
        for ev in scroll_evr.read() {
            let (_,y) = match ev.unit {
                MouseScrollUnit::Line => {
                    (ev.x * 21., ev.y * 21.)
                }
                MouseScrollUnit::Pixel => {
                    (ev.x * 24., ev.y * 24.)
                }
            };

            for mut style in &mut query {
                let mt = match style.margin.top {
                    Val::Px(v) => v,
                    _ => 0.
                };

                style.margin.top = Val::Px((y + mt).min(0.));
            }
        }
    }
}
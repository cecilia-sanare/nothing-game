use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::{MouseButtonInput, MouseWheel};
use bevy::prelude::*;
use bevy::window::close_on_esc;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    NotStarted,
    InProgress,
    GameOver
}

#[derive(Resource)]
pub struct GameTime {
    start_time: f64,
}

impl Default for GameTime {
    fn default() -> Self {
        GameTime {
            start_time: 0.0,
        }
    }
}

#[derive(Component)]
pub struct NothingTextParent;

#[derive(Component)]
pub struct NothingText;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::hex("303030").unwrap()))
        .insert_resource(GameTime::default())
        .add_plugins(DefaultPlugins)
        .add_state::<GameState>()
        .add_systems(Startup, spawn_camera)
        .add_systems(OnExit(GameState::NotStarted), destroy_text)
        .add_systems(OnExit(GameState::InProgress), destroy_text)
        .add_systems(OnExit(GameState::GameOver), destroy_text)
        .add_systems(OnEnter(GameState::NotStarted), spawn_not_started_text)
        .add_systems(OnEnter(GameState::InProgress), spawn_in_progress_text)
        .add_systems(OnEnter(GameState::GameOver), spawn_game_over_text)
        .add_systems(Update, close_on_esc)
        .add_systems(Update, not_started_update.run_if(in_state(GameState::NotStarted)))
        .add_systems(Update, in_progress_update.run_if(in_state(GameState::InProgress)))
        .add_systems(Update, game_over_update.run_if(in_state(GameState::GameOver)))
        .add_systems(OnEnter(GameState::GameOver), game_over_text)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_not_started_text(mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                min_width: Val::Vw(100.0),
                min_height: Val::Vh(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        },
        NothingTextParent
    )).with_children(|parent| {
        parent.spawn((
            TextBundle::from_sections([
                TextSection::new(
                    "Press any key to start doing ",
                    TextStyle {
                        font_size: 50.0,
                        color: Color::hex("818181").unwrap(),
                        ..default()
                    },
                ),
                TextSection::new(
                    "Nothing",
                    TextStyle {
                        font_size: 50.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
            ])
            .with_text_alignment(TextAlignment::Center),
            NothingText
        ));
    });
}

fn spawn_in_progress_text(mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                min_width: Val::Vw(100.0),
                min_height: Val::Vh(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        },
        NothingTextParent
    )).with_children(|parent| {
        parent.spawn((
            TextBundle::from_sections([
                TextSection::new(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    "You have been doing ",
                    TextStyle {
                        font_size: 50.0,
                        color: Color::hex("818181").unwrap(),
                        ..default()
                    },
                ),
                TextSection::new(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    "Nothing",
                    TextStyle {
                        font_size: 50.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                TextSection::new(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    " for ",
                    TextStyle {
                        font_size: 50.0,
                        color: Color::hex("818181").unwrap(),
                        ..default()
                    },
                ),
                TextSection::new(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    "0 seconds",
                    TextStyle {
                        font_size: 50.0,
                        color: Color::hex("818181").unwrap(),
                        ..default()
                    },
                )
            ])
            .with_text_alignment(TextAlignment::Center),
            NothingText
        ));
    });
}

fn spawn_game_over_text(
    time: Res<Time>,
    game_time: Res<GameTime>,
    mut commands: Commands
) {
    let time_elapsed = (time.elapsed_seconds_f64() - game_time.start_time).floor() as i32;

    commands.spawn((
        NodeBundle {
            style: Style {
                min_width: Val::Vw(100.0),
                min_height: Val::Vh(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        },
        NothingTextParent
    )).with_children(|parent| {
        parent.spawn((
            TextBundle::from_sections([
                TextSection::new(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    "You did ",
                    TextStyle {
                        font_size: 50.0,
                        color: Color::hex("818181").unwrap(),
                        ..default()
                    },
                ),
                TextSection::new(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    "Something",
                    TextStyle {
                        font_size: 50.0,
                        color: Color::BLACK,
                        ..default()
                    },
                ),
                TextSection::new(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    ", you lost\nYou did ",
                    TextStyle {
                        font_size: 50.0,
                        color: Color::hex("818181").unwrap(),
                        ..default()
                    },
                ),
                TextSection::new(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    "Nothing",
                    TextStyle {
                        font_size: 50.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                TextSection::new(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    format!(" for {}", format_time(time_elapsed)),
                    TextStyle {
                        font_size: 50.0,
                        color: Color::hex("818181").unwrap(),
                        ..default()
                    },
                ),
            ]).with_text_alignment(TextAlignment::Center),
            NothingText
        ));
    });
}

fn destroy_text(mut commands: Commands, mut query: Query<Entity, With<NothingTextParent>>) {
    for entity in query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
}

fn not_started_update(
    keys: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    time: Res<Time>,
    mut game_time: ResMut<GameTime>,
    mut state: ResMut<NextState<GameState>>,
    key_evr: EventReader<KeyboardInput>, 
    mousebtn_evr: EventReader<MouseButtonInput>,
) {
    if any_button(keys, buttons, key_evr, mousebtn_evr) {
        game_time.start_time = time.elapsed_seconds_f64();
        state.set(GameState::InProgress);
    }
}

fn in_progress_update(
    keys: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    time: Res<Time>,
    game_time: Res<GameTime>,
    mut state: ResMut<NextState<GameState>>,
    key_evr: EventReader<KeyboardInput>, 
    mousebtn_evr: EventReader<MouseButtonInput>,
    scroll_evr: EventReader<MouseWheel>,
    cursor_evr: EventReader<CursorMoved>,
    mut query: Query<&mut Text, With<NothingText>>
) {
    if let Ok(mut text) = query.get_single_mut() {
        let time_elapsed = (time.elapsed_seconds_f64() - game_time.start_time).floor() as i32;
        text.sections[3].value = format_time(time_elapsed);
    }

    if any_input(keys, buttons, key_evr, mousebtn_evr, scroll_evr, cursor_evr) {
        state.set(GameState::GameOver);
    }
}

fn game_over_update(
    keys: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    mut state: ResMut<NextState<GameState>>,
    key_evr: EventReader<KeyboardInput>, 
    mousebtn_evr: EventReader<MouseButtonInput>,
) {
    if any_button(keys, buttons, key_evr, mousebtn_evr) {
        state.set(GameState::NotStarted);
    }
}

fn game_over_text(
    time: Res<Time>,
    game_time: Res<GameTime>,
    mut query: Query<&mut Text, With<NothingText>>
) {
    if let Ok(mut text) = query.get_single_mut() {
        let time_elapsed = (time.elapsed_seconds_f64() - game_time.start_time).floor() as i32;
        text.sections[0].value = format!("You did something, you lost\nYou did nothing for {}", format_time(time_elapsed));
    }
}

fn any_button(
    keys: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    mut key_evr: EventReader<KeyboardInput>, 
    mut mousebtn_evr: EventReader<MouseButtonInput>,
) -> bool {
    for ev in key_evr.read() {
        if let Some(code) = ev.key_code {
            if keys.just_pressed(code) {
                return true;
            }
        }
    }

    for ev in mousebtn_evr.read() {
        if buttons.just_pressed(ev.button) {
            return true;
        }
    }

    return false;
}

fn any_input(
    keys: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    key_evr: EventReader<KeyboardInput>, 
    mousebtn_evr: EventReader<MouseButtonInput>,
    scroll_evr: EventReader<MouseWheel>,
    cursor_evr: EventReader<CursorMoved>,
) -> bool {
    if any_button(keys, buttons, key_evr, mousebtn_evr) {
        return true;
    }

    if scroll_evr.len() > 0 || cursor_evr.len() > 0 {
        return true;
    }

    return false;
}

fn format_time(duration: i32) -> String {
    let days = duration / 86400;
    let hours = duration / 3600;
    let minutes = duration / 60;
    if days > 0 {
        if days == 1 {
            return format!("{days} day");
        }

        return format!("{days} days");
    } else if hours > 0 {
        if hours == 1 {
            return format!("{hours} hour");
        }

        return format!("{hours} hours");
    } else if minutes > 0 {
        if minutes == 1 {
            return format!("{minutes} minute");
        }

        return format!("{minutes} minutes");
    }

    if duration == 1 {
        return format!("{duration} second");
    }

    return format!("{duration} seconds");
}
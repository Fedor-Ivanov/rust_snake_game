use bevy::{
    app::AppExit,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use rand::prelude::*;
use std::vec;

const PIXEL_SIZE: f32 = 30.0;
const WINDOW_WIDTH: f32 = 21.0 * PIXEL_SIZE;
const WINDOW_HEIGHT: f32 = 21.0 * PIXEL_SIZE;
const DEFAULT_SNAKE_BODY_SIZE: u32 = 2;
const DEFAULT_SNAKE_DIRECTION: Direction = Direction::Right;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    Start,
    Playing,
    GameOver,
}
#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
enum MenuButtonAction {
    Play,
    Quit,
}

#[derive(Component, Deref, DerefMut)]
struct MoveTimer(Timer);

#[derive(Component)]
struct SnakeHead {
    direction: Direction,
}
#[derive(Component)]
struct SnakeBody;

#[derive(Component)]
struct Food;

const BLACK_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const BLACK_TRANSPARENT_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
const SNAKE_HEAD_COLOR: Color = Color::rgb(0.125, 0.35, 0.85);
const SNAKE_BODY_COLOR: Color = Color::rgba(0.78, 0.48, 0.5, 0.8);

fn main() {
    let background_color = ClearColor(Color::hex("#9bba59").unwrap());

    App::new()
        .insert_resource(background_color)
        .init_state::<GameState>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snake".to_string(),
                resolution: (WINDOW_WIDTH + 200., WINDOW_HEIGHT + 50.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (bevy::window::close_on_esc,))
        .add_systems(OnEnter(GameState::Start), starting_menu)
        .add_systems(OnExit(GameState::Start), teardown)
        .add_systems(OnEnter(GameState::Playing), snake_setup)
        .add_systems(
            Update,
            (move_forward, change_direction, check_collision).run_if(in_state(GameState::Playing)),
        )
        .add_systems(OnExit(GameState::Playing), teardown)
        .add_systems(
            Update,
            button_system
                .run_if((in_state(GameState::Start)).or_else(in_state(GameState::GameOver))),
        )
        .add_systems(OnEnter(GameState::GameOver), game_over_menu)
        .add_systems(OnExit(GameState::GameOver), teardown)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn starting_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },

                        border_color: BorderColor(Color::BLACK),
                        background_color: BLACK_TRANSPARENT_COLOR.into(),
                        ..default()
                    },
                    MenuButtonAction::Play,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font: asset_server.load("fonts/NovaSlim-Regular.ttf"),
                            font_size: 40.0,
                            color: BLACK_COLOR,
                        },
                    ));
                });
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),

                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: BLACK_TRANSPARENT_COLOR.into(),
                        ..default()
                    },
                    MenuButtonAction::Quit,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Exit",
                        TextStyle {
                            font: asset_server.load("fonts/NovaSlim-Regular.ttf"),
                            font_size: 40.0,
                            color: BLACK_COLOR,
                        },
                    ));
                });
        });
}

fn game_over_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: BLACK_TRANSPARENT_COLOR.into(),
                        ..default()
                    },
                    MenuButtonAction::Play,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play again",
                        TextStyle {
                            font: asset_server.load("fonts/NovaSlim-Regular.ttf"),
                            font_size: 30.0,
                            color: BLACK_COLOR,
                        },
                    ));
                });
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: BLACK_TRANSPARENT_COLOR.into(),
                        ..default()
                    },
                    MenuButtonAction::Quit,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Exit",
                        TextStyle {
                            font: asset_server.load("fonts/NovaSlim-Regular.ttf"),
                            font_size: 40.0,
                            color: BLACK_COLOR,
                        },
                    ));
                });
        });
}

fn snake_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let snake_cell: Mesh2dHandle = meshes.add(Rectangle::new(PIXEL_SIZE, PIXEL_SIZE)).into();

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(Rectangle::new(WINDOW_WIDTH, WINDOW_HEIGHT))
            .into(),
        material: materials.add(Color::rgb(0.67, 0.55, 0.6)),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..default()
    });

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: snake_cell.clone(),
            material: materials.add(SNAKE_HEAD_COLOR),
            transform: Transform::from_translation(Vec3::new(0., 0., 2.)),
            ..default()
        },
        SnakeHead {
            direction: DEFAULT_SNAKE_DIRECTION,
        },
        MoveTimer(Timer::from_seconds(1., TimerMode::Repeating)),
    ));

    let mut defaults_positions_of_x: Vec<f32> = vec![0.];

    for i in 1..DEFAULT_SNAKE_BODY_SIZE + 1 {
        defaults_positions_of_x.push(PIXEL_SIZE * i as f32);

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: snake_cell.clone(),
                material: materials.add(SNAKE_BODY_COLOR),
                transform: Transform::from_translation(Vec3::new(
                    0. - (PIXEL_SIZE * i as f32),
                    0.,
                    2.,
                )),
                ..default()
            },
            SnakeBody,
        ));
    }

    let mut food_position = get_random_position();

    loop {
        if defaults_positions_of_x.contains(&food_position[0])
            && food_position != Vec3::new(0.0, 0.0, 2.)
            && food_position != Vec3::new(0.0, -PIXEL_SIZE, 2.)
        {
            break;
        }

        food_position = get_random_position();
    }

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new((PIXEL_SIZE) / 2.)).into(),
            material: materials.add(Color::hex("#808000").unwrap()),
            transform: Transform::from_translation(food_position),
            ..default()
        },
        Food,
    ));
}

fn move_forward(
    time: Res<Time>,
    mut snake_head_query: Query<(&mut Transform, &mut SnakeHead, &mut MoveTimer)>,
    mut snake_body_query: Query<&mut Transform, (With<SnakeBody>, Without<SnakeHead>)>,
) {
    for (mut transform, snake_head, mut timer) in snake_head_query.iter_mut() {
        let shake_head_translation = transform.translation.clone();

        if timer.tick(time.delta()).just_finished() {
            match snake_head.direction {
                Direction::Up => {
                    transform.translation.y += PIXEL_SIZE;
                }
                Direction::Down => {
                    transform.translation.y -= PIXEL_SIZE;
                }
                Direction::Left => {
                    transform.translation.x -= PIXEL_SIZE;
                }
                Direction::Right => {
                    transform.translation.x += PIXEL_SIZE;
                }
            }

            let mut last_translation = shake_head_translation;

            snake_body_query.iter_mut().for_each(|mut body_transform| {
                let temp_translation = body_transform.translation.clone();
                body_transform.translation = last_translation;
                last_translation = temp_translation;
            });
        }
    }
}

fn change_direction(keyboard_input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut SnakeHead>) {
    for mut snake_head in query.iter_mut() {
        if (keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp))
            && snake_head.direction != Direction::Down
        {
            snake_head.direction = Direction::Up;
        } else if (keyboard_input.pressed(KeyCode::KeyA)
            || keyboard_input.pressed(KeyCode::ArrowLeft))
            && snake_head.direction != Direction::Right
        {
            snake_head.direction = Direction::Left;
        } else if (keyboard_input.pressed(KeyCode::KeyS)
            || keyboard_input.pressed(KeyCode::ArrowDown))
            && snake_head.direction != Direction::Up
        {
            snake_head.direction = Direction::Down;
        } else if (keyboard_input.pressed(KeyCode::KeyD)
            || keyboard_input.pressed(KeyCode::ArrowRight))
            && snake_head.direction != Direction::Left
        {
            snake_head.direction = Direction::Right;
        }
    }
}

fn check_collision(
    mut snake_head_query: Query<&mut Transform, (With<SnakeHead>, Without<SnakeBody>)>,
    mut snake_body_query: Query<&mut Transform, (With<SnakeBody>, Without<SnakeHead>)>,
    mut food_query: Query<&mut Transform, (With<Food>, Without<SnakeHead>, Without<SnakeBody>)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    for snake_head_transform in snake_head_query.iter_mut() {
        let mut crashed: bool = false;
        for body_transform in &snake_body_query {
            if snake_head_transform.translation == body_transform.translation {
                crashed = true;
            }
        }

        let mut side_crashed: bool = false;

        if snake_head_transform.translation.x > WINDOW_WIDTH / 2.
            || snake_head_transform.translation.x < -WINDOW_WIDTH / 2.
            || snake_head_transform.translation.y > WINDOW_HEIGHT / 2.
            || snake_head_transform.translation.y < -WINDOW_HEIGHT / 2.
        {
            side_crashed = true;
        }

        if crashed || side_crashed {
            next_state.set(GameState::GameOver)
        }

        let food_transform = food_query.single_mut();

        if food_transform.translation == snake_head_transform.translation {
            let mut body_positions: Vec<Vec3> = vec![snake_head_transform.translation];

            snake_body_query.iter_mut().for_each(|body_transform| {
                let temp_translation = body_transform.translation.clone();
                body_positions.push(temp_translation);
            });

            let mut food_position = get_random_position();

            loop {
                if !body_positions.contains(&food_position) {
                    break;
                }
                food_position = get_random_position();
            }

            food_query.single_mut().translation = food_position;

            let snake_cell: Mesh2dHandle =
                meshes.add(Rectangle::new(PIXEL_SIZE, PIXEL_SIZE)).into();

            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: snake_cell,
                    material: materials.add(SNAKE_BODY_COLOR),
                    transform: Transform::from_translation(food_position),
                    ..default()
                },
                SnakeBody,
            ));
        }
    }
}

fn button_system(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    app_exit_events.send(AppExit);
                }
                MenuButtonAction::Play => {
                    game_state.set(GameState::Playing);
                }
            }
        }
    }
}

fn teardown(mut commands: Commands, entities: Query<Entity, (Without<Camera>, Without<Window>)>) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}

fn get_random_position() -> Vec3 {
    println!("get_random_position");
    let mut rng = rand::thread_rng();
    let x: f32 = (rng.gen_range(0..(WINDOW_WIDTH / PIXEL_SIZE) as i32)
        - (((WINDOW_WIDTH / PIXEL_SIZE) as i32) / 2)) as f32
        * PIXEL_SIZE;
    let y: f32 = (rng.gen_range(0..(WINDOW_HEIGHT / PIXEL_SIZE) as i32)
        - (((WINDOW_HEIGHT / PIXEL_SIZE) as i32) / 2)) as f32
        * PIXEL_SIZE;
    Vec3::new(x, y, 2.)
}

use bevy::{
    prelude::*,
    app::AppExit,
};
use crate::{style, utils, player, enemy, score};
use utils::bevy::{state::Simulation, projectile::Projectile};

#[derive(Component)]
pub struct PausedMenu;

#[derive(Component)]
pub enum Button {
    Play,
    Exit,
}

impl Button {
    fn color(&self) -> Color {
        match self {
            Button::Play => style::MENU_PLAY_COLOR,
            Button::Exit => style::MENU_EXIT_COLOR,
        }
    }
}

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn)
            .add_systems(Update, (toggle_visibility, button_change));
    }
}

const ALPHA: f32 = 1.0 - 1.0 / 16.0;

fn button_change(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
            &Button,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut commands: Commands,
    mut exit: EventWriter<AppExit>,
    player_query: Query<With<player::Player>>,
    enemy_query: Query<Entity, With<enemy::Enemy>>,
    projectile_query: Query<Entity, With<Projectile>>,
    mut enemies_count: ResMut<enemy::EnemiesCount>,
    asset_server: Res<AssetServer>,
    camera_query: Query<&Transform, With<Camera>>,
    mut score: ResMut<score::Score>,
) {
    let mut play = || {
        let state = Simulation::Running;
        commands.insert_resource(NextState(Some(state)));
        if let Err(_) = player_query.get_single() {
            player::spawn(
                &mut commands,
                &asset_server,
                &camera_query,
            );
            for entity in enemy_query.iter() {
                commands.entity(entity).despawn();
            }
            for entity in projectile_query.iter() {
                commands.entity(entity).despawn();
            }
            enemies_count.count = 0;
            enemies_count.wave = 0;
            score.value = 0;
        }
    };
    for (interaction, mut color, mut border_color, children, button)
    in &mut interaction_query {
        let mut _text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                match button {
                    Button::Play => {
                        play();
                    }
                    Button::Exit => {
                        exit.send(AppExit)
                    }
                }
            }
            Interaction::Hovered => {
                color.0 = button.color().with_a(0.75);
            }
            Interaction::None => {
                color.0 = button.color().with_a(0.5);
                border_color.0 = utils::bevy::grey(0.0, 0.9);
            }
        }
    }
}

fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let minimap_width = style::MINIMAP_WIDTH * 100.0;
    let minimap_height = style::MINIMAP_HEIGHT * 100.0;
    let pad = 0.125 * 600.0 * 0.25;
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(50.0),
                height: Val::Percent(100.0 - minimap_height),
                justify_content: JustifyContent::Default,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(pad),
                top: Val::Percent(minimap_height),
                left: Val::Percent((100.0 - minimap_width) * 0.5),
                padding: UiRect::all(Val::Px(pad)),
                border: UiRect::horizontal(Val::Px(1.0)),
                ..default()
            },
            border_color: utils::bevy::grey(1.0, 0.5).into(),
            ..default()
        },
        PausedMenu
    )).with_children(|parent| {
        let mut show_binding = |tiles: Vec<u32>, msg| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    background_color:
                        utils::bevy::grey(1.0 / 16.0, ALPHA).into(),
                    ..default()
                })
                .with_children(|parent| {
                    let section = TextSection::new(msg, TextStyle {
                        color: Color::WHITE,
                        font_size: 24.0,
                        font: asset_server.load(style::FONT),
                        ..default()
                    });
                    parent
                        .spawn(TextBundle {
                            text: Text::from_sections([section]),
                            style: Style {
                                align_self: AlignSelf::Center,
                                left: Val::Px(16.0),
                                ..default()
                            },
                            ..default()
                        });
                    
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Px(16.0 * 8.0),
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            background_color:
                                utils::bevy::grey(1.0, 0.5).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            for tile in tiles.iter() {
                                let path = format!("ui/tile_{:04}.png", tile);
                                parent.spawn((
                                    NodeBundle {
                                        style: Style {
                                            width: Val::Px(16.0 * 3.0),
                                            height: Val::Px(16.0 * 3.0),
                                            ..default()
                                        },
                                        background_color: Color::WHITE.into(),
                                        ..default()
                                    },
                                    UiImage::new(asset_server.load(path))
                                ));
                            }
                        });
                });
        };
        let esc = vec![17];
        let w = vec![86];
        let a = vec![120];
        let s = vec![121];
        let d = vec![122];
        let space = vec![235, 236, 237];
        show_binding(esc, "pause");
        show_binding(a, "move left");
        show_binding(d, "move right");
        show_binding(w, "move up");
        show_binding(s, "move down");
        show_binding(space, "shoot laser");
        let mut button = |msg, button: Button| {
            parent
            .spawn((
                ButtonBundle {
                    style: Style {
                        border: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    background_color: button.color().into(),
                    ..default()
                },
                button,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    msg,
                    TextStyle {
                        font: asset_server.load(style::FONT),
                        font_size: 40.0,
                        color: utils::bevy::grey(1.0, 1.0),
                    },
                ));
            });
        };
        button("play", Button::Play);
        button("exit", Button::Exit);
    });
}

fn toggle_visibility(
    state: Res<State<Simulation>>,
    mut query: Query<&mut Style, With<PausedMenu>>,
) {
    let mut style = query.get_single_mut().unwrap();
    match state.get() {
        Simulation::Paused => style.display = Display::Flex,
        Simulation::Running => style.display = Display::None,
    }
}
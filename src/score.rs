use bevy::{prelude::*, window::PrimaryWindow};

use crate::style;
use std::f32::consts::TAU;

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

#[derive(Component)]
struct ScoreText;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score { value: 0 })
            .add_systems(Startup, spawn_score_text)
            .add_systems(Update, update_score_text);
    }
}

const TEXT_SPACE: f32 = 0.5 * (1.0 - style::MINIMAP_SIZE.x);

fn spawn_score_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    let font_size = style::SCORE_FONT_SIZE;
    commands.spawn((
        TextBundle::from_sections([TextSection::from_style(TextStyle {
            font: asset_server.load(style::FONT),
            font_size,
            color: style::SCORE_COLOR,
        })])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px((window.height() * style::MINIMAP_SIZE.y - font_size) * 0.5),
            right: Val::Px(window.width() * (1.0 - TEXT_SPACE) + 15.0),
            ..default()
        }),
        ScoreText,
    ));
}

const HZ: f32 = 4.0;
const DIM: f32 = 1.0 / 3.0;

fn update_score_text(
    mut query: Query<(&mut Text, &mut Style), With<ScoreText>>,
    score: Res<Score>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let (mut text, mut style) = query.single_mut();
    let window = window_query.single();
    let font_size = style::SCORE_FONT_SIZE;
    style.top = Val::Px((window.height() * style::MINIMAP_SIZE.y - font_size) * 0.5);
    style.right = Val::Px(window.width() * (1.0 - TEXT_SPACE) + 15.0);
    text.sections[0].value = format!("{:06}", score.value);
    text.sections[0].style.color = Color::Hsla {
        hue: (time.elapsed_seconds() / 6.0 * HZ).fract() * 360.0,
        saturation: 1.0,
        lightness: 0.75,
        alpha: (time.elapsed_seconds() * TAU * HZ).sin() * DIM + (1.0 - DIM),
    };
}

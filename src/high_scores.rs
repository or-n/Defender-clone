use crate::{
    style,
    utils::{self, bevy::window},
};
use bevy::prelude::*;
use std::f32::consts::TAU;
use utils::bevy::state::Simulation;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (load_high_scores, spawn_high_score_text).chain())
            .add_systems(Update, (update_high_score_text, toggle_visibility));
    }
}

fn load_high_scores(mut commands: Commands) {
    commands.insert_resource(HighScores::load());
}

#[derive(Resource)]
pub struct HighScores {
    scores: Vec<u32>,
}

pub enum Error {
    Score(std::num::ParseIntError),
}

const PATH: &str = "assets/high_scores.txt";

impl HighScores {
    pub fn load() -> Self {
        //let input = std::fs::read_to_string(PATH).unwrap_or("".to_string());
        let input = "".to_string();
        Self::read(input).unwrap_or(Self { scores: vec![] })
    }

    pub fn save(&mut self, new: u32) {
        self.scores.push(new);
        self.top10_ordered();
        //let _ = std::fs::write(PATH, self.write());
    }

    fn read(input: String) -> Result<Self, Error> {
        let mut read = Reader::new(input.as_str());
        let mut scores = vec![];
        while !read.ended() {
            let score = read
                .till(|c| c != '\n')
                .parse::<u32>()
                .map_err(Error::Score)?;
            scores.push(score);
        }
        let mut high_scores = Self { scores };
        high_scores.top10_ordered();
        Ok(high_scores)
    }

    fn write(&self) -> String {
        let mut output = String::new();
        for score in &self.scores {
            output.extend(format!("{:06}", score).chars());
            output.push('\n');
        }
        output
    }

    fn top10_ordered(&mut self) {
        self.scores.sort_by(|a, b| b.cmp(a));
        while self.scores.len() > 10 {
            self.scores.pop();
        }
    }
}

pub struct Reader<'a> {
    it: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Reader<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            it: input.chars().peekable(),
        }
    }

    pub fn till(&mut self, satisfy: impl Fn(char) -> bool) -> String {
        let mut output = String::new();
        while let Some(c) = self.it.next() {
            if satisfy(c) {
                output.push(c);
            } else {
                break;
            }
        }
        output
    }

    pub fn ended(&mut self) -> bool {
        self.it.peek().is_none()
    }
}

#[derive(Component)]
struct HighScoreText;

const TEXT_SPACE: f32 = 0.5 * (1.0 - style::MINIMAP_SIZE.x);

fn spawn_high_score_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_size = style::SCORE_FONT_SIZE;
    let section = TextSection::from_style(TextStyle {
        font: asset_server.load(style::FONT),
        font_size,
        color: style::SCORE_COLOR,
    });
    commands.spawn((
        TextBundle::from_sections([section.clone(), section]).with_style(Style {
            position_type: PositionType::Absolute,
            align_self: AlignSelf::FlexEnd,
            ..default()
        }),
        HighScoreText,
    ));
}

const HZ: f32 = 1.0;
const DIM: f32 = 1.0 / 3.0;

fn update_high_score_text(
    mut query: Query<(&mut Text, &mut Style), With<HighScoreText>>,
    high_scores: Res<HighScores>,
    time: Res<Time>,
    window_size: Res<window::Size>,
) {
    let (mut text, mut style) = query.single_mut();
    let font_size = style::SCORE_FONT_SIZE;
    style.top = Val::Px(window_size.0.y * style::MINIMAP_SIZE.y + font_size * 0.5);
    style.right = Val::Px(window_size.0.x * (1.0 - TEXT_SPACE) + 15.0);
    text.sections[0].value = format!("TOP {}\n", high_scores.scores.len());
    text.sections[1].value = high_scores.write();
    text.sections[1].style.color = Color::Hsla {
        hue: (time.elapsed_seconds() / 6.0 * HZ).fract() * 360.0,
        saturation: 1.0,
        lightness: 0.75,
        alpha: (time.elapsed_seconds() * TAU * HZ).sin() * DIM + (1.0 - DIM),
    };
}

fn toggle_visibility(
    state: Res<State<Simulation>>,
    mut query: Query<&mut Style, With<HighScoreText>>,
) {
    let mut style = query.get_single_mut().unwrap();
    match state.get() {
        Simulation::Paused => style.display = Display::Flex,
        Simulation::Running => style.display = Display::None,
    }
}

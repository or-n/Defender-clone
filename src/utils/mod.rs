pub mod bevy;
pub mod range;

#[derive(PartialEq, Clone, Copy)]
pub enum Side {
    Left,
    Right,
}

impl Side {
    pub fn sign(&self) -> f32 {
        match self {
            Side::Left => -1.0,
            Side::Right => 1.0,
        }
    }
}

pub fn my_fract(x: f32) -> f32 {
    let n = x.fract();
    if n >= 0.0 {
        n
    } else {
        1.0 + n
    }
}

pub fn variant(path_extension: (&str, &str), index: String) -> String {
    format!("{}{}{}", path_extension.0, index, path_extension.1)
}

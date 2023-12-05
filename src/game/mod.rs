use std::f32::consts::PI;

use bevy::{
    app::Plugin,
    math::{IVec2, Quat, Vec2Swizzles},
    reflect::Reflect,
};

pub mod machines;
pub mod material;

pub struct GameUtilsPlugin;

impl Plugin for GameUtilsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<Direction2D>();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum Direction2D {
    Forward,
    Backward,
    Left,
    Right,
}

impl Into<Quat> for Direction2D {
    fn into(self) -> Quat {
        Quat::from_rotation_y(match self {
            Direction2D::Forward => 0.0,
            Direction2D::Backward => PI,
            Direction2D::Left => PI / 2.0,
            Direction2D::Right => 3.0 * PI / 2.0,
        })
    }
}

impl Direction2D {
    pub fn rotate(self) -> Self {
        match self {
            Direction2D::Forward => Direction2D::Right,
            Direction2D::Backward => Direction2D::Left,
            Direction2D::Right => Direction2D::Backward,
            Direction2D::Left => Direction2D::Forward,
        }
    }

    pub fn rotate_size(self, size: IVec2) -> IVec2 {
        match self {
            Direction2D::Backward | Direction2D::Forward => size,
            Direction2D::Left | Direction2D::Right => size.yx(),
        }
    }
}

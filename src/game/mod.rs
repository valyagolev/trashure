use std::{f32::consts::PI, ops::Mul};

use bevy::{
    app::Plugin,
    math::{IVec2, Quat, Vec2Swizzles},
    prelude::Component,
    reflect::Reflect,
};

pub mod machines;
pub mod material;

pub struct GameUtilsPlugin;

impl Plugin for GameUtilsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<Direction2D>()
            .add_plugins(machines::MachinesPlugin);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Component)]
pub enum Direction2D {
    Forward = 0,
    Right = 1,
    Backward = 2,
    Left = 3,
}

impl From<&Direction2D> for Quat {
    fn from(val: &Direction2D) -> Self {
        Quat::from_rotation_y(match val {
            Direction2D::Forward => 0.0,
            Direction2D::Backward => PI,
            Direction2D::Left => PI / 2.0,
            Direction2D::Right => 3.0 * PI / 2.0,
        })
    }
}

impl From<usize> for Direction2D {
    fn from(val: usize) -> Self {
        match val % 4 {
            0 => Direction2D::Forward,
            1 => Direction2D::Right,
            2 => Direction2D::Backward,
            3 => Direction2D::Left,
            _ => unreachable!(),
        }
    }
}

impl Mul<Direction2D> for Direction2D {
    type Output = Direction2D;

    fn mul(self, rhs: Direction2D) -> Self::Output {
        ((self as usize) + (rhs as usize)).into()
    }
}

impl Direction2D {
    pub fn rotate(self) -> Self {
        ((self as usize) + 1).into()
    }

    pub fn rotate_size(self, size: IVec2) -> IVec2 {
        match self {
            Direction2D::Backward | Direction2D::Forward => size,
            Direction2D::Left | Direction2D::Right => size.yx(),
        }
    }

    pub fn within_cone(self, pos: IVec2) -> bool {
        match self {
            Direction2D::Forward => pos.y < 0 && pos.x.abs() < -pos.y,
            Direction2D::Backward => pos.y > 0 && pos.x.abs() < pos.y,
            Direction2D::Left => pos.x < 0 && pos.y.abs() < -pos.x,
            Direction2D::Right => pos.x > 0 && pos.y.abs() < pos.x,
        }
    }
}

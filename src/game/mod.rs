use std::{
    f32::consts::PI,
    ops::{Mul, Neg},
};

use bevy::{
    app::Plugin,
    math::{IVec2, Quat, Vec2Swizzles},
    prelude::Component,
    reflect::Reflect,
};

pub mod machines;
pub mod material;
pub mod voxelmailbox;

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

impl Neg for &Direction2D {
    type Output = Direction2D;

    fn neg(self) -> Self::Output {
        ((*self as usize) + 2).into()
    }
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

impl Into<IVec2> for Direction2D {
    fn into(self) -> IVec2 {
        match self {
            Direction2D::Forward => IVec2::new(0, -1),
            Direction2D::Backward => IVec2::new(0, 1),
            Direction2D::Left => IVec2::new(-1, 0),
            Direction2D::Right => IVec2::new(1, 0),
        }
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

    pub fn within_cone(self, pos: IVec2, min_dims: IVec2) -> bool {
        // dbg!(&self, pos, min_dims);
        match self {
            Direction2D::Forward => pos.y < 0 && pos.x.abs() <= (-pos.y).max(min_dims.x / 2),
            Direction2D::Backward => pos.y > 0 && pos.x.abs() <= pos.y.max(min_dims.x / 2),
            Direction2D::Left => pos.x <= 0 && pos.y.abs() <= (-pos.x).max(min_dims.y / 2),
            Direction2D::Right => pos.x > 0 && pos.y.abs() <= pos.x.max(min_dims.y / 2),
        }
    }

    pub fn random_in_cone(self, max_d: i32, min_dims: IVec2, rng: &mut impl rand::Rng) -> IVec2 {
        // too lazy for trigonometry lmao
        loop {
            let pos = IVec2::new(rng.gen_range(-max_d..=max_d), rng.gen_range(-max_d..=max_d));

            if self.within_cone(pos, min_dims) {
                return pos;
            }
        }
    }

    pub fn line_in_direction(self, center: IVec2, size: IVec2) -> impl Iterator<Item = IVec2> {
        let width = match self {
            Direction2D::Forward | Direction2D::Backward => size.x,
            Direction2D::Left | Direction2D::Right => size.y,
        };
        let start = match self {
            Direction2D::Forward => center + IVec2::new(-width / 2, -size.y / 2),
            Direction2D::Backward => center + IVec2::new(-width / 2, size.y / 2),
            Direction2D::Left => center + IVec2::new(-size.x / 2, -width / 2),
            Direction2D::Right => center + IVec2::new(size.x / 2, -width / 2),
        };

        (0..width).map(move |i| match self {
            Direction2D::Forward | Direction2D::Backward => start + IVec2::new(i, 0),
            Direction2D::Left | Direction2D::Right => start + IVec2::new(0, i),
        })
    }
}

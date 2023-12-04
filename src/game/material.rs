use bevy::prelude::*;
use rand::Rng;

#[derive(Debug, Reflect, Clone, Copy, PartialEq, Eq, Hash)]
// #[repr(u32)]
pub enum GameMaterial {
    Reddish = 1,
    Greenish = 2,
    Blueish = 3,
    Brownish = 4,
}

impl GameMaterial {
    pub fn random(rng: &mut impl Rng) -> Self {
        match rng.gen_range(0..25) {
            0 => Self::Reddish,
            1..=3 => Self::Greenish,
            4..=8 => Self::Blueish,
            _ => Self::Brownish,
        }
    }
}

#[cfg(test)]
mod test {
    use std::mem::size_of;

    use super::*;

    #[test]
    fn test_repr() {
        println!("{:?}", GameMaterial::Reddish);
        println!("{:?}", GameMaterial::Reddish as u32);
        println!("{:?}", GameMaterial::Reddish as usize);
        println!("{:?}", size_of::<GameMaterial>());
        println!("{:?}", size_of::<Option<GameMaterial>>());
    }
}

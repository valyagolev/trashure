use bevy::prelude::*;
use rand::Rng;

#[derive(Debug, Reflect, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameMaterial {
    Reddish = 0b1,
    Greenish = 0b10,
    Blueish = 0b100,
    Brownish = 0b1000,
    Golden = 0b10000,
}

impl Into<Color> for &GameMaterial {
    fn into(self) -> Color {
        match self {
            GameMaterial::Reddish => Color::rgb(0.8, 0.5, 0.4),
            GameMaterial::Greenish => Color::rgb(0.5, 0.8, 0.4),
            GameMaterial::Blueish => Color::rgb(0.4, 0.5, 0.8),
            GameMaterial::Brownish => Color::rgb(0.8, 0.7, 0.6),
            GameMaterial::Golden => Color::rgb(0.8, 0.7, 0.2),
        }
    }
}

impl GameMaterial {
    pub fn as_usize(self) -> usize {
        (self as u8).trailing_zeros() as usize
    }

    pub fn random(rng: &mut impl Rng) -> Self {
        match rng.gen_range(0..25) {
            0 => Self::Reddish,
            1..=3 => Self::Greenish,
            4..=8 => Self::Blueish,
            _ => Self::Brownish,
        }
    }

    #[inline]
    pub fn any_of_mask(of: &[GameMaterial]) -> u8 {
        of.iter().fold(0, |acc, &m| acc | m as u8)
    }

    #[inline]
    pub fn all() -> &'static [GameMaterial] {
        &[
            GameMaterial::Reddish,
            GameMaterial::Greenish,
            GameMaterial::Blueish,
            GameMaterial::Brownish,
            GameMaterial::Golden,
        ]
    }

    #[inline]
    pub fn mask_contains(&self, mask: u8) -> bool {
        (*self as u8) & mask != 0
    }
}

#[cfg(test)]
mod test {
    use std::mem::size_of;

    use super::*;

    #[test]
    fn test_repr() {
        println!("{:?}", GameMaterial::Greenish as u32);
        println!("{:?}", GameMaterial::Greenish as usize);
        println!("{:?}", GameMaterial::Greenish);
        println!("{:?}", size_of::<GameMaterial>());
        println!("{:?}", size_of::<Option<GameMaterial>>());

        println!(
            "{:?}",
            GameMaterial::any_of_mask(&[GameMaterial::Reddish, GameMaterial::Greenish])
        );
        println!(
            "{:?}",
            GameMaterial::any_of_mask(&[
                GameMaterial::Reddish,
                GameMaterial::Greenish,
                GameMaterial::Golden
            ])
        );

        assert!(
            GameMaterial::Reddish.mask_contains(GameMaterial::any_of_mask(&[
                GameMaterial::Reddish,
                GameMaterial::Greenish
            ]))
        );

        assert!(
            !GameMaterial::Golden.mask_contains(GameMaterial::any_of_mask(&[
                GameMaterial::Reddish,
                GameMaterial::Greenish
            ]))
        );

        assert!(
            GameMaterial::Golden.mask_contains(GameMaterial::any_of_mask(&[
                GameMaterial::Reddish,
                GameMaterial::Greenish,
                GameMaterial::Golden
            ]))
        );

        assert!(
            !GameMaterial::Golden.mask_contains(GameMaterial::any_of_mask(&[
                GameMaterial::Reddish,
                GameMaterial::Greenish,
                GameMaterial::Brownish
            ]))
        );

        assert!(!GameMaterial::Golden.mask_contains(GameMaterial::any_of_mask(&[])));
    }
}

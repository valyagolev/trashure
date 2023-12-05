use std::f32::consts::E;

use bevy::prelude::*;

use crate::graphics::recolor::Tinted;

#[derive(Debug, Component, PartialEq, Eq, Hash, Clone, Copy, Reflect)]
pub enum MachineRecolor {
    Ghost,
    ForbiddenGhost,
    Selected,
}

impl Into<Color> for MachineRecolor {
    fn into(self) -> Color {
        match self {
            MachineRecolor::Ghost => Color::rgba_u8(180, 194, 201, 192),
            MachineRecolor::ForbiddenGhost => Color::rgba_u8(177, 61, 61, 192),
            MachineRecolor::Selected => Color::rgba_u8(0, 162, 220, 255),
        }
    }
}

impl Into<Tinted> for MachineRecolor {
    fn into(self) -> Tinted {
        match self {
            MachineRecolor::Ghost | MachineRecolor::ForbiddenGhost => {
                let mut t = Tinted::new(self.into());
                t.alpha_mode = Some(AlphaMode::Blend);
                t
            }
            MachineRecolor::Selected => Tinted::new_emissive(self.into(), self.emissive().unwrap()),
        }
    }
}

impl MachineRecolor {
    pub fn emissive(self) -> Option<Color> {
        if self == MachineRecolor::Selected {
            // Some(Color::rgba_u8(80, 83, 101, 73))

            Some(Color::rgba_u8(13, 16, 34, 73))
        } else {
            None
        }
    }
}

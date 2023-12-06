

use bevy::prelude::*;

use crate::graphics::recolor::Tinted;

#[derive(Debug, Component, PartialEq, Eq, Hash, Clone, Copy, Reflect)]
pub enum MachineRecolor {
    Ghost,
    ForbiddenGhost,
    Selected,
}

impl From<MachineRecolor> for Color {
    fn from(val: MachineRecolor) -> Self {
        match val {
            MachineRecolor::Ghost => Color::rgba_u8(180, 194, 201, 192),
            MachineRecolor::ForbiddenGhost => Color::rgba_u8(177, 61, 61, 192),
            MachineRecolor::Selected => Color::rgba_u8(0, 162, 220, 255),
        }
    }
}

impl From<MachineRecolor> for Tinted {
    fn from(val: MachineRecolor) -> Self {
        match val {
            MachineRecolor::Ghost | MachineRecolor::ForbiddenGhost => {
                let mut t = Tinted::new(val.into());
                t.alpha_mode = Some(AlphaMode::Blend);
                t
            }
            MachineRecolor::Selected => Tinted::new_emissive(val.into(), val.emissive().unwrap()),
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

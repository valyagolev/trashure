use std::borrow::Cow;

use bevy::prelude::*;

use crate::game::{
    machines::{GameMachineSettings, GameMachineSettingsDiscriminants},
    Direction2D,
};

use super::selectable::CurrentlySelected;

// use self::recolor::RecoloredScenes;

pub mod building;
mod colors;
pub mod radar;

pub struct MachinesPlugin;

impl Plugin for MachinesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((building::MachinesBuildingPlugin, radar::RadarPlugin))
            .add_systems(Startup, load_machines)
            // .add_systems(Update, debug_keyboard)
            .add_systems(
                Update,
                (
                    update_machines,
                    //  update_boxes,
                    update_colors,
                    rotate_selected_machine,
                ),
            )
            .register_type::<MyMachine>()
            .register_type::<MachineType>();
    }
}

// #[derive(Debug, Reflect)]
// pub struct MachineState {
//     Ghost,
//     Built,
// }

#[derive(Debug, Component, Reflect)]
pub struct BuiltMachine(pub GameMachineSettings);

#[derive(Debug, Component, Reflect)]
pub struct MachineType {
    pub gmt: GameMachineSettingsDiscriminants,
    pub name: Cow<'static, str>,
    scene: Handle<Scene>,
    pub dims: IVec2,
}

#[derive(Debug, Component, Reflect)]
pub struct MyMachine {
    pub gmt: GameMachineSettingsDiscriminants,
    pub tp: Entity,
    pub pos: IVec2,
    // pub direction: Direction2D,
    pub dims: IVec2,
}

impl MyMachine {
    pub fn intersects(&self, self_dir: Direction2D, other: &Self, other_dir: Direction2D) -> bool {
        let (x1, y1) = (self.pos.x, self.pos.y);
        let (x2, y2) = (other.pos.x, other.pos.y);

        let (w1, h1) = self_dir.rotate_size(self.dims).into();
        let (w2, h2) = other_dir.rotate_size(other.dims).into();

        // we don't care about recentering them...

        x1 < x2 + w2 && x1 + w1 > x2 && y1 < y2 + h2 && y1 + h1 > y2
    }
}

#[derive(Debug, Resource, Reflect)]
pub struct MachineResources {
    selection_cube: Handle<Mesh>,
    debug_reddish: Handle<StandardMaterial>,
    radar: Handle<Scene>,
}

#[derive(Debug, Component)]
pub struct DebugCube;

pub fn load_machines(
    mut commands: Commands,
    ass: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let selection_cube = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let debug_reddish = materials.add(Color::rgba(0.8, 0.5, 0.4, 0.2).into());

    commands.insert_resource(MachineResources {
        selection_cube: selection_cube.clone(),
        debug_reddish: debug_reddish.clone(),
        radar: ass.load("objects/radar.glb#Scene0"),
    });

    commands.spawn(MachineType {
        gmt: GameMachineSettingsDiscriminants::Recycler,
        name: "Recycler".into(),
        scene: ass.load("objects/recycler.glb#Scene0"),
        // scenes: RecoloredScenes::new(ass, "objects/recycler.glb#Scene0"),
        // dims: IVec2 { x: 7, y: 12 },

        // always square for now
        dims: IVec2 { x: 12, y: 12 },
    });

    commands.spawn((
        DebugCube,
        PbrBundle {
            mesh: selection_cube,
            material: debug_reddish,
            // transform: Transform::from_scale(Vec3::new(tp.dims.x as f32, 32.0, tp.dims.y as f32)),
            ..default()
        },
    ));
}

fn update_machines(
    mut commands: Commands,
    // ass: Res<AssetServer>,
    q_machinetypes: Query<&MachineType>,
    mut q_machines: Query<(Entity, &MyMachine, &Direction2D, Option<&mut Transform>)>,
    _mres: Res<MachineResources>,
) {
    for (e, machine, dir, spawn) in q_machines.iter_mut() {
        let trans = Transform::from_translation(machine.pos.extend(0).xzy().as_vec3())
            .with_rotation(dir.into());

        match spawn {
            None => {
                let tp = q_machinetypes.get(machine.tp).unwrap();

                commands.entity(e).insert(SceneBundle {
                    scene: tp.scene.clone(),
                    transform: trans,
                    ..default()
                });

                // commands.entity(e).push_children(&[bx]);
            }
            Some(mut ts) => {
                *ts = trans;
            }
        }
    }
}

// fn update_boxes(
//     // mut commands: Commands,
//     // ass: Res<AssetServer>,
//     q_machinetypes: Query<&MachineType>,
//     mut q_machines: Query<(Entity, &MyMachine, &Children)>,
//     mut q_cubes: Query<(&DebugCube, &mut Transform)>,
//     // mres: Res<MachineResources>,
// ) {
//     for (e, machine, children) in q_machines.iter_mut() {
//         let tp = q_machinetypes.get(machine.tp).unwrap();

//         for ch in children {
//             let Ok((cube, mut trans)) = q_cubes.get_mut(*ch) else {
//                 continue;
//             };

//             *trans = Transform::from_scale(Vec3::new(tp.dims.x as f32, 32.0, tp.dims.y as f32));
//         }
//     }
// }

fn update_colors(
    // mut commands: Commands,
    // ass: Res<AssetServer>,
    q_machinetypes: Query<&MachineType>,
    mut q_machines: Query<(Entity, &MyMachine, &Children)>,
    mut q_cubes: Query<(&DebugCube, &mut Transform)>,
    // mres: Res<MachineResources>,
) {
    for (_e, machine, children) in q_machines.iter_mut() {
        let tp = q_machinetypes.get(machine.tp).unwrap();

        for ch in children {
            let Ok((_cube, mut trans)) = q_cubes.get_mut(*ch) else {
                continue;
            };

            *trans = Transform::from_scale(Vec3::new(tp.dims.x as f32, 32.0, tp.dims.y as f32));
        }
    }
}

fn rotate_selected_machine(
    selected: Res<CurrentlySelected>,
    mut q_machines: Query<&mut Direction2D, With<BuiltMachine>>,
    keyb: Res<Input<KeyCode>>,
) {
    if !keyb.just_released(KeyCode::R) {
        return;
    }

    let Some(mid) = selected.0 else {
        return;
    };

    let Ok(mut m) = q_machines.get_mut(mid) else {
        return;
    };

    *m = m.rotate();

    dbg!(&m);
}

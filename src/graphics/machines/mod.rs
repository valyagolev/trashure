use std::borrow::Cow;

use bevy::{prelude::*, scene::SceneInstance};

use crate::game::Direction2D;

// use self::recolor::RecoloredScenes;

use super::{recolor::Tinted, selectable::Selectable};

pub struct MachinesPlugin;
mod colors;

impl Plugin for MachinesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_machines)
            // .add_systems(Update, debug_keyboard)
            .add_systems(Update, (update_machines, update_boxes))
            .register_type::<MyMachine>()
            .register_type::<MachineType>();
    }
}

#[derive(Debug, Component, Reflect)]
pub struct MachineType {
    name: Cow<'static, str>,
    scene: Handle<Scene>,
    dims: IVec2,
}

#[derive(Debug, Component, Reflect)]
pub struct MyMachine {
    tp: Entity,
    pos: IVec2,
    direction: Direction2D,
}

#[derive(Debug, Resource, Reflect)]
pub struct MachineResources {
    selection_cube: Handle<Mesh>,
    debug_reddish: Handle<StandardMaterial>,
}

#[derive(Debug, Component)]
pub struct DebugCube;

fn load_machines(
    mut commands: Commands,
    ass: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(MachineResources {
        selection_cube: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        debug_reddish: materials.add(Color::rgba(0.8, 0.5, 0.4, 0.2).into()),
    });

    let t = commands
        .spawn(MachineType {
            name: "Recycler".into(),
            scene: ass.load("objects/recycler.glb#Scene0"),
            // scenes: RecoloredScenes::new(ass, "objects/recycler.glb#Scene0"),
            dims: IVec2 { x: 7, y: 12 },
        })
        .id();

    commands.spawn((
        Tinted::new(Color::rgba(0.1, 0.6, 0.2, 0.1)),
        MyMachine {
            tp: t,
            pos: IVec2 { x: 5, y: 4 },
            direction: Direction2D::Backward,
        },
    ));

    commands.spawn((
        Tinted::new(Color::rgba(0.3, 0.2, 0.2, 0.1)),
        MyMachine {
            tp: t,
            pos: IVec2 { x: 15, y: 4 },
            direction: Direction2D::Left,
        },
    ));
}

fn update_machines(
    mut commands: Commands,
    // ass: Res<AssetServer>,
    q_machinetypes: Query<&MachineType>,
    mut q_machines: Query<(Entity, &MyMachine, Option<&mut Transform>)>,
    mres: Res<MachineResources>,
) {
    for (e, machine, spawn) in q_machines.iter_mut() {
        let trans = Transform::from_translation(machine.pos.extend(0).xzy().as_vec3())
            .with_rotation(machine.direction.into());

        match spawn {
            None => {
                let tp = q_machinetypes.get(machine.tp).unwrap();

                commands.entity(e).insert(SceneBundle {
                    scene: tp.scene.clone(),
                    transform: trans,
                    ..default()
                });

                // let bx = commands
                //     .spawn((
                //         DebugCube,
                //         PbrBundle {
                //             mesh: mres.selection_cube.clone(),
                //             material: mres.debug_reddish.clone(),
                //             transform: Transform::from_scale(Vec3::new(
                //                 tp.dims.x as f32,
                //                 32.0,
                //                 tp.dims.y as f32,
                //             )),
                //             ..default()
                //         },
                //     ))
                //     .id();

                // commands.entity(e).push_children(&[bx]);
            }
            Some(mut ts) => {
                *ts = trans;
            }
        }
    }
}

fn update_boxes(
    // mut commands: Commands,
    // ass: Res<AssetServer>,
    q_machinetypes: Query<&MachineType>,
    mut q_machines: Query<(Entity, &MyMachine, &Children)>,
    mut q_cubes: Query<(&DebugCube, &mut Transform)>,
    // mres: Res<MachineResources>,
) {
    for (e, machine, children) in q_machines.iter_mut() {
        let tp = q_machinetypes.get(machine.tp).unwrap();

        for ch in children {
            let Ok((cube, mut trans)) = q_cubes.get_mut(*ch) else {
                continue;
            };

            *trans = Transform::from_scale(Vec3::new(tp.dims.x as f32, 32.0, tp.dims.y as f32));
        }
    }
}
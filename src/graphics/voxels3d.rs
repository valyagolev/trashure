use bevy::{prelude::*, render::camera::ScalingMode, utils::HashMap};
use rand::seq::SliceRandom;

use crate::conf::Configuration;

use super::pieces::Material;

pub struct Voxels3d;
impl Plugin for Voxels3d {
    fn build(&self, app: &mut App) {
        app.add_systems(
            // OnEnter(AtlasesPluginState::Finished),
            Startup, setup,
        );
        // .add_systems(Update, camera_setup)
        // .add_systems(Update, handle_camera_move);
    }
}

#[derive(Debug, Resource, Reflect)]
pub struct VoxelResources {
    pub mesh: Handle<Mesh>,
    materials: [Handle<StandardMaterial>; 4],
}

impl VoxelResources {
    pub fn material(&self, m: Material) -> Handle<StandardMaterial> {
        match m {
            Material::Reddish => self.materials[0].clone(),
            Material::Greenish => self.materials[1].clone(),
            Material::Blueish => self.materials[2].clone(),
            Material::Brownish => self.materials[3].clone(),
        }
    }

    pub fn pbr_bundle(&self, m: Material, pos: Vec3) -> PbrBundle {
        PbrBundle {
            mesh: self.mesh.clone(),
            material: self.material(m).clone(),
            transform: Transform::from_translation(pos),
            ..default()
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // camera3d_transform: Query<(&GlobalTransform, &Camera), With<Camera3d>>,
    // camera2d_transform: Query<(&GlobalTransform, &Camera), With<Camera2d>>,
    // emojis: Res<Emojis>,
) {
    // let camera3d_transform = camera3d_transform.single();
    // let camera2d_transform = camera2d_transform.single();

    let cube_mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let cube_materials = HashMap::from([
        ("redish", materials.add(Color::rgb(0.8, 0.5, 0.4).into())),
        ("greenish", materials.add(Color::rgb(0.5, 0.8, 0.4).into())),
        ("blueish", materials.add(Color::rgb(0.4, 0.5, 0.8).into())),
        ("brownish", materials.add(Color::rgb(0.8, 0.7, 0.6).into())),
    ]);

    commands.insert_resource(VoxelResources {
        mesh: cube_mesh.clone(),
        materials: [
            cube_materials.get("redish").unwrap().clone(),
            cube_materials.get("greenish").unwrap().clone(),
            cube_materials.get("blueish").unwrap().clone(),
            cube_materials.get("brownish").unwrap().clone(),
        ],
    });

    // // let rand = &mut rand::thread_rng();
    // // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(1000.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        transform: Transform::from_xyz(0.0, -0.5, 0.0),
        ..default()
    });
    // // cubes
    // for row in 0..6 {
    //     let half_radius = 6 - row;

    //     for i in -half_radius..half_radius {
    //         for j in -half_radius..half_radius {
    //             let pos = Vec3::new(i as f32, row as f32, j as f32);

    //             commands.spawn(PbrBundle {
    //                 mesh: cube_mesh.clone(),
    //                 material: cube_materials
    //                     .get(
    //                         *["redish", "greenish", "blueish", "brownish"]
    //                             .choose(rand)
    //                             .unwrap(),
    //                     )
    //                     .unwrap()
    //                     .clone(),
    //                 transform: Transform::from_translation(pos),
    //                 ..default()
    //             });

    //             // let viewport_pos = camera3d_transform.1.world_to_ndc(camera3d_transform.0, pos);

    //             // let pos2d = camera2d_transform
    //             //     .1
    //             //     .viewport_to_world_2d(camera2d_transform.0, viewport_pos.unwrap().xy())
    //             //     .unwrap();
    //             // // .world_to_viewport(camera_transform.0, pos);
    //             // dbg!(viewport_pos, pos2d);

    //             // let mut sbundle = emojis
    //             //     .sbundle(emojis.random_emoji())
    //             //     .expect("couldn't find emoji?");

    //             // sbundle.transform = Transform::from_xyz(pos2d.x, pos2d.y, 0.0);

    //             // commands.spawn(sbundle);
    //         }
    //     }
    // }
}

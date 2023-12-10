use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_inspector_egui::quick::{FilterQueryInspectorPlugin, ResourceInspectorPlugin};

use crate::graphics::gamemenu::GameMenuButton;
use crate::graphics::machines::radar::Radar;
use crate::graphics::machines::targets::TargetInst;
use crate::graphics::machines::{BuiltMachine, MachineType, MyMachine};
use crate::graphics::recolor::Tinted;
use crate::graphics::selectable::CurrentlySelected;

pub struct DebugEditorPlugin;
impl Plugin for DebugEditorPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_plugins(EguiPlugin)
            // .add_plugins(ResourceInspectorPlugin::<CurrentlySelected>::default())
            // .add_plugins(FilterQueryInspectorPlugin::<With<Camera3d>>::default())
            // .add_plugins(FilterQueryInspectorPlugin::<With<PointLight>>::default())
            // .add_plugins(FilterQueryInspectorPlugin::<With<DirectionalLight>>::default());
            // .add_plugins(FilterQueryInspectorPlugin::<
            //     Or<(With<MyMachine>, With<MachineType>)>,
            // >::default())
            .add_plugins(FilterQueryInspectorPlugin::<With<BuiltMachine>>::default());
        // .add_systems(Update, edit_stuff)
    }
}

fn edit_stuff(
    mut contexts: EguiContexts,
    // reg: Res<bevy::ecs::reflect::AppTypeRegistry>,
    // mut q_camera: Query<(
    //     &mut Transform,
    //     &mut Camera,
    //     &mut OrthographicProjection,
    //     &Children,
    // )>,
    // mut q_camera_light: Query<(&mut Transform, &mut PointLight)>,
    // mut r_amb: ResMut<AmbientLight>,
) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        // let (mut camera_ts, camera, camera_proj, camera_children) = q_camera.single_mut();

        // let camera_light_e = camera_children
        //     .iter()
        //     .filter(|c| q_camera_light.get(**c).is_ok())
        //     .next()
        //     .unwrap();
        // let (camera_light_ts, camera_light) = q_camera_light.get_mut(*camera_light_e).unwrap();

        // let reg = reg.read();

        ui.label("world");

        // bevy_inspector_egui::reflect_inspector::ui_for_value(&mut camera_ts.translation, ui, &reg);
    });
}

use bevy::prelude::*;
use bevy_mod_raycast::immediate::{Raycast, RaycastSettings, RaycastVisibility};

use super::{cursor::CursorOver, recolor::Tinted};

pub struct SelectablePlugin;

impl Plugin for SelectablePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentlySelected(None))
            .add_systems(Update, handle_selection);
    }
}

#[derive(Component)]
pub struct Selectable;

#[derive(Resource, Deref, Reflect)]
pub struct CurrentlySelected(pub Option<Entity>);

fn handle_selection(
    mut raycast: Raycast,
    mut q_targets: Query<
        (Entity, &GlobalTransform, &ViewVisibility, &mut Tinted),
        With<Selectable>,
    >,
    mouse: Res<CursorOver>,
    parent_query: Query<&Parent>,
    mouse_inp: Res<Input<MouseButton>>,
    mut currently_selected: ResMut<CurrentlySelected>,
) {
    let valid_entities = q_targets
        .iter()
        .filter(|tpl| tpl.2.get())
        .map(|tpl| tpl.0)
        .collect::<Vec<_>>();

    if valid_entities.is_empty() {
        return;
    }

    let hit = raycast
        .cast_ray(
            mouse.ray.into(),
            &RaycastSettings {
                visibility: RaycastVisibility::MustBeVisible,
                filter: &|entity: Entity| {
                    parent_query
                        .iter_ancestors(entity)
                        .find(|anc| valid_entities.contains(&anc))
                        .is_some()
                },
                early_exit_test: &|_| true,
            },
        )
        .get(0);

    let hovered_inst = hit.and_then(|hit| {
        parent_query
            .iter_ancestors(hit.0)
            .find(|anc| valid_entities.contains(&anc))
    });

    for t in valid_entities {
        let mut tpl = q_targets.get_mut(t).unwrap();

        if hovered_inst == Some(t) {
            *tpl.3 = Tinted::new(Color::rgb(0.3, 0.0, 0.1));
        } else {
            *tpl.3 = Tinted::empty();
        }
    }

    // if mouse_inp.pressed(MouseButton::Left) {
    //     if let Some(hovered_inst) = hovered_inst {
    //         let tpl = q_targets.get(hovered_inst).unwrap();

    //         let delta = mouse.ground - tpl.1.translation().xz();

    //         target_being_moved.0 = Some((tpl.3 .0, delta));
    //     }
    // }
}

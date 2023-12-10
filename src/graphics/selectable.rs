use bevy::{prelude::*, render::view::RenderLayers};
use bevy_mod_raycast::immediate::{Raycast, RaycastSettings, RaycastVisibility};

use crate::game::machines::GameMachineSettingsDiscriminants;

use super::{
    cursor::CursorOver,
    gamemenu::{tutorial::mark_tutorial_event, GameMenu, GameMenuState},
    machines::{
        radar::{Radar, RadarScene},
        MyMachine,
    },
    recolor::Tinted,
    scenerenderlayer::SceneRenderLayers,
};

pub struct SelectablePlugin;

impl Plugin for SelectablePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentlySelected(None)).add_systems(
            Update,
            (
                handle_selection,
                recolor_selection,
                // handle_deselection
            ),
        );
    }
}

#[derive(Component)]
pub struct Selectable;

#[derive(Resource, Deref, Reflect)]
pub struct CurrentlySelected(pub Option<Entity>);

fn recolor_selection(
    mut q_targets: Query<
        (Entity, &mut Tinted, &mut SceneRenderLayers, &Children),
        With<Selectable>,
    >,
    currently_selected: Res<CurrentlySelected>,
    mut q_radars: Query<&mut Visibility, With<Radar>>,
) {
    for (ent, mut tpl, mut layers, children) in q_targets.iter_mut() {
        if Some(ent) == currently_selected.0 {
            *tpl = Tinted::new(Color::rgb(0.0, 0.0, 0.1));
            *layers = SceneRenderLayers(RenderLayers::layer(6))
        } else {
            *tpl = Tinted::empty();
            *layers = SceneRenderLayers(RenderLayers::default())
        }

        for child in children {
            let Ok(mut child) = q_radars.get_mut(*child) else {
                continue;
            };

            if Some(ent) == currently_selected.0 {
                *child = Visibility::Visible;
            } else {
                *child = Visibility::Hidden;
            }
        }
    }
}

fn handle_selection(
    mut raycast: Raycast,
    q_targets: Query<
        (
            Entity,
            &GlobalTransform,
            &ViewVisibility,
            Option<&MyMachine>,
        ),
        With<Selectable>,
    >,
    mouse: Res<CursorOver>,
    parent_query: Query<&Parent>,
    mouse_inp: Res<Input<MouseButton>>,
    mut currently_selected: ResMut<CurrentlySelected>,
    mut menu: ResMut<GameMenu>,
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
                        .any(|anc| valid_entities.contains(&anc))
                },
                early_exit_test: &|_| true,
            },
        )
        .get(0);

    let Some(hovered_inst) = hit.and_then(|hit| {
        parent_query
            .iter_ancestors(hit.0)
            .find(|anc| valid_entities.contains(anc))
    }) else {
        return;
    };

    if mouse_inp.just_pressed(MouseButton::Left) {
        currently_selected.0 = Some(hovered_inst);

        menu.0 = GameMenuState::SelectedMachine;

        if let Ok((_, _, _, Some(m))) = q_targets.get(hovered_inst) {
            if m.gmt == GameMachineSettingsDiscriminants::Recycler {
                mark_tutorial_event("recycler_selected");
            }
        }
    }
}

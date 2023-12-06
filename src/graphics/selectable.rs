use bevy::prelude::*;

pub struct SelectablePlugin;

impl Plugin for SelectablePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentlySelected(None));
    }
}

#[derive(Component)]
pub struct Selectable;

#[derive(Resource)]
pub struct CurrentlySelected(pub Option<Entity>);

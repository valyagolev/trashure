use std::sync::atomic::{AtomicBool, Ordering};

use bevy::{prelude::*, utils::HashMap};

use super::{
    textref::{QueryTexts, TextRefs},
    TutorialNode,
};

static EVENTS: [(&'static str, AtomicBool, &'static str); 6] = [
    ("recycler_placed", AtomicBool::new(false), "Welcome to Trashure: a game about recycling a massive (endless actually) landfill. You can look around using arrow keys.\n\n\nThen, start by building a recycler."),
    ("recycler_finished", AtomicBool::new(false), "Machines consume green material to be built. Wait for that to finish."),
    ("recycler_selected", AtomicBool::new(false), "Select the recycler by clicking on it. You'll see the radars.\n\nBlue radar looks for fuel, brownish radar looks for work.\n\nRecycler's work is to recycle trash into fuel (blue), building material (green), and precious maintenance (red) that doesn't occur by itself."),
    ("plower_built", AtomicBool::new(false), "Blue radar looks for fuel, brownish radar looks for work.\n\nWait for the recycler to clear out a bit of space to build your plower – then build it.\n\nRecycler's work is to recycle trash into flue (blue), building material (green), and precious maintenance (red) that doesn't occur by itself."),
    ("plower_wants_maintenance", AtomicBool::new(false), "Plower's work is to send resources to another place, which is handy because the recycler will work very slowly if it doesn't have any resources in front of it. It also consumes a lot of fuel when moving."),
    ("plower_maintained", AtomicBool::new(false), "Plower has stopped, because it needs maintenance, which is provided by the precious red materials. Those do not occur by themselves, and need the recycler to be obtained. Plower will use its radar to find maintenance."),
];

pub fn mark_tutorial_event(name: &'static str) {
    for (event_name, event, _) in EVENTS.iter() {
        if *event_name == name {
            event.store(true, Ordering::Relaxed);
            return;
        }
    }

    panic!("Unknown tutorial event: {}", name);
}

fn earliest_event() -> Option<(&'static str, &'static str)> {
    for (event_name, event, txt) in EVENTS.iter() {
        if !event.load(Ordering::Relaxed) {
            return Some((*event_name, *txt));
        }
    }

    return None;
}

pub struct TutorialPlugin;

impl Plugin for TutorialPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Self::show_tutorial);
    }
}

impl TutorialPlugin {
    fn show_tutorial(q_tutorial: Query<&TextRefs, With<TutorialNode>>, mut q_texts: QueryTexts) {
        let Ok(tutorial) = q_tutorial.get_single() else {
            return;
        };

        let txt = earliest_event()
            .map(|a| a.1)
            .unwrap_or("You have finished the tutorial. Good luck with your plowing!");

        tutorial.update(&mut q_texts, "text", txt, None);
    }
}

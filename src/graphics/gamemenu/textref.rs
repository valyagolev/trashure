use bevy::{prelude::*, utils::HashMap};

pub struct TextRefPlugin;

impl Plugin for TextRefPlugin {
    fn build(&self, app: &mut App) {}
}

pub type QueryTexts<'a, 'b, 'c> = Query<'a, 'b, &'c mut Text>;

#[derive(Component)]
pub struct TextRefs(pub HashMap<&'static str, (Entity, usize)>);

impl TextRefs {
    pub fn new() -> Self {
        Self(HashMap::default())
    }

    pub fn with(mut self, name: &'static str, ent: Entity, idx: usize) -> Self {
        self.0.insert(name, (ent, idx));
        self
    }

    pub fn update(
        &self,
        q_texts: &mut QueryTexts,
        name: &'static str,
        value: impl Into<String>,
        style: Option<&TextStyle>,
    ) -> Option<()> {
        let (ent, i) = self.0.get(name).unwrap();

        let mut txt = q_texts.get_mut(*ent).ok()?;

        txt.sections[*i].value = value.into();

        if let Some(style) = style {
            txt.sections[*i].style = style.clone();
        }

        Some(())
    }
}

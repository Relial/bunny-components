use bunny_plugin::bunny_ui::{
    Pos2, Rect, Vec2, align::Align2, ui::BunnyUi, widgets::drag_value::DragValue,
};
use serde::{Deserialize, Serialize};

use crate::EditMenu;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelativePosition {
    pub anchor: Align2,
    pub offset: Vec2,
}

impl RelativePosition {
    pub fn new(anchor: Align2, offset: impl Into<Vec2>) -> Self {
        Self {
            anchor,
            offset: offset.into(),
        }
    }

    pub fn pos_in_rect(&self, rect: &Rect) -> Pos2 {
        self.anchor.pos_in_rect(rect) + self.offset
    }
}

impl<'a> EditMenu<'a> for RelativePosition {
    fn edit_menu(&'a mut self, ui: &mut BunnyUi<'a>) {
        ui.horizontal(|ui| {
            ui.label("Anchor");
            self.anchor.edit_menu(ui);
        });
        ui.horizontal(|ui| {
            ui.label("Position");
            ui.add(DragValue::new(&mut self.offset.x).prefix("x"));
            ui.add(DragValue::new(&mut self.offset.y).prefix("y"));
        });
    }
}

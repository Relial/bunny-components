use bunny_plugin::bunny_ui::{Vec2, ui::BunnyUi, widgets::drag_value::DragValue};

use crate::EditMenu;

impl<'a> EditMenu<'a> for Vec2 {
    fn edit_menu(&'a mut self, ui: &mut BunnyUi<'a>) {
        ui.horizontal(|ui| {
            ui.label("Width:");
            ui.add(DragValue::new(&mut self.x).fixed_decimals(1).speed(0.01));
        });
        ui.horizontal(|ui| {
            ui.label("Height:");
            ui.add(DragValue::new(&mut self.y).fixed_decimals(1).speed(0.01));
        });
    }
}

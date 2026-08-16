use bunny_ui::{Vec2, ui::BunnyUi, widgets::drag_value::DragValue};
use glam::Vec3;

use crate::EditMenu;

impl<'a> EditMenu<'a> for Vec2 {
    fn edit_menu(&'a mut self, ui: &mut BunnyUi<'a>) {
        ui.horizontal(|ui| {
            ui.add(
                DragValue::new(&mut self.x)
                    .fixed_decimals(1)
                    .speed(0.01)
                    .prefix("x"),
            );
            ui.add(
                DragValue::new(&mut self.y)
                    .fixed_decimals(1)
                    .speed(0.01)
                    .prefix("y"),
            );
        });
    }
}

impl<'a> EditMenu<'a> for Vec3 {
    fn edit_menu(&'a mut self, ui: &mut BunnyUi<'a>) {
        ui.horizontal(|ui| {
            ui.add(
                DragValue::new(&mut self.x)
                    .fixed_decimals(1)
                    .speed(0.01)
                    .prefix("x"),
            );
            ui.add(
                DragValue::new(&mut self.y)
                    .fixed_decimals(1)
                    .speed(0.01)
                    .prefix("y"),
            );
            ui.add(
                DragValue::new(&mut self.z)
                    .fixed_decimals(1)
                    .speed(0.01)
                    .prefix("z"),
            );
        });
    }
}

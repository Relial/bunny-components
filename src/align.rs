use bunny_ui::{align::Align2, containers::combo_box::ComboBox, ui::BunnyUi};

use crate::{EditMenu, display::MenuDisplay};

impl EditMenu<'_> for Align2 {
    fn edit_menu(&mut self, ui: &mut BunnyUi) {
        const ANCHORS: [Align2; 9] = [
            Align2::LEFT_BOTTOM,
            Align2::LEFT_CENTER,
            Align2::LEFT_TOP,
            Align2::CENTER_BOTTOM,
            Align2::CENTER_CENTER,
            Align2::CENTER_TOP,
            Align2::RIGHT_BOTTOM,
            Align2::RIGHT_CENTER,
            Align2::RIGHT_TOP,
        ];
        ComboBox::from_id(ui.next_id())
            .selected_text(self.menu_display())
            .show_ui(ui, |ui| {
                for anchor in ANCHORS {
                    ui.selectable_value(self, anchor, anchor.menu_display());
                }
            });
    }
}

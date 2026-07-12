use bunny_plugin::bunny_ui::ui::BunnyUi;

pub mod align;
pub mod color;
pub mod position;
pub mod progress_bar;
pub mod text;
pub mod vec;

mod display;

pub trait EditMenu<'a> {
    fn edit_menu(&'a mut self, ui: &mut BunnyUi<'a>);
}

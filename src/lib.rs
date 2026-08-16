use bunny_ui::ui::BunnyUi;

mod align;
mod position;
mod progress_bar;
mod text;
mod vec;
mod display;

pub use position::*;
pub use progress_bar::*;
pub use text::*;

pub trait EditMenu<'a> {
    fn edit_menu(&'a mut self, ui: &mut BunnyUi<'a>);
}

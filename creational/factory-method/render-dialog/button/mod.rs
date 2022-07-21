pub mod html;
pub mod windows;

pub trait Button {
    fn render(&self);
    fn on_click(&self);
}

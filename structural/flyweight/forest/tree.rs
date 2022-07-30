use std::rc::Rc;

use draw::{Canvas, Color, Drawing, Shape, Style, RGB};

pub struct Tree {
    x: u32,
    y: u32,
    kind: Rc<TreeKind>,
}

impl Tree {
    pub fn new(x: u32, y: u32, kind: Rc<TreeKind>) -> Self {
        Self { x, y, kind }
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        self.kind.draw(canvas, self.x, self.y);
    }
}

pub struct TreeKind {
    color: RGB,
    _name: String,
    _data: String,
}

impl TreeKind {
    pub fn new(name: String, color: RGB, data: String) -> Self {
        Self {
            _name: name,
            color,
            _data: data,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas, x: u32, y: u32) {
        let rect = Drawing::new()
            .with_xy(x.saturating_sub(1) as f32, y as f32)
            .with_shape(Shape::Rectangle {
                width: 2,
                height: 5,
            })
            .with_style(Style::filled(Color::black()));

        let circle = Drawing::new()
            .with_xy(x as f32, y.saturating_sub(5) as f32)
            .with_shape(Shape::Circle { radius: 5 })
            .with_style(Style::filled(self.color));

        canvas.display_list.add(rect);
        canvas.display_list.add(circle);
    }
}

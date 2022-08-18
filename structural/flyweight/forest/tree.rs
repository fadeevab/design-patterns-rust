use std::rc::Rc;

use draw::{Canvas, Drawing, Shape, Style, RGB};

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum TreeColor {
    Color1,
    Color2,
    TrunkColor,
}

impl TreeColor {
    fn rgb(&self) -> RGB {
        match self {
            Self::Color1 => RGB::new(0x17, 0xd7, 0xa0),
            Self::Color2 => RGB::new(0xd8, 0x21, 0x48),
            Self::TrunkColor => RGB::new(0x15, 0x1d, 0x3b),
        }
    }
}

/// A cacheable item. It derives `PartialEq`, `Eq`, and `Hash` in order to be
/// used in the `HashSet`.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TreeKind {
    color: TreeColor,
    _name: String,
    _data: String,
}

impl TreeKind {
    pub fn new(color: TreeColor, _name: String, _data: String) -> Self {
        Self {
            color,
            _name,
            _data,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas, x: u32, y: u32) {
        let rect = Drawing::new()
            .with_xy(x.saturating_sub(2) as f32, y as f32)
            .with_shape(Shape::Rectangle {
                width: 4,
                height: 5,
            })
            .with_style(Style::filled(TreeColor::TrunkColor.rgb()));

        let circle = Drawing::new()
            .with_xy(x as f32, y.saturating_sub(5) as f32)
            .with_shape(Shape::Circle { radius: 5 })
            .with_style(Style::filled(self.color.rgb()));

        canvas.display_list.add(rect);
        canvas.display_list.add(circle);
    }
}

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

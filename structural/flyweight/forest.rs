mod tree;

use draw::Canvas;
use std::{collections::HashMap, rc::Rc};
use tree::{Tree, TreeKind};

pub use self::tree::TreeColor;

#[derive(Default)]
pub struct Forest {
    pub tree_kinds: HashMap<String, Rc<TreeKind>>,
    pub trees: Vec<Tree>,
}

impl Forest {
    pub fn plant_tree(&mut self, x: u32, y: u32, color: TreeColor, name: String, data: String) {
        // Here is the substance of the Flywheight Pattern:
        // there is always a single instance of a "tree kind" structure.
        let tree_kind = self
            .tree_kinds
            .entry(name.clone())
            .or_insert(Rc::new(TreeKind::new(color, name.clone(), data)));

        // A tree kind is referenced from each tree instance using `Rc` pointer.
        // `tree_kind.clone()` increases a reference counter instead of real cloning.
        let tree = Tree::new(x, y, tree_kind.clone());
        self.trees.push(tree);
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        for tree in &self.trees {
            tree.draw(canvas);
        }
    }
}

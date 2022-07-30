mod tree;

use draw::{Canvas, RGB};
use std::{collections::HashMap, rc::Rc};
use tree::{Tree, TreeKind};

#[derive(Default)]
pub struct Forest {
    pub tree_kinds: HashMap<String, Rc<TreeKind>>,
    pub trees: Vec<Tree>,
}

impl Forest {
    pub fn plant_tree(&mut self, x: u32, y: u32, name: String, color: RGB, data: String) {
        // Here is the "trick": there is always a single instance of a "tree kind" structure.
        let tree_kind = self
            .tree_kinds
            .entry(name.clone())
            .or_insert(Rc::new(TreeKind::new(name.clone(), color, data)));

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

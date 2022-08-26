mod tree;

use draw::Canvas;
use std::{collections::HashSet, rc::Rc};
use tree::{Tree, TreeKind};

pub use self::tree::TreeColor;

/// Forest implements an internal cache that is hidden behind the public API.
///
/// The point is having an opaque cache implementation. It can use a hash set,
/// FIFO, or even a simple vector.
///
/// Here are the key points:
/// - `cache` is of `HashSet` type, so it can hold only a single
///    instance of a `TreeKind`,
/// - `Rc` is needed to get the reference on the tree kind without
///    cloning the full structure,
/// - `TreeKind` must derive `Eq`, `PartialEq`, and `Hash` traits to be
///    used in the `HashSet`.
#[derive(Default)]
pub struct Forest {
    cache: HashSet<Rc<TreeKind>>,
    trees: Vec<Tree>,
}

impl Forest {
    pub fn plant_tree(&mut self, x: u32, y: u32, color: TreeColor, name: String, data: String) {
        let tree_kind = TreeKind::new(color, name, data);

        // Here is an essence of Flyweight: it's an internal cache,
        // there is always a single instance of a "tree kind" structure.
        self.cache.insert(Rc::new(tree_kind.clone()));

        // A tree kind is referenced from each tree instance using `Rc` pointer.
        // `tree_kind.clone()` increases a reference counter instead of real cloning.
        let tree = Tree::new(x, y, self.cache.get(&tree_kind).unwrap().clone());
        self.trees.push(tree);
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        for tree in &self.trees {
            tree.draw(canvas);
        }
    }

    pub fn cache_len(&self) -> usize {
        self.cache.len()
    }
}

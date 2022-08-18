mod forest;

use draw::{render, render::svg::SvgRenderer, Canvas};
use rand::Rng;

use crate::forest::{Forest, TreeColor};

const CANVAS_SIZE: u32 = 500;
const TREES_TO_DRAW: u32 = 100000;
const TREE_TYPES: u32 = 2;

fn main() {
    let forest = &mut Forest::default();

    for _ in 0..TREES_TO_DRAW / TREE_TYPES {
        let mut rng = rand::thread_rng();

        forest.plant_tree(
            rng.gen_range(0..CANVAS_SIZE),
            rng.gen_range(0..CANVAS_SIZE),
            TreeColor::Color1,
            "Summer Oak".into(),
            "Oak texture stub".into(),
        );

        forest.plant_tree(
            rng.gen_range(0..CANVAS_SIZE),
            rng.gen_range(0..CANVAS_SIZE),
            TreeColor::Color2,
            "Autumn Oak".into(),
            "Autumn Oak texture stub".into(),
        );
    }

    let mut canvas = Canvas::new(CANVAS_SIZE, CANVAS_SIZE);
    forest.draw(&mut canvas);

    render::save(&canvas, "res/forest.svg", SvgRenderer::new()).expect("Rendering");

    println!("{} trees drawn", TREES_TO_DRAW);
    println!("Cache length: {} tree kinds", forest.cache_len());
    println!("-------------------------------");
    println!("Memory usage:");
    println!("Tree size (16 bytes) * {}", TREES_TO_DRAW);
    println!("+ TreeKind size (~30 bytes) * {}", TREE_TYPES);
    println!("-------------------------------");
    println!(
        "Total: {}MB (instead of {}MB)",
        ((TREES_TO_DRAW * 16 + TREE_TYPES * 30) / 1024 / 1024),
        ((TREES_TO_DRAW * 46) / 1024 / 1024)
    );
}

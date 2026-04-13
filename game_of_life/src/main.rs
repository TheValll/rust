pub mod grid;
pub mod game;

use std::{thread, time};
use rand::Rng;
use grid::display_grid;
use game::rules::apply_rules;
use grid::cell::Cells;

const MAX_GENERATION: u32 = 100;
const GRID_HEIGHT: u32 = 60;
const GRID_WIDTH: u32 = 100;
const SPEED: time::Duration = time::Duration::from_millis(300);

fn main() {
    let mut generation: u32 = 0;
    let mut rng = rand::thread_rng();
    let mut grid: Vec<Vec<Cells>> = (0..GRID_HEIGHT)
    .map(|_| {
        (0..GRID_WIDTH)
            .map(|_| match rng.gen_range(0..2){
                0 => Cells::Dead,
                _ => Cells::Alive,
            })
            .collect()
    })
    .collect();

    while generation < MAX_GENERATION {
        display_grid(&grid);
        println!("Génération : {}", generation);

        let (next_grid, next_gen) = apply_rules(&grid, generation);
        grid = next_grid;
        generation = next_gen;

        thread::sleep(SPEED);
    }
}
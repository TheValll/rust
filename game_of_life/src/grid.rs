pub mod cell;
use cell::Cells;

pub fn display_grid(grid: &Vec<Vec<Cells>>) {
    print!("{}[2J", 27 as char);

    for row in grid {
        for cell in row {
            print!("{}", if matches!(cell, Cells::Alive) { "■ " } else { ". " });
        }
        println!();
    }
}

use crate::grid::cell::Cells;

pub fn apply_rules(grid: &Vec<Vec<Cells>>, generation: u32) -> (Vec<Vec<Cells>>, u32) {
    let mut next_generation = grid.clone();
    let height = grid.len();
    let width = grid[0].len();

    for (row_index, row_value) in grid.iter().enumerate() {
        for (cell_index, cell_value) in row_value.iter().enumerate() {
            let mut active_neighbors_count = 0;
            
            if row_index > 0 {
                if matches!(grid[row_index - 1][cell_index], Cells::Alive) { active_neighbors_count += 1; }
                if cell_index > 0 && matches!(grid[row_index - 1][cell_index - 1], Cells::Alive) { active_neighbors_count += 1; }
                if cell_index < width - 1 && matches!(grid[row_index - 1][cell_index + 1], Cells::Alive) { active_neighbors_count += 1; }
            }

            if cell_index > 0 && matches!(grid[row_index][cell_index - 1], Cells::Alive) { active_neighbors_count += 1; }
            if cell_index < width - 1 && matches!(grid[row_index][cell_index + 1], Cells::Alive) { active_neighbors_count += 1; }

            if row_index < height - 1 {
                if matches!(grid[row_index + 1][cell_index], Cells::Alive) { active_neighbors_count += 1; }
                if cell_index > 0 && matches!(grid[row_index + 1][cell_index - 1], Cells::Alive) { active_neighbors_count += 1; }
                if cell_index < width - 1 && matches!(grid[row_index + 1][cell_index + 1], Cells::Alive) { active_neighbors_count += 1; }
            }

            next_generation[row_index][cell_index] = match (*cell_value, active_neighbors_count) {
                (Cells::Alive, 2) | (Cells::Alive, 3) => Cells::Alive,
                (Cells::Dead, 3)                      => Cells::Alive,
                _                                     => Cells::Dead, 
            };
        }
    } 

    (next_generation, generation + 1)
}
mod pair_solvers;

use grid::Grid;

use super::verlet_object::VerletObject;

const GRID_ROWS: u16 = 50;
const GRID_COLS: u16 = 50;

const ROW_HEIGHT: f32 = 2.0 / GRID_ROWS as f32;
const COL_WIDTH: f32 = 2.0 / GRID_COLS as f32;

const ROWS: usize = GRID_ROWS as usize;
const COLS: usize = GRID_COLS as usize;

fn get_index_grid(objects: &[VerletObject]) -> Grid<Vec<usize>> {
    let mut grid: Grid<Vec<usize>> = Grid::new(GRID_ROWS as usize, GRID_COLS as usize);

    for (idx, obj) in objects.iter().enumerate() {
        let mut i = obj.get_center().x;
        let mut j = obj.get_center().y;

        i += 1.0;
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let i = (i / COL_WIDTH).abs().trunc() as usize;

        j += 1.0;
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let j = (j / ROW_HEIGHT).abs().trunc() as usize;

        grid[i][j].push(idx);
    }

    grid
}

fn get_grid(objects: &[VerletObject]) -> Grid<Vec<VerletObject>> {
    let mut grid: Grid<Vec<VerletObject>> = Grid::new(GRID_ROWS as usize, GRID_COLS as usize);

    for obj in objects {
        let i = obj.get_center().x + 1.0;
        let i = (i / COL_WIDTH).abs().trunc() as usize;

        let j = obj.get_center().y + 1.0;
        let j = (j / ROW_HEIGHT).abs().trunc() as usize;

        grid[i][j].push(*obj);
    }

    grid
}

pub fn solve_grid(objects: &mut [VerletObject]) {
    let grid = get_index_grid(objects);

    for row in 1..ROWS - 1 {
        for col in 1..COLS - 1 {
            let start_row = row - 1;
            let end_row = row + 1;

            let start_col = col - 1;
            let end_col = col + 1;

            let mut big_pocket = Vec::new();

            for row in start_row..end_row {
                for col in start_col..end_col {
                    big_pocket.append(&mut grid[row][col].clone());
                }
            }

            solve_at(objects, &big_pocket);
        }
    }
}

// TODO: I'm doing it 4 times more than I should. wth?
pub fn solve_grid_chunks_cloning(objects: &mut Vec<VerletObject>) {
    for offset_rows in 1..4 {
        for offset_cols in 1..4 {
            let mut grid = get_grid(objects);
            objects.clear();
            for row in (offset_rows..ROWS - 1).step_by(3) {
                for col in (offset_cols..COLS - 1).step_by(3) {
                    let start_row = row - 1;
                    let end_row = row + 1;

                    let start_col = col - 1;
                    let end_col = col + 1;

                    let mut big_pocket = Vec::new();

                    for row in start_row..end_row {
                        for col in start_col..end_col {
                            // let pocket = ;
                            big_pocket.append(&mut grid[row][col]);
                        }
                    }
                    solve(&mut big_pocket);
                    objects.append(&mut big_pocket);
                }
            }
            grid.iter_mut().for_each(|vec| {
                objects.append(vec);
            });
        }
    }
}

pub fn solve_at(objects: &mut [VerletObject], indexes: &[usize]) {
    for i in 0..indexes.len() {
        for j in i + 1..indexes.len() {
            let idx0 = indexes[i];
            let idx1 = indexes[j];

            if let Some((adjustment0, adjustment1)) =
                pair_solvers::get_adjustments(objects[idx0].into(), objects[idx1].into())
            {
                objects[idx0].shift(adjustment0);
                objects[idx1].shift(adjustment1);
            }
        }
    }
}

pub fn solve(objects: &mut [VerletObject]) {
    let len = objects.len();
    for i in 0..len {
        for j in i + 1..len {
            if let Some((adjustment0, adjustment1)) =
                pair_solvers::get_adjustments(objects[i].into(), objects[j].into())
            {
                objects[i].shift(adjustment0);
                objects[j].shift(adjustment1);
            }
        }
    }
}

pub fn solve_grid_chunks(objects: &mut [VerletObject]) {
    let grid = get_index_grid(objects);

    for offset_rows in 1..4 {
        for offset_cols in 1..4 {
            for row in (offset_rows..ROWS - 1).step_by(3) {
                for col in (offset_cols..COLS - 1).step_by(3) {
                    let start_row = row - 1;
                    let end_row = row + 1;

                    let start_col = col - 1;
                    let end_col = col + 1;

                    let mut big_pocket = Vec::new();

                    for row in start_row..end_row {
                        for col in start_col..end_col {
                            big_pocket.append(&mut grid[row][col].clone());
                        }
                    }

                    solve_at(objects, &big_pocket);
                }
            }
        }
    }
}

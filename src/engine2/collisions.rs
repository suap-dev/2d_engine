use std::f32::consts::PI;

use grid::Grid;
use itertools::Itertools;

use crate::engine2::verlet_object::VerletObject;

const GRID_ROWS: u16 = 50;
const GRID_COLS: u16 = 50;

const ROW_HEIGHT: f32 = 2.0 / GRID_ROWS as f32;
const COL_WIDTH: f32 = 2.0 / GRID_COLS as f32;

const ROWS: usize = GRID_ROWS as usize;
const COLS: usize = GRID_COLS as usize;

fn get_grid(objects: &[VerletObject]) -> Grid<Vec<usize>> {
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

// pub fn solve_collisions(objects: &mut Vec<VerletObject>) {
//     (0..objects.len()).tuple_combinations().for_each(|(i, j)| {
//         if objects[i].collides_with(&objects[j]) {
//             self.solve_collision(i, j);
//         }
//     });
// }

pub fn solve_grid(objects: &mut [VerletObject]) {
    let grid = get_grid(objects);

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

pub fn solve_at(objects: &mut [VerletObject], indexes: &[usize]) {
    indexes.iter().tuple_combinations().for_each(|(&i, &j)| {
        if let Some((obj1, obj2)) = solve_owned(objects[i], objects[j]) {
            objects[i] = obj1;
            objects[j] = obj2;
        }
    });
}

pub fn solve_grid_chunks(objects: &mut [VerletObject]) {
    let grid = get_grid(objects);

    for offset in 1..4 {
        for row in (offset..ROWS - 1).step_by(3) {
            for offset in 1..4 {
                for col in (offset..COLS - 1).step_by(3) {
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

fn solve_owned(
    mut obj1: VerletObject,
    mut obj2: VerletObject,
) -> Option<(VerletObject, VerletObject)> {
    let centers_distance = obj2.get_center().metric_distance(&obj1.get_center());
    let radius_sum = obj2.get_radius() + obj1.get_radius();

    if centers_distance < radius_sum {
        let delta_versor = (obj2.get_center() - obj1.get_center()).normalize();
        let m1 = PI * obj1.get_radius().powi(2);
        let m2 = PI * obj2.get_radius().powi(2);

        let adjustment_vector = delta_versor * (radius_sum - centers_distance);

        let adjustment1 = -(m2 / (m1 + m2)) * adjustment_vector;
        let adjustment2 = (m1 / (m1 + m2)) * adjustment_vector;

        obj1.shift(adjustment1);
        obj2.shift(adjustment2);
        Some((obj1, obj2))
    } else {
        None
    }
}

fn solve_ref_mut(obj1: &mut VerletObject, obj2: &mut VerletObject) {
    let centers_distance = obj2.get_center().metric_distance(&obj1.get_center());
    let radius_sum = obj2.get_radius() + obj1.get_radius();

    if centers_distance < radius_sum {
        let delta_versor = (obj2.get_center() - obj1.get_center()).normalize();
        let m1 = PI * obj1.get_radius().powi(2);
        let m2 = PI * obj2.get_radius().powi(2);

        let adjustment_vector = delta_versor * (radius_sum - centers_distance);

        let adjustment1 = -(m2 / (m1 + m2)) * adjustment_vector;
        let adjustment2 = (m1 / (m1 + m2)) * adjustment_vector;

        // let mut obj1 = &obj1;
        obj1.shift(adjustment1);
        obj2.shift(adjustment2);
    }
}

fn solve_indexed(objects: &mut [VerletObject], obj1_idx: usize, obj2_idx: usize) {
    let obj1 = &objects[obj1_idx];
    let obj2 = &objects[obj2_idx];

    let centers_distance = obj2.get_center().metric_distance(&obj1.get_center());
    let radius_sum = obj2.get_radius() + obj1.get_radius();

    if centers_distance < radius_sum {
        let delta_versor = (obj2.get_center() - obj1.get_center()).normalize();
        let m1 = PI * obj1.get_radius().powi(2);
        let m2 = PI * obj2.get_radius().powi(2);

        let adjustment_vector = delta_versor * (radius_sum - centers_distance);

        let adjustment1 = -(m2 / (m1 + m2)) * adjustment_vector;
        let adjustment2 = (m1 / (m1 + m2)) * adjustment_vector;

        // let mut obj1 = &obj1;
        objects[obj1_idx].shift(adjustment1);
        objects[obj2_idx].shift(adjustment2);
    }
}

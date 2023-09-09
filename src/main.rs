extern crate rand;
extern crate image;

use rand::Rng;
use image::{Rgb, RgbImage};

fn random_initial_state(n_cells: usize, n_generations: usize) -> Vec<Vec<u8>> {
    let mut rng = rand::thread_rng();
    let first_row: Vec<u8> = (0..n_cells).map(|_| rng.gen_range(1..=2)).collect();
    let mut spacetime: Vec<Vec<u8>> = vec![vec![0; n_cells]; n_generations];
    spacetime[0] = first_row.clone();

    spacetime
}

fn apply_rules(cell: u8, left_neighbor: u8, right_neighbor: u8, rule: u8) -> u8 {
    // Ensure that index is within the bounds of a u8 (0-7)
    let index = ((left_neighbor & 1) << 2) | ((cell & 1) << 1) | (right_neighbor & 1);
    (rule >> index) & 1
}


fn update_grid(grid: &mut Vec<Vec<u8>>, rule: u8) {
    let n_cells = grid[0].len();
    let n_generations = grid.len();

    for i in 0..n_generations - 1 {
        for j in 0..n_cells {
            let left_neighbor = grid[i][(j + n_cells - 1) % n_cells];
            let cell = grid[i][j];
            let right_neighbor = grid[i][(j + 1) % n_cells];

            grid[i + 1][j] = apply_rules(cell, left_neighbor, right_neighbor, rule);
        }
    }
}

use std::io;

fn main() {
    let _rng = rand::thread_rng();
    let mut n_cells_input = String::new();
    println!("Number of cells: ");
    io::stdin().read_line(&mut n_cells_input).expect("Failed to read input");
    let n_cells: usize = n_cells_input.trim().parse().expect("Invalid input");

    let mut n_generations_input = String::new();
    println!("Number of generations: ");
    io::stdin().read_line(&mut n_generations_input).expect("Failed to read input");
    let n_generations: usize = n_generations_input.trim().parse().expect("Invalid input");

    let mut rule_input = String::new();
    println!("Rule number: ");
    io::stdin().read_line(&mut rule_input).expect("Failed to read input");
    let rule: u8 = rule_input.trim().parse().expect("Invalid input");

    let mut initial_state = random_initial_state(n_cells, n_generations);
    let mut rule_image = RgbImage::new(n_cells as u32, n_generations as u32);

    update_grid(&mut initial_state, rule);

    for (i, row) in initial_state.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            let color = if cell == 1 {
                Rgb([0, 0, 255])
            } else {
                Rgb([255, 255, 255])
            };
            rule_image.put_pixel(j as u32, i as u32, color);
        }
    }

    rule_image
        .save("cellular_automaton.png")
        .expect("Failed to save image");
}

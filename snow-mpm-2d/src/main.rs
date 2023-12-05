mod params;
mod particle;
mod grid;

use std::time::Instant;
use macroquad::input::{is_key_pressed, KeyCode};
use macroquad::prelude::{clear_background, Color, draw_text, get_fps, next_frame, screen_height, screen_width};
use macroquad::window::request_new_screen_size;
use crate::grid::Grid;
use crate::particle::Particle;
use nalgebra::{Vector2};
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
use crate::params::Params;

#[macroquad::main("Snow simulation")]
async fn main() {
    request_new_screen_size(1000.0, 1000.0);

    let mut rng = StdRng::seed_from_u64(20);
    let params = Params::new();
    let particle_mass = params.particle_mass.clone();
    let particle_area = params.particle_area.clone();
    let mut grid = Grid::new(64, params);

    let size1: f64 = 0.3;
    let r1: f64 = size1 / 2.0;
    let a1: f64 = size1 * size1;
    let pc1: usize = (a1 / particle_area) as usize;
    for _ in 0..pc1 {
        let pos = Vector2::new(0.1 + size1 * rng.gen::<f64>(), 0.3 + size1 * rng.gen::<f64>());
        if (pos - Vector2::new(0.1 + r1, 0.3 + r1)).norm() < r1 {
            let particle = Particle::new(pos, Vector2::new(5.0, 0.0), particle_mass);
            grid.add_particle(particle);
        }
    }

    let size2: f64 = 0.2;
    let r2: f64 = size2 / 2.0;
    let a2: f64 = size2 * size2;
    let pc2: usize = (a2 / particle_area) as usize;
    for _ in 0..pc2 {
        let pos = Vector2::new(0.7 + size2 * rng.gen::<f64>(), 0.5 + size2 * rng.gen::<f64>());
        if (pos - Vector2::new(0.7 + r2, 0.5 + r2)).norm() < r2 {
            let particle = Particle::new(pos, Vector2::new(-5.0, 0.0), particle_mass);
            grid.add_particle(particle);
        }
    }

    // for pos in positions.iter() {
    //     let position = pos.clone();
    //     let particle = Particle::new(position, Vector2::new(-5.0, 0.0), particle_mass);
    //     grid.add_particle(particle);
    // }

    grid.p2g_mass();
    grid.calculate_volumes();

    let print_time_taken = |start: Instant, name: &str| {
        // println!("{}: {} ms", name, start.elapsed().as_millis());
    };

    let mut sim = false;
    // let mut frame_count: i32 = 1;
    loop {
        if is_key_pressed(KeyCode::Space) {
            sim = !sim;
        }
        if !sim {
            let frame = next_frame().await;
            continue;
        }

        clear_background(Color::new(0.2, 0.2, 0.2, 1.0));

        // Calculate time taken
        let start = Instant::now();
        grid.reset_parameters();
        print_time_taken(start, "Reset parameters");

        let start = Instant::now();
        grid.p2g_mass();
        print_time_taken(start, "P2G mass");

        let start = Instant::now();
        grid.p2g_velocity();
        print_time_taken(start, "P2G velocity");

        let start = Instant::now();
        grid.calculate_volumes();
        print_time_taken(start, "Calculate volumes");

        let start = Instant::now();
        grid.compute_grid_forces();
        print_time_taken(start, "Compute grid forces");

        let start = Instant::now();
        grid.update_grid_velocities();
        print_time_taken(start, "Update grid velocities");

        let start = Instant::now();
        grid.collision_grid();
        print_time_taken(start, "Collision grid");

        let start = Instant::now();
        grid.update_deformation_gradient();
        print_time_taken(start, "Update deformation gradient");

        let start = Instant::now();
        grid.update_velocity();
        print_time_taken(start, "Update velocity");

        let start = Instant::now();
        grid.update_particle_positions();
        print_time_taken(start, "Update particle positions");

        let start = Instant::now();
        grid.draw();
        for particle in &grid.particles {
            particle.draw();
        }
        draw_text("Particle mass view", 10.0, 20.0, 30.0, Color::new(0.0, 1.0, 0.0, 1.0));
        draw_text("Grid mass view", screen_width() / 2.0 + 10.0, 20.0, 30.0, Color::new(0.0, 1.0, 0.0, 1.0));
        draw_text("Particle velocity view", 10.0, screen_height() / 2.0 + 20.0, 30.0, Color::new(0.0, 1.0, 0.0, 1.0));
        draw_text("Grid velocity view", screen_width() / 2.0 + 10.0, screen_height() / 2.0 + 20.0, 30.0, Color::new(0.0, 1.0, 0.0, 1.0));
        print_time_taken(start, "Draw");

        // frame_count += 1;
        let frame = next_frame().await;
        frame
    }
}

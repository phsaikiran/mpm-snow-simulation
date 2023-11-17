use macroquad::prelude::{clear_background, Color, draw_text, get_fps, next_frame};
use crate::grid::Grid;
use crate::particle::Particle;
use nalgebra::{Vector2};
use rand::Rng;
use crate::params::{PARTICLE_MASS};

mod params;
mod particle;
mod grid;

#[macroquad::main("Snow simulation")]
async fn main() {
    let mut rng = rand::thread_rng();
    let mut grid = Grid::new(64);

    // Generate snow cluster 1
    let segment_size_1: f64 = 0.3;
    let radius_1: f64 = segment_size_1 / 2.0;
    let area_1: f64 = segment_size_1 * segment_size_1;
    let particle_count_1: usize = (area_1 / params::PARTICLE_AREA) as usize;
    for _ in 0..particle_count_1 {
        let pos = Vector2::new(0.3 + segment_size_1 * rng.gen::<f64>(), 0.3 + segment_size_1 * rng.gen::<f64>());
        if (pos - Vector2::new(0.3 + radius_1, 0.3 + radius_1)).norm() < radius_1 {
            let particle = Particle::new(pos, Vector2::new(10.0, 0.0), PARTICLE_MASS);
            grid.add_particle(particle);
        }
    }

    // Generate snow cluster 2
    let segment_size_2: f64 = 0.2;
    let radius_2: f64 = segment_size_2 / 2.0;
    let area_2: f64 = segment_size_2 * segment_size_2;
    let particle_count_2: usize = (area_2 / params::PARTICLE_AREA) as usize;
    for _ in 0..particle_count_2 {
        let pos = Vector2::new(0.7 + segment_size_2 * rng.gen::<f64>(), 0.3 + segment_size_2 * rng.gen::<f64>());
        if (pos - Vector2::new(0.7 + radius_2, 0.3 + radius_2)).norm() < radius_2 {
            let particle = Particle::new(pos, Vector2::new(-10.0, 0.0), PARTICLE_MASS);
            grid.add_particle(particle);
        }
    }

    grid.p2g_mass();
    grid.calculate_volumes();

    // let mut frame_count: i32 = 1;
    loop {
        // if frame_count > 200 {
        //     break;
        // }

        clear_background(Color::new(0.2, 0.2, 0.2, 1.0));

        grid.reset_parameters();
        grid.p2g_mass();
        grid.p2g_velocity();
        grid.compute_grid_forces();
        grid.update_grid_velocities();
        grid.collision_grid();
        grid.update_deformation_gradient();
        grid.update_velocity();
        grid.update_particle_positions();

        for particle in &grid.particles {
            particle.draw();
        }
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 30.0, Color::new(0.0, 1.0, 0.0, 1.0));

        // frame_count += 1;
        next_frame().await
    }
}

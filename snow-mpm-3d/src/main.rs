mod particle;
mod grid;
mod helpers;
mod params;
mod plane;

use std::f32::consts::PI;
use nalgebra::Vector3;
use three_d::{Camera, ClearState, CpuTexture, DirectionalLight, FrameOutput, Mat4, OrbitControl, radians, Srgba, vec3, Window, WindowSettings};
use three_d_asset::TextureData;
use crate::grid::Grid;
use crate::params::Params;
use crate::plane::{Cube, Plane};
use three_d_asset::io::Serialize;

pub fn main() {
    let window = Window::new(WindowSettings {
        title: "Snow Simulation".to_string(),
        max_size: Some((800, 600)),
        ..Default::default()
    }).unwrap();
    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(3.624689, 3.1030223, 9.828069),
        vec3(2.224157, 1.4711663, 4.0509143),
        vec3(0.0, 1.0, 0.0),
        radians(std::f32::consts::FRAC_PI_4),
        0.1,
        1000.0,
    );
    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);

    let light0 = DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(0.0, -0.5, -0.5));
    let light1 = DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(0.0, 0.5, 0.5));

    let young_modulus: f32 = 1.4e5;
    let poisson_ration: f32 = 0.2;
    let hardening_coefficient: f32 = 10.0;
    let critical_compression: f32 = 2.5e-2;
    let critical_stretch: f32 = 7.5e-3;
    let flip_pic_ration: f32 = 0.95;
    let gravity: Vector3<f32> = Vector3::new(0.0, -9.8, 0.0);

    let resolution: Vector3<usize> = Vector3::new(32, 32, 32);
    let dim: Vector3<f32> = Vector3::new(resolution.x as f32, resolution.y as f32, resolution.z as f32);
    let h: f32 = 5.0 / dim.y;

    let delta_t: f32 = 1e-3;
    let num_particles: i32 = 10000;
    let radius = (3.0 * 500.0 / (16.0 * PI)).cbrt() * h;

    let mut grid = Grid::new(resolution, h);
    let params = Params::new(young_modulus, poisson_ration, hardening_coefficient, critical_compression, critical_stretch, flip_pic_ration);

    grid.create_sphere_uniform_particles(Vector3::new(grid.dim_x, grid.dim_y + 0.8, grid.dim_z) / 2.0, num_particles, radius);
    // grid.create_tower(num_particles);

    let model = Mat4::from_translation(vec3(grid.dim_x / 2.0, grid.dim_y / 2.0, grid.dim_z / 2.0));

    let mut collision_planes: Vec<Plane> = Vec::new();

    let ground_color = Srgba::new(150, 75, 0, 1);
    let origin = Vector3::new(-1.0 * grid.dim_x / 2.0, -1.0 * grid.dim_y / 2.0, -1.0 * grid.dim_z / 2.0);
    let axis_x = Vector3::new(grid.dim_x, 0.0, 0.0);
    let axis_y = Vector3::new(0.0, grid.dim_y, 0.0);
    let axis_z = Vector3::new(0.0, 0.0, grid.dim_z);
    let ground_rect = Plane::new(origin, axis_x, axis_z, 0.2, Vector3::zeros(), model, ground_color);
    collision_planes.push(ground_rect);

    let wedge_color = Srgba::new(173, 216, 230, 0);
    let corner = Vector3::new(0.0, -0.05 * grid.dim_y, -grid.dim_z / 2.0);
    let top_edge = Vector3::new(0.0, 0.0, 0.8 * grid.dim_z);
    let edge1 = Vector3::new(0.15 * grid.dim_x, -0.15 * grid.dim_y, 0.0);
    let edge2 = Vector3::new(-0.15 * grid.dim_x, -0.15 * grid.dim_y, 0.0);
    let wedge_rect1 = Plane::new(corner, top_edge, edge1, 0.2, Vector3::zeros(), model, wedge_color);
    let wedge_rect2 = Plane::new(corner, top_edge, edge2, 0.2, Vector3::zeros(), model, wedge_color);
    collision_planes.push(wedge_rect1);
    collision_planes.push(wedge_rect2);

    let mut collision_cubes: Vec<Cube> = Vec::new();

    // let cube_color = Srgba::new(13, 13, 13, 0);
    // let cube_origin = origin + Vector3::new(0.01 * grid.dim_x, 0.15 * grid.dim_y, 0.45 * grid.dim_z);
    // let cube_u = Vector3::new(0.1 * grid.dim_x, 0.0, 0.0);
    // let cube_v = Vector3::new(0.0, 0.1 * grid.dim_y, 0.0);
    // let cube_w = Vector3::new(0.0, 0.0, 0.1 * grid.dim_z);
    // let cube = Cube::new(cube_origin, cube_u, cube_v, cube_w, 0.2, Vector3::new(grid.dim_x * 5.0, 0.0, 0.0), model, cube_color);
    // collision_cubes.push(cube);

    let mut frame = 0;
    window.render_loop(move |mut frame_input| {
        println!("Frame {}", frame);

        camera.set_viewport(frame_input.viewport);
        // println!("Camera: {:?}", camera);
        control.handle_events(&mut camera, &mut frame_input.events);

        let start = std::time::Instant::now();

        for co in &mut collision_cubes {
            for plane in &mut co.sides {
                plane.update_position(delta_t);
            }
        }

        grid.simulate(delta_t, gravity, &params, &collision_planes, &collision_cubes);
        println!("Simulation took {} ms", start.elapsed().as_millis());

        let start = std::time::Instant::now();
        let mut spheres = Vec::new();
        for particle in &grid.all_particles {
            let sphere = particle.get_sphere_material(&context);
            spheres.push(sphere);
        }

        let mut planes = Vec::new();
        for plane in &collision_planes {
            let plane = plane.get_material(&context);
            planes.push(plane);
        }

        for cube in &collision_cubes {
            for plane in &cube.sides {
                let plane = plane.get_material(&context);
                planes.push(plane);
            }
        }

        let pixels = frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(
                &camera,
                spheres.into_iter().chain(planes.into_iter()),
                &[&light0, &light1],
            ).read_color();

        three_d_asset::io::save(
            &CpuTexture {
                data: TextureData::RgbaU8(pixels),
                width: 800,
                height: 600,
                ..Default::default()
            }.serialize(format!("frames/frame-{}.png", frame)).unwrap(),
        ).unwrap();

        println!("Rendering took {} ms", start.elapsed().as_millis());
        frame += 1;

        FrameOutput::default()
    });
}
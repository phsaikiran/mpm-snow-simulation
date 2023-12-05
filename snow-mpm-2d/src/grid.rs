use macroquad::color::Color;
use macroquad::prelude::{draw_circle, screen_height, screen_width};
use macroquad::shapes::draw_line;
use nalgebra::{DMatrix, Matrix2, Matrix4, Vector2};
use rayon::prelude::*;
use crate::params::Params;
use crate::particle::Particle;

#[derive(Clone, Copy, Debug, PartialEq)]
struct GridNode {
    mass: f64,
    vel: Vector2<f64>,
    vel_new: Vector2<f64>,
    active: bool,
}

impl GridNode {
    fn new() -> Self {
        GridNode {
            mass: 0.0,
            vel: Vector2::new(0.0, 0.0),
            vel_new: Vector2::new(0.0, 0.0),
            active: false,
        }
    }
}

pub struct Grid {
    cs: f64,
    na: f64,
    cc: usize,
    nodes: DMatrix<GridNode>,
    pub particles: Vec<Particle>,
    params: Params,
}

impl Grid {
    pub fn new(cc: usize, params: Params) -> Self {
        let cs = 1.0 / cc as f64;
        let na = cs * cs;
        let nodes = DMatrix::from_element(cc, cc, GridNode::new());
        Grid {
            cs,
            na,
            cc,
            nodes,
            particles: vec![],
            params,
        }
    }

    pub fn add_particle(&mut self, particle: Particle) {
        self.particles.push(particle);
    }

    pub fn reset_parameters(&mut self) {
        unsafe {
            self.nodes.data.as_vec_mut().par_iter_mut().for_each(|n| {
                n.mass = 0.0;
                n.vel = Vector2::new(0.0, 0.0);
                n.vel_new = Vector2::new(0.0, 0.0);
            });
        }

        self.particles.par_iter_mut().for_each(|p| {
            p.w = Matrix4::zeros();
            p.w_d_x = Matrix4::zeros();
            p.w_d_y = Matrix4::zeros();
        });
    }

    pub fn p2g_mass(&mut self) {
        for p in self.particles.iter_mut() {
            let grid_index = Vector2::new(
                (p.pos.x / self.cs).floor() as usize,
                (p.pos.y / self.cs).floor() as usize,
            );
            // println!("grid_index: {:?}", grid_index);
            for i in 0..4 {
                for j in 0..4 {
                    let distance = Vector2::new(
                        p.pos.x / self.cs - (grid_index.x + i - 1) as f64,
                        p.pos.y / self.cs - (grid_index.y + j - 1) as f64,
                    );
                    let index_i = grid_index.x + i;
                    let index_j = grid_index.y + j;
                    let wy = Self::n(distance.y);
                    let dy = Self::n_x(distance.y);
                    let wx = Self::n(distance.x);
                    let dx = Self::n_x(distance.x);
                    let weight = wx * wy;
                    p.w[(i, j)] = weight;
                    p.w_d_x[(i, j)] = (dx * wy) / self.cs;
                    p.w_d_y[(i, j)] = (wx * dy) / self.cs;
                    self.nodes[(index_i, index_j)].mass += weight * p.mass;
                }
            }
        }
    }

    pub fn p2g_velocity(&mut self) {
        for p in self.particles.iter_mut() {
            let grid_index = Vector2::new(
                (p.pos.x / self.cs).floor() as usize,
                (p.pos.y / self.cs).floor() as usize,
            );
            for i in 0..4 {
                for j in 0..4 {
                    let index_i = grid_index.x + i;
                    let index_j = grid_index.y + j;
                    let w = p.w[(i, j)];
                    if w > self.params.bspline_epsilon {
                        self.nodes[(index_i, index_j)].vel += p.vel * (w * p.mass);
                        self.nodes[(index_i, index_j)].active = true;
                    }
                }
            }
        }
        unsafe {
            self.nodes.data.as_vec_mut().par_iter_mut()
                .filter(|n| n.active)
                .for_each(|n| n.vel /= n.mass);
        }
    }

    pub fn calculate_volumes(&mut self) {
        self.particles.par_iter_mut().for_each(|p| {
            let grid_index = Vector2::new(
                (p.pos.x / self.cs).floor() as usize,
                (p.pos.y / self.cs).floor() as usize,
            );
            let mut density = 0.0;
            for i in 0..4 {
                for j in 0..4 {
                    let index_i = grid_index.x + i;
                    let index_j = grid_index.y + j;
                    let w = p.w[(i, j)];
                    if w > self.params.bspline_epsilon {
                        density += w * self.nodes[(index_i, index_j)].mass;
                    }
                }
            }
            density /= self.na;
            p.vol = p.mass / density;
            // println!("density, volume: {}, {}", density, p.volume);
        });
    }

    pub fn compute_grid_forces(&mut self) {
        for p in self.particles.iter() {
            let jp = p.e_p.determinant();
            let je = p.e_d.determinant();
            let svd_result = p.e_d.svd(true, true);
            let w = svd_result.u.unwrap();
            let v_t = svd_result.v_t.unwrap();
            let re = w * v_t;
            // println!("JP, JE, W, V, RE, def_elastic: {}, {}, {:?}, {:?}, {:?} {:?}", jp, je, w, v, re, p.def_elastic);
            let mu = self.params.mu_0 * (self.params.hardening_coefficient * (1.0 - jp)).exp();
            let lambda = self.params.lambda_0 * (self.params.hardening_coefficient * (1.0 - jp)).exp();
            let sigma = 2.0 * mu / jp * (p.e_d - re) * p.e_d.transpose() + lambda / jp * (je - 1.0) * je * Matrix2::identity();
            let jn = (p.e_d * p.e_p).determinant();
            let v_n = jn * p.vol;
            let energy = v_n * sigma;
            let grid_index = Vector2::new(
                (p.pos.x / self.cs).floor() as usize,
                (p.pos.y / self.cs).floor() as usize,
            );
            for i in 0..4 {
                for j in 0..4 {
                    let index_i = grid_index.x + i;
                    let index_j = grid_index.y + j;
                    let w = p.w[(i, j)];
                    if w > self.params.bspline_epsilon {
                        self.nodes[(index_i, index_j)].vel_new -= energy * Vector2::new(p.w_d_x[(i, j)], p.w_d_y[(i, j)]);
                        // println!("velocity_new: {:?}", self.nodes[(index_i, index_j)].velocity_new);
                    }
                }
            }
        }
    }

    pub fn update_grid_velocities(&mut self) {
        unsafe {
            self.nodes.data.as_vec_mut().par_iter_mut()
                .filter(|n| n.active)
                .for_each(|n| n.vel_new = n.vel + self.params.dt * (self.params.gravity + n.vel_new / n.mass));
        }
    }

    pub fn collision_grid(&mut self) {
        unsafe {
            self.nodes.data.as_vec_mut().par_iter_mut().enumerate()
                .filter(|(_, n)| n.active)
                .for_each(|(index, n)| {
                    let j = index / self.cc;
                    let i = index % self.cc;
                    let new_pos = n.vel_new * (self.params.dt / self.cs) + Vector2::new(i as f64, j as f64);
                    if new_pos.x < self.params.bspline_radius || new_pos.x > self.cc as f64 - self.params.bspline_radius - 1.0 {
                        n.vel_new.x = 0.0;
                        n.vel_new.y *= 0.9;
                    }
                    if new_pos.y < self.params.bspline_radius || new_pos.y > self.cc as f64 - self.params.bspline_radius - 1.0 {
                        n.vel_new.y = 0.0;
                        n.vel_new.x *= 0.9;
                    }
                });
        }
    }

    pub fn update_velocity(&mut self) {
        self.particles.par_iter_mut().for_each(|p| {
            let mut pic = Vector2::new(0.0, 0.0);
            let mut flip = p.vel;
            p.vel_d = Matrix2::zeros();
            let grid_index = Vector2::new(
                (p.pos.x / self.cs).floor() as usize,
                (p.pos.y / self.cs).floor() as usize,
            );
            for i in 0..4 {
                for j in 0..4 {
                    let index_i = grid_index.x + i;
                    let index_j = grid_index.y + j;
                    let w = p.w[(i, j)];
                    if w > self.params.bspline_epsilon {
                        pic += self.nodes[(index_i, index_j)].vel_new * w;
                        flip += (self.nodes[(index_i, index_j)].vel_new - self.nodes[(index_i, index_j)].vel) * w;
                        p.vel_d += self.nodes[(index_i, index_j)].vel_new * Vector2::new(p.w_d_x[(i, j)], p.w_d_y[(i, j)]).transpose();
                    }
                }
            }
            p.vel = flip * 0.95 + pic * (1.0 - 0.95);
        });
    }

    pub fn update_deformation_gradient(&mut self) {
        self.particles.par_iter_mut().for_each(|p| {
            p.vel_d = Matrix2::identity() + self.params.dt * p.vel_d;
            p.e_d = p.vel_d * p.e_d;
            // println!("def_elastic: {:?}", p.def_elastic);
            let f_all = p.e_d * p.e_p;
            let svd_result = p.e_d.svd(true, true);
            let w = svd_result.u.unwrap();
            let v_t = svd_result.v_t.unwrap();
            let mut e = Matrix2::from_diagonal(&svd_result.singular_values);
            // println!("w, v, e: {:?}, {:?}, {:?}", w, v, e);
            for i in 0..2 {
                if e[(i, i)] < self.params.critical_compression {
                    e[(i, i)] = self.params.critical_compression;
                } else if e[(i, i)] > self.params.critical_stretch {
                    e[(i, i)] = self.params.critical_stretch;
                }
            }
            p.e_p = v_t.transpose() * e.try_inverse().unwrap() * w.transpose() * f_all;
            p.e_d = w * e * v_t;
            // println!("def_elastic, def_plastic: {:?}, {:?}", p.def_elastic, p.def_plastic);
        });
    }

    pub fn update_particle_positions(&mut self) {
        self.particles.par_iter_mut().for_each(|p| {
            p.pos += self.params.dt * p.vel;
        });
    }

    fn n(x: f64) -> f64 {
        let x = x.abs();
        let x2 = x * x;
        let x3 = x2 * x;
        let w: f64;
        if x < 1.0 {
            w = 1.0 / 2.0 * x3 - x2 + 2.0 / 3.0
        } else if x < 2.0 {
            w = -1.0 / 6.0 * x3 + x2 - 2.0 * x + 4.0 / 3.0
        } else {
            return 0.0;
        }

        if w < 1e-4 {
            0.0
        } else {
            w
        }
    }

    fn n_x(x: f64) -> f64 {
        let abs_x = x.abs();
        if abs_x < 1.0 {
            3.0 / 2.0 * x * abs_x - 2.0 * x
        } else if abs_x < 2.0 {
            -1.0 / 2.0 * x * abs_x + 2.0 * x - 2.0 * x.signum()
        } else {
            0.0
        }
    }

    pub fn draw(&self) {
        let mut count = 0;
        for node in self.nodes.iter() {
            let index_i = count % self.cc;
            let index_j = count / self.cc;
            let x = screen_width() / 2.0 + (screen_width() / 2.0) * index_i as f32 / self.cc as f32;
            let y = (screen_height() / 2.0) * index_j as f32 / self.cc as f32;
            // if node.mass > 0.0 {
            let mass = node.mass * 100.0 / 6.0;
            let mass = if mass > 1.0 { 1.0 } else { mass };
            let mass = if mass < 0.1 { 0.1 } else { mass };
            let mass = mass as f32;
            let color = Color::new(mass, mass, mass, 1.0);
            // cap color at 0.5
            // let color = if color.r < 0.5 { Color::new(0.5, 0.5, 0.5, 1.0) } else { color };
            draw_circle(x, y, mass * 10.0, color);
            // println!("vel: {:?}", node.vel);

            // let vel = 10.0 * node.vel / 5.0;
            // draw_line(x, y, x + vel.x as f32, y + vel.y as f32, 1.0, Color::new(1.0, 0.0, 0.0, 1.0));
            // }

            count += 1;
        }

        count = 0;
        for node in self.nodes.iter() {
            let index_i = count % self.cc;
            let index_j = count / self.cc;
            let x = screen_width() / 2.0 + (screen_width() / 2.0) * index_i as f32 / self.cc as f32;
            let y = screen_height() / 2.0 + (screen_height() / 2.0) * index_j as f32 / self.cc as f32;

            let vel = 10.0 * node.vel / 5.0;
            draw_line(x, y, x + vel.x as f32, y + vel.y as f32, 1.0, Color::new(1.0, 0.0, 0.0, 1.0));

            count += 1;
        }
    }
}
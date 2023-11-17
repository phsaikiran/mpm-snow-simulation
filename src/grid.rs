use nalgebra::{DMatrix, Matrix2, Matrix4, Vector2};
use crate::params::{BSPLINE_EPSILON, BSPLINE_RADIUS, CRIT_COMPRESS, CRIT_STRETCH, DT, GRAVITY, HARDENING, LAMBDA, MU};
use crate::particle::Particle;

#[derive(Clone, Copy, Debug, PartialEq)]
struct GridNode {
    mass: f64,
    velocity: Vector2<f64>,
    velocity_new: Vector2<f64>,

    active: bool,
}

impl GridNode {
    fn new() -> Self {
        GridNode {
            mass: 0.0,
            velocity: Vector2::new(0.0, 0.0),
            velocity_new: Vector2::new(0.0, 0.0),
            active: false,
        }
    }
}

pub struct Grid {
    // Size of each cell
    cell_size: f64,
    // Area of each cell
    node_area: f64,
    cell_count: usize,
    nodes: DMatrix<GridNode>,
    pub particles: Vec<Particle>,
}

impl Grid {
    pub fn new(cell_count: usize) -> Self {
        let cell_size = 1.0 / cell_count as f64;
        let node_area = cell_size * cell_size;
        let nodes = DMatrix::from_element(cell_count, cell_count, GridNode::new());
        Grid {
            cell_size,
            node_area,
            cell_count,
            nodes,
            particles: vec![],
        }
    }

    pub fn add_particle(&mut self, particle: Particle) {
        self.particles.push(particle);
    }

    pub fn reset_parameters(&mut self) {
        for i in 0..self.cell_count {
            for j in 0..self.cell_count {
                self.nodes[(i, j)].mass = 0.0;
                self.nodes[(i, j)].velocity = Vector2::new(0.0, 0.0);
                self.nodes[(i, j)].velocity_new = Vector2::new(0.0, 0.0);
                self.nodes[(i, j)].active = false;
            }
        }
        for p in self.particles.iter_mut() {
            p.weights = Matrix4::zeros();
            p.weight_gradients_x = Matrix4::zeros();
            p.weight_gradients_y = Matrix4::zeros();
            // println!("position, velocity, mass: {:?}, {:?}, {}", p.position, p.velocity, p.mass)
        }
    }

    // Rasterize particle mass to the grid
    pub fn p2g_mass(&mut self) {
        for p in self.particles.iter_mut() {
            let grid_index = Vector2::new(
                (p.position.x / self.cell_size).floor() as usize,
                (p.position.y / self.cell_size).floor() as usize,
            );
            // println!("grid_index: {:?}", grid_index);
            for i in 0..4 {
                for j in 0..4 {
                    let distance = Vector2::new(
                        p.position.x / self.cell_size - (grid_index.x + i - 1) as f64,
                        p.position.y / self.cell_size - (grid_index.y + j - 1) as f64,
                    );
                    let index_i = grid_index.x + i;
                    let index_j = grid_index.y + j;
                    let wy = Self::n(distance.y);
                    let dy = Self::n_x(distance.y);
                    let wx = Self::n(distance.x);
                    let dx = Self::n_x(distance.x);
                    let weight = wx * wy;
                    // println!("distance, index_i, index_j, wy, dy, wx, dx, weight: {:?}, {}, {}, {}, {}, {}, {}, {}", distance, index_i, index_j, wy, dy, wx, dx, weight);
                    p.weights[(i, j)] = weight;
                    p.weight_gradients_x[(i, j)] = (dx * wy) / self.cell_size;
                    p.weight_gradients_y[(i, j)] = (wx * dy) / self.cell_size;
                    self.nodes[(index_i, index_j)].mass += weight * p.mass;
                }
            }
        }
    }

    // Rasterize particle velocity to the grid
    pub fn p2g_velocity(&mut self) {
        for p in self.particles.iter_mut() {
            let grid_index = Vector2::new(
                (p.position.x / self.cell_size).floor() as usize,
                (p.position.y / self.cell_size).floor() as usize,
            );
            for i in 0..4 {
                for j in 0..4 {
                    let index_i = (grid_index.x + i) as usize;
                    let index_j = (grid_index.y + j) as usize;
                    let w = p.weights[(i, j)];
                    if w > BSPLINE_EPSILON {
                        self.nodes[(index_i, index_j)].velocity += p.velocity * (w * p.mass);
                        self.nodes[(index_i, index_j)].active = true;
                    }
                }
            }
        }
        for i in 0..self.cell_count {
            for j in 0..self.cell_count {
                if self.nodes[(i, j)].active {
                    let velocity_scale = self.nodes[(i, j)].velocity / self.nodes[(i, j)].mass;
                    self.nodes[(i, j)].velocity = velocity_scale;
                    // println!("mass, velocity: {}, {:?}", self.nodes[(i, j)].mass, self.nodes[(i, j)].velocity);
                }
            }
        }
    }

    // Calculate volumes
    pub fn calculate_volumes(&mut self) {
        for p in self.particles.iter_mut() {
            let grid_index = Vector2::new(
                (p.position.x / self.cell_size).floor() as usize,
                (p.position.y / self.cell_size).floor() as usize,
            );
            let mut density = 0.0;
            for i in 0..4 {
                for j in 0..4 {
                    let index_i = grid_index.x + i;
                    let index_j = grid_index.y + j;
                    let w = p.weights[(i, j)];
                    if w > BSPLINE_EPSILON {
                        density += w * self.nodes[(index_i, index_j)].mass;
                    }
                }
            }
            density /= self.node_area;
            p.volume = p.mass / density;
            // println!("density, volume: {}, {}", density, p.volume);
        }
    }

    pub fn compute_grid_forces(&mut self) {
        for p in self.particles.iter_mut() {
            let jp = p.def_plastic.determinant();
            let je = p.def_elastic.determinant();
            let svd_result = p.def_elastic.svd(true, true);
            let w = svd_result.u.unwrap();
            let v_t = svd_result.v_t.unwrap();
            let re = w * v_t;
            // println!("JP, JE, W, V, RE, def_elastic: {}, {}, {:?}, {:?}, {:?} {:?}", jp, je, w, v, re, p.def_elastic);
            let mu = MU * (HARDENING * (1.0 - jp)).exp();
            let lambda = LAMBDA * (HARDENING * (1.0 - jp)).exp();
            let sigma = 2.0 * mu / jp * (p.def_elastic - re) * p.def_elastic.transpose() + lambda / jp * (je - 1.0) * je * Matrix2::identity();
            let jn = (p.def_elastic * p.def_plastic).determinant();
            let v_n = jn * p.volume;
            let energy = v_n * sigma;
            let grid_index = Vector2::new(
                (p.position.x / self.cell_size).floor() as usize,
                (p.position.y / self.cell_size).floor() as usize,
            );
            for i in 0..4 {
                for j in 0..4 {
                    let index_i = grid_index.x + i;
                    let index_j = grid_index.y + j;
                    let w = p.weights[(i, j)];
                    if w > BSPLINE_EPSILON {
                        self.nodes[(index_i, index_j)].velocity_new -= energy * Vector2::new(p.weight_gradients_x[(i, j)], p.weight_gradients_y[(i, j)]);
                        // println!("velocity_new: {:?}", self.nodes[(index_i, index_j)].velocity_new);
                    }
                }
            }
        }
    }

    pub fn update_grid_velocities(&mut self) {
        for i in 0..self.cell_count {
            for j in 0..self.cell_count {
                if self.nodes[(i, j)].active {
                    self.nodes[(i, j)].velocity_new = self.nodes[(i, j)].velocity + DT * (GRAVITY + self.nodes[(i, j)].velocity_new / self.nodes[(i, j)].mass);
                    // println!("velocity_new: {:?}", self.nodes[(i, j)].velocity_new);
                }
            }
        }
    }

    pub fn collision_grid(&mut self) {
        for i in 0..self.cell_count {
            for j in 0..self.cell_count {
                if self.nodes[(i, j)].active {
                    let new_pos = self.nodes[(i, j)].velocity_new * (DT / self.cell_size) + Vector2::new(i as f64, j as f64);
                    if new_pos.x < BSPLINE_RADIUS || new_pos.x > self.cell_count as f64 - BSPLINE_RADIUS - 1.0 {
                        self.nodes[(i, j)].velocity_new.x = 0.0;
                        self.nodes[(i, j)].velocity_new.y *= 0.9;
                    }
                    if new_pos.y < BSPLINE_RADIUS || new_pos.y > self.cell_count as f64 - BSPLINE_RADIUS - 1.0 {
                        self.nodes[(i, j)].velocity_new.y = 0.0;
                        self.nodes[(i, j)].velocity_new.x *= 0.9;
                    }
                    // println!("new_pos, velocity_new: {:?}, {:?}", new_pos, self.nodes[(i, j)].velocity_new);
                }
            }
        }
    }

    pub fn update_velocity(&mut self) {
        for p in self.particles.iter_mut() {
            let mut pic = Vector2::new(0.0, 0.0);
            let mut flip = p.velocity;
            p.velocity_gradient = Matrix2::zeros();
            let grid_index = Vector2::new(
                (p.position.x / self.cell_size).floor() as usize,
                (p.position.y / self.cell_size).floor() as usize,
            );
            for i in 0..4 {
                for j in 0..4 {
                    let index_i = grid_index.x + i;
                    let index_j = grid_index.y + j;
                    let w = p.weights[(i, j)];
                    if w > BSPLINE_EPSILON {
                        pic += self.nodes[(index_i, index_j)].velocity_new * w;
                        flip += (self.nodes[(index_i, index_j)].velocity_new - self.nodes[(index_i, index_j)].velocity) * w;
                        p.velocity_gradient += self.nodes[(index_i, index_j)].velocity_new * Vector2::new(p.weight_gradients_x[(i, j)], p.weight_gradients_y[(i, j)]).transpose();
                    }
                }
            }
            p.velocity = flip * 0.95 + pic * (1.0 - 0.95);
            // println!("velocity: {:?}", p.velocity);
        }
    }

    // pub fn collision_particles(&mut self) {
    //     for p in self.particles.iter_mut() {
    //         let grid_position = p.position / self.cell_size;
    //         let new_pos = grid_position + (DT * (p.velocity / self.cell_size));
    //         if new_pos.x < BSPLINE_RADIUS || new_pos.x > self.cell_count as f64 - BSPLINE_RADIUS {
    //             p.velocity.x = -0.9 * p.velocity.x;
    //         }
    //         if new_pos.y < BSPLINE_RADIUS || new_pos.y > self.cell_count as f64 - BSPLINE_RADIUS {
    //             p.velocity.y = -0.9 * p.velocity.y;
    //         }
    //     }
    // }

    pub fn update_deformation_gradient(&mut self) {
        for p in self.particles.iter_mut() {
            p.velocity_gradient = Matrix2::identity() + DT * p.velocity_gradient;
            p.def_elastic = p.velocity_gradient * p.def_elastic;
            // println!("def_elastic: {:?}", p.def_elastic);
            let f_all = p.def_elastic * p.def_plastic;
            let svd_result = p.def_elastic.svd(true, true);
            let w = svd_result.u.unwrap();
            let v_t = svd_result.v_t.unwrap();
            let mut e = Matrix2::from_diagonal(&svd_result.singular_values);
            // println!("w, v, e: {:?}, {:?}, {:?}", w, v, e);
            for i in 0..2 {
                if e[(i, i)] < CRIT_COMPRESS {
                    e[(i, i)] = CRIT_COMPRESS;
                } else if e[(i, i)] > CRIT_STRETCH {
                    e[(i, i)] = CRIT_STRETCH;
                }
            }
            p.def_plastic = v_t.transpose() * e.try_inverse().unwrap() * w.transpose() * f_all;
            p.def_elastic = w * e * v_t;
            // println!("def_elastic, def_plastic: {:?}, {:?}", p.def_elastic, p.def_plastic);
        }
    }

    pub fn update_particle_positions(&mut self) {
        for p in self.particles.iter_mut() {
            p.position += DT * p.velocity;
        }
    }

    // pub fn print_grid(&self) {
    //     for i in 0..self.cell_count {
    //         for j in 0..self.cell_count {
    //             print!("{} ", self.nodes[(i, j)].mass);
    //         }
    //         println!();
    //     }
    // }

    // one-dimensional cubic B-splines
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

        if w < BSPLINE_EPSILON {
            0.0
        } else {
            w
        }
    }

    // Partial derivative of N(x) with respect to x
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
}
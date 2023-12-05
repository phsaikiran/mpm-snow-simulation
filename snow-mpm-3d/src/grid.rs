use std::collections::HashSet;
use nalgebra::{clamp, Matrix3, Vector3, SVD};
use rand::Rng;
use crate::helpers::Helpers;
use crate::params::Params;
use crate::particle::Particle;
use crate::plane::{Cube, Plane};

#[derive(Clone)]
struct GridNode {
    index: Vector3<f32>,
    mass: f32,
    vel: Vector3<f32>,
    next_vel: Vector3<f32>,
    force: Vector3<f32>,
}

impl GridNode {
    fn new() -> Self {
        GridNode {
            index: Vector3::new(0.0, 0.0, 0.0),
            mass: 0.0,
            vel: Vector3::new(0.0, 0.0, 0.0),
            next_vel: Vector3::new(0.0, 0.0, 0.0),
            force: Vector3::new(0.0, 0.0, 0.0),
        }
    }

    fn reset(&mut self) {
        self.mass = 0.0;
        self.vel = Vector3::new(0.0, 0.0, 0.0);
        self.next_vel = Vector3::new(0.0, 0.0, 0.0);
        self.force = Vector3::new(0.0, 0.0, 0.0);
    }
}

pub struct Grid {
    pub dim_x: f32,
    pub dim_y: f32,
    pub dim_z: f32,
    h: f32,
    nodes: Vec<Vec<Vec<GridNode>>>,
    node_init: Vec<Vec<Vec<bool>>>,
    pub all_particles: Vec<Particle>,
    nodes_in_use: HashSet<Vector3<usize>>,
    first_step: bool,
}

impl Grid {
    pub fn new(resolution: Vector3<usize>, h: f32) -> Self {
        let dim_x = resolution.x as f32 * h;
        let dim_y = resolution.y as f32 * h;
        let dim_z = resolution.z as f32 * h;

        let nodes = vec![vec![vec![GridNode::new(); resolution.z]; resolution.y]; resolution.x];
        let node_init = vec![vec![vec![false; resolution.z]; resolution.y]; resolution.x];

        Grid {
            dim_x,
            dim_y,
            dim_z,
            h,
            nodes,
            node_init,
            all_particles: Vec::new(),
            nodes_in_use: HashSet::new(),
            first_step: true,
        }
    }

    fn reset_grid(&mut self) {
        for node in self.nodes_in_use.iter() {
            let node = &mut self.nodes[node.x][node.y][node.z];
            node.mass = 0.0;
            node.vel = Vector3::new(0.0, 0.0, 0.0);
            node.next_vel = Vector3::new(0.0, 0.0, 0.0);
            node.force = Vector3::new(0.0, 0.0, 0.0);
        }

        for particle in self.all_particles.iter_mut() {
            particle.pos.x = clamp(particle.pos.x, 0.0, self.dim_x - 1e-5);
            particle.pos.y = clamp(particle.pos.y, 0.0, self.dim_y - 1e-5);
            particle.pos.z = clamp(particle.pos.z, 0.0, self.dim_z - 1e-5);

            particle.compute_neighborhood_bounds();
            particle.compute_b_spline_d();
            for dest_i in particle.i1..particle.i2 {
                for dest_j in particle.j1..particle.j2 {
                    for dest_k in particle.k1..particle.k2 {
                        let node = &mut self.nodes[dest_i][dest_j][dest_k];
                        let node_init = &mut self.node_init[dest_i][dest_j][dest_k];

                        if !*node_init {
                            node.reset();
                            node.index = Vector3::new(dest_i as f32, dest_j as f32, dest_k as f32);
                        }
                        self.nodes_in_use.insert(Vector3::new(dest_i, dest_j, dest_k));
                    }
                }
            }
        }
    }

    fn particle_to_grid(&mut self) {
        for particle in &self.all_particles {
            for dest_i in particle.i1..particle.i2 {
                for dest_j in particle.j1..particle.j2 {
                    for dest_k in particle.k1..particle.k2 {
                        let weight = particle.b_spline_at(dest_i, dest_j, dest_k);
                        let node = &mut self.nodes[dest_i][dest_j][dest_k];
                        node.mass += weight * particle.mass;
                        node.vel += weight * particle.mass * particle.vel;
                    }
                }
            }
        }

        for node_index in &self.nodes_in_use {
            let node = &mut self.nodes[node_index.x][node_index.y][node_index.z];
            if node.mass > 0.0 {
                node.vel /= node.mass;
            }
        }
    }

    fn compute_particle_volumes(&mut self) {
        let h3 = self.h.powi(3);

        for particle in self.all_particles.iter_mut() {
            let mut density = 0.0;

            for dest_i in particle.i1..particle.i2 {
                for dest_j in particle.j1..particle.j2 {
                    for dest_k in particle.k1..particle.k2 {
                        let weight = particle.b_spline_at(dest_i, dest_j, dest_k);
                        density += weight * self.nodes[dest_i][dest_j][dest_k].mass;
                    }
                }
            }

            density /= h3;
            particle.vol = particle.mass / density;
        }
    }

    fn compute_f_hat_ep(&mut self, delta_t: f32) {
        for particle in self.all_particles.iter_mut() {
            let mut sum = Matrix3::zeros();

            for dest_i in particle.i1..particle.i2 {
                for dest_j in particle.j1..particle.j2 {
                    for dest_k in particle.k1..particle.k2 {
                        let weight_grad = particle.b_spline_grad_at(dest_i, dest_j, dest_k);
                        let velocity = self.nodes[dest_i][dest_j][dest_k].vel;
                        sum += delta_t * Helpers::outer_product(velocity, weight_grad);
                    }
                }
            }

            let identity = Matrix3::identity();
            particle.f_ep_d = (identity + sum) * particle.def_e_d;
        }
    }

    fn compute_grid_forces(&mut self, mu_0: f32, lambda_0: f32, xi: f32) {
        for particle in &self.all_particles {
            let volume = particle.vol;
            let sigma_p: Matrix3<f32> = Helpers::psi_derivative(mu_0, lambda_0, xi, particle) * particle.def_e_d.transpose();

            let neg_force_unweighted = volume * sigma_p;

            for dest_i in particle.i1..particle.i2 {
                for dest_j in particle.j1..particle.j2 {
                    for dest_k in particle.k1..particle.k2 {
                        let weight_grad = particle.b_spline_grad_at(dest_i, dest_j, dest_k);
                        self.nodes[dest_i][dest_j][dest_k].force -= neg_force_unweighted * weight_grad;
                    }
                }
            }
        }
    }

    fn compute_grid_velocities(&mut self, delta_t: f32, collision_objects: &Vec<Plane>, collision_cubes: &Vec<Cube>) {
        for node_index in &self.nodes_in_use {
            let node = &mut self.nodes[node_index.x][node_index.y][node_index.z];
            node.next_vel = node.vel;

            if node.mass > 0.0 {
                node.next_vel += node.force * delta_t / node.mass;
            }

            let position = node.index * self.h;
            for co in collision_objects {
                node.next_vel = co.collide(position, node.next_vel, delta_t);
            }
            for cube in collision_cubes {
                for co in cube.sides.iter() {
                    node.next_vel = co.collide(position, node.next_vel, delta_t);
                }
            }
        }
    }

    fn update_deformation_gradients(&mut self, theta_c: f32, theta_s: f32, delta_t: f32) {
        for particle in &mut self.all_particles {
            let mut grad_vp = Matrix3::zeros();

            for dest_i in particle.i1..particle.i2 {
                for dest_j in particle.j1..particle.j2 {
                    for dest_k in particle.k1..particle.k2 {
                        let weight_grad = particle.b_spline_grad_at(dest_i, dest_j, dest_k);
                        let velocity = self.nodes[dest_i][dest_j][dest_k].next_vel;
                        grad_vp += Helpers::outer_product(velocity, weight_grad);
                    }
                }
            }

            let identity = Matrix3::identity();
            let dgrad_e_next = (identity + delta_t * grad_vp) * particle.def_e_d;
            let f_next = dgrad_e_next * particle.def_p_d;

            let svd_result = SVD::new(dgrad_e_next, true, true);
            let u = svd_result.u.unwrap();
            let v_t = svd_result.v_t.unwrap();
            let s_hat_vec = svd_result.singular_values;

            let s_vec_x = clamp(s_hat_vec.x, 1.0 - theta_c, 1.0 + theta_s);
            let s_vec_y = clamp(s_hat_vec.y, 1.0 - theta_c, 1.0 + theta_s);
            let s_vec_z = clamp(s_hat_vec.z, 1.0 - theta_c, 1.0 + theta_s);
            let s_vec = Vector3::new(s_vec_x, s_vec_y, s_vec_z);
            let s = Matrix3::new(
                s_vec.x, 0.0, 0.0,
                0.0, s_vec.y, 0.0,
                0.0, 0.0, s_vec.z,
            );
            let s_inv = Matrix3::new(
                1.0 / s_vec.x, 0.0, 0.0,
                0.0, 1.0 / s_vec.y, 0.0,
                0.0, 0.0, 1.0 / s_vec.z,
            );

            particle.def_e_d = u * s * v_t;
            particle.def_p_d = v_t.transpose() * s_inv * u.transpose() * f_next;
        }
    }

    fn update_particle_velocities(&mut self, alpha: f32) {
        for particle in &mut self.all_particles {
            let mut v_pic = Vector3::new(0.0, 0.0, 0.0);
            let mut v_flip = particle.vel;

            for dest_i in particle.i1..particle.i2 {
                for dest_j in particle.j1..particle.j2 {
                    for dest_k in particle.k1..particle.k2 {
                        let dest = &self.nodes[dest_i][dest_j][dest_k];
                        let weight = particle.b_spline_at(dest_i, dest_j, dest_k);
                        v_pic += dest.next_vel * weight;
                        v_flip += (dest.next_vel - dest.vel) * weight;
                    }
                }
            }

            particle.vel = (1.0 - alpha) * v_pic + alpha * v_flip;
        }
    }

    fn compute_particle_collisions(&mut self, delta_t: f32, collision_objects: &Vec<Plane>, collision_cubes: &Vec<Cube>) {
        for particle in &mut self.all_particles {
            for co in collision_objects {
                particle.vel = co.collide(particle.pos, particle.vel, delta_t);
            }
            for cube in collision_cubes {
                for co in cube.sides.iter() {
                    particle.vel = co.collide(particle.pos, particle.vel, delta_t);
                }
            }
        }
    }

    fn update_particle_positions(&mut self, delta_t: f32) {
        for particle in &mut self.all_particles {
            particle.pos += particle.vel * delta_t;
        }
    }

    pub fn simulate(&mut self, delta_t: f32, gravity: Vector3<f32>, params: &Params, collision_objects: &Vec<Plane>, collision_cubes: &Vec<Cube>) {
        self.reset_grid();
        self.particle_to_grid();
        if self.first_step {
            self.compute_particle_volumes();
            self.first_step = true;
        }
        self.compute_f_hat_ep(delta_t);
        self.compute_grid_forces(params.mu_0, params.lambda_0, params.hardening_coefficient);

        self.compute_grid_velocities(delta_t, collision_objects, collision_cubes);
        self.update_deformation_gradients(params.critical_compression, params.critical_stretch, delta_t);
        self.update_particle_velocities(params.flip_pic_ration);

        for particle in &mut self.all_particles {
            particle.vel += gravity * delta_t;
        }

        self.compute_particle_collisions(delta_t, collision_objects, collision_cubes);

        self.update_particle_positions(delta_t);
    }

    pub fn create_sphere_uniform_particles(&mut self, center: Vector3<f32>, num_particles: i32, radius: f32, vel: Vector3<f32>) {
        let mut rng = rand::thread_rng();

        for _ in 0..num_particles {
            let random_offset = Vector3::new(
                rng.gen_range(-radius..radius),
                rng.gen_range(-radius..radius),
                rng.gen_range(-radius..radius),
            );
            let position = Vector3::from(center + random_offset);
            if (position - center).norm() > radius {
                continue;
            }
            let particle = Particle::new(position, 1.0, Vector3::new((self.dim_x / self.h) as usize, (self.dim_y / self.h) as usize, (self.dim_z / self.h) as usize), self.h, vel);
            self.all_particles.push(particle);
        }

        self.reset_grid();
    }
    pub fn create_snowman(&mut self, num_particles: i32, speed: f32) {
        let radius1 = 0.8;
        self.create_sphere_uniform_particles(Vector3::new(self.dim_x / 2.0, radius1, self.dim_z / 2.0), num_particles * 2, radius1, Vector3::zeros());
        let radius2 = 0.4;
        self.create_sphere_uniform_particles(Vector3::new(self.dim_x / 2.0, 2.0 * radius1 + radius2 - 0.1, self.dim_z / 2.0), num_particles / 2, radius2, Vector3::zeros());
        let radius3 = 0.2;
        self.create_sphere_uniform_particles(Vector3::new(self.dim_x / 2.0, 2.0 * radius1 + 2.0 * radius2 + radius3 - 0.2, self.dim_z / 2.0), num_particles / 4, radius3, Vector3::zeros());

        let radius3 = 0.2;
        self.create_sphere_uniform_particles(Vector3::new(radius3, radius1, self.dim_z / 2.0), num_particles / 4, radius3, Vector3::new(speed, 0.0, 0.0));

        // let radius = 0.1;
        // self.create_sphere_uniform_particles(Vector3::new(self.dim_x / 2.0, radius1, self.dim_z / 2.0 + radius1), 5, radius, Vector3::zeros(), Srgba::new(0, 0, 0, 255));
        // self.create_sphere_uniform_particles(Vector3::new(self.dim_x / 2.0, 2.0 * radius1 + radius2 - 0.1, self.dim_z / 2.0 + radius2), 5, radius, Vector3::zeros(), Srgba::new(0, 0, 0, 255));
        self.reset_grid();
    }
}
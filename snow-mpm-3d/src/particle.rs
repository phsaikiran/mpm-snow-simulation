use nalgebra::{Matrix3, Vector3};
use three_d::{ColorMaterial, Context, CpuMaterial, CpuMesh, Gm, Mat4, Mesh, PhysicalMaterial, Srgba};
use crate::helpers::Helpers;

#[derive(Clone)]
pub struct Particle {
    pub pos: Vector3<f32>,
    pub vel: Vector3<f32>,
    resolution: Vector3<usize>,
    pub mass: f32,
    pub vol: f32,
    h: f32,
    pub def_e_d: Matrix3<f32>,
    pub def_p_d: Matrix3<f32>,
    pub f_ep_d: Matrix3<f32>,
    pub i1: usize,
    pub i2: usize,
    pub j1: usize,
    pub j2: usize,
    pub k1: usize,
    pub k2: usize,
    w: [[[f32; 4]; 4]; 4],
    w_d: [[[Vector3<f32>; 4]; 4]; 4],
}

impl Particle {
    pub fn new(position: Vector3<f32>, mass: f32, resolution: Vector3<usize>, h: f32, vel: Vector3<f32>) -> Self {
        Particle {
            pos: position,
            mass,
            vel,
            resolution,
            h,
            vol: 0.0,
            def_e_d: Matrix3::identity(),
            def_p_d: Matrix3::identity(),
            f_ep_d: Matrix3::identity(),
            i1: 0,
            i2: 0,
            j1: 0,
            j2: 0,
            k1: 0,
            k2: 0,
            w: [[[0.0; 4]; 4]; 4],
            w_d: [[[Vector3::new(0.0, 0.0, 0.0); 4]; 4]; 4],
        }
    }

    fn set_b_spline_val(&mut self, i: usize, j: usize, k: usize, val: f32) {
        if i < 4 && j < 4 && k < 4 {
            self.w[i][j][k] = val;
        }
    }

    fn set_b_spline_d_val(&mut self, i: usize, j: usize, k: usize, val: Vector3<f32>) {
        if i < 4 && j < 4 && k < 4 {
            self.w_d[i][j][k] = val;
        }
    }

    pub fn compute_neighborhood_bounds(&mut self) {
        self.i1 = (self.pos.x / self.h - 2.0).ceil() as usize;
        self.i2 = (self.pos.x / self.h + 2.0).floor() as usize + 1;
        self.j1 = (self.pos.y / self.h - 2.0).ceil() as usize;
        self.j2 = (self.pos.y / self.h + 2.0).floor() as usize + 1;
        self.k1 = (self.pos.z / self.h - 2.0).ceil() as usize;
        self.k2 = (self.pos.z / self.h + 2.0).floor() as usize + 1;

        self.i1 = self.i1.max(0);
        self.i2 = self.i2.min(self.resolution.x);
        self.j1 = self.j1.max(0);
        self.j2 = self.j2.min(self.resolution.y);
        self.k1 = self.k1.max(0);
        self.k2 = self.k2.min(self.resolution.z);
    }

    pub fn compute_b_spline_d(&mut self) {
        for dest_i in self.i1..self.i2 {
            for dest_j in self.j1..self.j2 {
                for dest_k in self.k1..self.k2 {
                    let scaled = self.pos / self.h - Vector3::new(dest_i as f32, dest_j as f32, dest_k as f32);
                    self.set_b_spline_val(dest_i - self.i1, dest_j - self.j1, dest_k - self.k1, Helpers::b_spline(scaled));
                    self.set_b_spline_d_val(dest_i - self.i1, dest_j - self.j1, dest_k - self.k1, Helpers::b_spline_d(scaled, self.h));
                }
            }
        }
    }

    pub fn b_spline_at(&self, grid_i: usize, grid_j: usize, grid_k: usize) -> f32 {
        let i = grid_i - self.i1;
        let j = grid_j - self.j1;
        let k = grid_k - self.k1;
        if i < 4 && j < 4 && k < 4 {
            self.w[i][j][k]
        } else {
            0.0
        }
    }

    pub fn b_spline_grad_at(&self, grid_i: usize, grid_j: usize, grid_k: usize) -> Vector3<f32> {
        let i = grid_i - self.i1;
        let j = grid_j - self.j1;
        let k = grid_k - self.k1;
        if i < 4 && j < 4 && k < 4 {
            self.w_d[i][j][k]
        } else {
            Vector3::new(0.0, 0.0, 0.0)
        }
    }

    pub fn get_sphere_material(&self, context: &Context) -> Gm<Mesh, ColorMaterial> {
        let pos = three_d::Vector3::new(self.pos.x, self.pos.y, self.pos.z);
        let mut sphere = Gm::new(
            Mesh::new(context, &CpuMesh::sphere(8)), ColorMaterial {
                color: Srgba::new(255, 255, 255, 255),
                texture: None,
                render_states: Default::default(),
                is_transparent: false,
            });
        sphere.set_transformation(Mat4::from_translation(pos) * Mat4::from_scale(0.04));
        sphere
    }
}

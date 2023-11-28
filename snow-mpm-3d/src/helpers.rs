use std::f32::consts::E;
use nalgebra::{Matrix3, SVD, Vector3};
use crate::particle::Particle;

pub struct Helpers {}

impl Helpers {
    fn lame_mu(mu_0: f32, xi: f32, j_p: f32) -> f32 {
        mu_0 * E.powf(xi * (1.0 - j_p))
    }

    fn lame_lambda(lambda_0: f32, xi: f32, j_p: f32) -> f32 {
        lambda_0 * E.powf(xi * (1.0 - j_p))
    }

    fn polar_r(f: Matrix3<f32>) -> Matrix3<f32> {
        let svd_result = SVD::new(f, true, true);
        let u = svd_result.u.unwrap();
        let v_t = svd_result.v_t.unwrap();
        let v = v_t.transpose();
        u * v.adjoint()
    }

    pub fn psi_derivative(mu_0: f32, lambda_0: f32, xi: f32, particle: &Particle) -> Matrix3<f32> {
        let j_p = particle.def_p_d.determinant();
        let j_e = particle.f_ep_d.determinant();
        let r_e = Helpers::polar_r(particle.f_ep_d);
        2.0 * Helpers::lame_mu(mu_0, xi, j_p) * (particle.f_ep_d - r_e) + Helpers::lame_lambda(lambda_0, xi, j_p) * (j_e - 1.0) * j_e * particle.f_ep_d.transpose().try_inverse().unwrap()
    }

    pub fn outer_product(vec1: Vector3<f32>, vec2: Vector3<f32>) -> Matrix3<f32> {
        Matrix3::new(
            vec1.x * vec2.x, vec1.x * vec2.y, vec1.x * vec2.z,
            vec1.y * vec2.x, vec1.y * vec2.y, vec1.y * vec2.z,
            vec1.z * vec2.x, vec1.z * vec2.y, vec1.z * vec2.z,
        )
    }

    fn n(x: f32) -> f32 {
        let abs_x = x.abs();
        if abs_x < 1.0 {
            0.5 * abs_x * abs_x * abs_x - x * x + 2.0 / 3.0
        } else if abs_x < 2.0 {
            -1.0 / 6.0 * abs_x * abs_x * abs_x + x * x - 2.0 * abs_x + 4.0 / 3.0
        } else {
            0.0
        }
    }

    fn n_d(x: f32) -> f32 {
        let abs_x = x.abs();
        let sign = x.signum();
        if abs_x < 1.0 {
            1.5 * x * x * sign - 2.0 * x
        } else if abs_x < 2.0 {
            -0.5 * x * x * sign + 2.0 * x - 2.0 * sign
        } else {
            0.0
        }
    }

    pub fn b_spline(scaled: Vector3<f32>) -> f32 {
        Helpers::n(scaled.x) * Helpers::n(scaled.y) * Helpers::n(scaled.z)
    }

    pub fn b_spline_d(scaled: Vector3<f32>, h: f32) -> Vector3<f32> {
        let c = Vector3::new(Helpers::n(scaled.x), Helpers::n(scaled.y), Helpers::n(scaled.z));
        let dx = c.y * c.z * Helpers::n_d(scaled.x) / h;
        let dy = c.x * c.z * Helpers::n_d(scaled.y) / h;
        let dz = c.x * c.y * Helpers::n_d(scaled.z) / h;
        Vector3::new(dx, dy, dz)
    }
}
use nalgebra::Vector2;

#[derive(Debug)]
pub struct Params {
    pub hardening_coefficient: f64,
    pub critical_compression: f64,
    pub critical_stretch: f64,
    pub mu_0: f64,
    pub lambda_0: f64,
    pub dt: f64,
    pub bspline_epsilon: f64,
    pub bspline_radius: f64,
    pub particle_diam: f64,
    pub density: f64,
    pub gravity: Vector2<f64>,
    pub particle_area: f64,
    pub particle_mass: f64,
}

impl Params {
    pub fn new() -> Self {
        let young_modulus: f64 = 1.5e5;
        let poisson_ration: f64 = 0.2;
        let hardening_coefficient: f64 = 5.0;
        let critical_compression: f64 = 1.0 - 1.9e-2;
        let critical_stretch: f64 = 1.0 + 7.5e-3;

        let mu_0 = young_modulus / (2.0 * (2.0 + poisson_ration));
        let lambda_0 = young_modulus * poisson_ration / ((1.0 + poisson_ration) * (1.0 - 2.0 * poisson_ration));

        let dt: f64 = 0.0002;
        let bspline_epsilon: f64 = 1e-4;
        let bspline_radius: f64 = 2.0;
        let particle_diam: f64 = 0.002;
        let density: f64 = 100.0;
        let gravity: Vector2<f64> = Vector2::new(0.0, 9.81);
        let particle_area: f64 = particle_diam * particle_diam;
        let particle_mass: f64 = density * particle_area;

        Params {
            hardening_coefficient,
            critical_compression,
            critical_stretch,
            mu_0,
            lambda_0,
            dt,
            bspline_epsilon,
            bspline_radius,
            particle_diam,
            density,
            gravity,
            particle_area,
            particle_mass,
        }
    }
}
use macroquad::prelude::{Color, draw_circle, screen_height, screen_width};
use nalgebra::{Matrix2, Matrix4, Vector2};

pub struct Particle {
    pub volume: f64,
    pub mass: f64,
    pub position: Vector2<f64>,
    pub velocity: Vector2<f64>,

    // Elastic part of the deformation gradient
    pub def_elastic: Matrix2<f64>,
    // Plastic part of the deformation gradient
    pub def_plastic: Matrix2<f64>,

    pub weight_gradients_x: Matrix4<f64>,
    pub weight_gradients_y: Matrix4<f64>,
    pub weights: Matrix4<f64>,
    pub velocity_gradient: Matrix2<f64>,
}

impl Particle {
    pub fn new(position: Vector2<f64>, velocity: Vector2<f64>, mass: f64) -> Self {
        Particle {
            volume: 0.0,
            mass,
            position,
            velocity,

            def_elastic: Matrix2::identity(),
            def_plastic: Matrix2::identity(),

            weight_gradients_x: Matrix4::zeros(),
            weight_gradients_y: Matrix4::zeros(),
            weights: Matrix4::zeros(),
            velocity_gradient: Matrix2::zeros(),
        }
    }

    pub fn draw(&self) {
        let x = (self.position.x as f32) * screen_width();
        let y = (self.position.y as f32) * screen_height();
        let density = self.mass as f32 / (self.volume as f32);
        let density = if density > 100.0 { 100.0 } else { density };
        let color = Color::new(density / 100.0, density / 100.0, density / 100.0, 1.0);
        draw_circle(x, y, 4.0, color);
    }
}
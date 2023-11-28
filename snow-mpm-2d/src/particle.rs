use macroquad::prelude::{Color, draw_circle, screen_height, screen_width};
use nalgebra::{Matrix2, Matrix4, Vector2};

pub struct Particle {
    pub vol: f64,
    pub mass: f64,
    pub pos: Vector2<f64>,
    pub vel: Vector2<f64>,

    pub e_d: Matrix2<f64>,
    pub e_p: Matrix2<f64>,

    pub w_d_x: Matrix4<f64>,
    pub w_d_y: Matrix4<f64>,
    pub w: Matrix4<f64>,
    pub vel_d: Matrix2<f64>,
}

impl Particle {
    pub fn new(pos: Vector2<f64>, vel: Vector2<f64>, mass: f64) -> Self {
        Particle {
            vol: 0.0,
            mass,
            pos,
            vel,

            e_d: Matrix2::identity(),
            e_p: Matrix2::identity(),

            w_d_x: Matrix4::zeros(),
            w_d_y: Matrix4::zeros(),
            w: Matrix4::zeros(),
            vel_d: Matrix2::zeros(),
        }
    }

    pub fn draw(&self) {
        let x = (self.pos.x as f32) * screen_width();
        let y = (self.pos.y as f32) * screen_height();
        let density = self.mass as f32 / (self.vol as f32);
        let density = if density > 100.0 { 100.0 } else { density };
        let color = Color::new(density / 100.0, density / 100.0, density / 100.0, 1.0);
        // cap color at 0.5
        let color = if color.r < 0.5 { Color::new(0.5, 0.5, 0.5, 1.0) } else { color };
        draw_circle(x, y, 3.0, color);
    }
}
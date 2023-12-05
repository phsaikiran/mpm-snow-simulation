use nalgebra::Vector3;
use three_d::{ColorMaterial, Context, CpuMesh, Gm, Mat4, Mesh, Positions, Srgba, vec3, vec4};

pub struct Plane {
    o: Vector3<f32>,
    u: Vector3<f32>,
    v: Vector3<f32>,
    normal: Vector3<f32>,
    mu: f32,
    vel: Vector3<f32>,
    model: Mat4,
    color: Srgba,
}

impl Plane {
    pub fn new(o: Vector3<f32>, u: Vector3<f32>, v: Vector3<f32>, mu: f32, vel: Vector3<f32>, model: Mat4, color: Srgba) -> Self {
        if u.dot(&v).abs() > 1e-6 {
            panic!("edge_u and edge_v must be orthogonal");
        }

        let normal = u.cross(&v).normalize();
        Plane {
            o,
            u,
            v,
            normal,
            mu,
            vel,
            model,
            color,
        }
    }

    pub fn collide(&self, position: Vector3<f32>, velocity: Vector3<f32>, delta_t: f32) -> Vector3<f32> {
        let vel_rel = velocity - self.vel;
        let m3d = self.model * vec4(self.o.x, self.o.y, self.o.z, 1.0);
        let model = Vector3::new(m3d.x, m3d.y, m3d.z);
        let temp = self.o + self.vel * delta_t;
        let next_m3d = self.model * vec4(temp.x, temp.y, temp.z, 1.0);
        let next_model = Vector3::new(next_m3d.x, next_m3d.y, next_m3d.z);
        let next_pos = position + velocity * delta_t;

        let next_position_origin = next_pos - model;
        let offset = (position - model).dot(&self.normal);
        let offset_next = (next_pos - next_model).dot(&self.normal);

        if offset.abs() < 1e-3 || offset * offset_next < 0.0 {
            let next_position_plane = next_position_origin - self.normal * next_position_origin.dot(&self.normal);
            let proj_u = next_position_plane.dot(&self.u);
            let proj_v = next_position_plane.dot(&self.v);
            if proj_u > 0.0 && proj_u < self.u.norm_squared()
                && proj_v > 0.0 && proj_v < self.v.norm_squared() {
                let outward_normal = if (position - model).dot(&self.normal) > 0.0 { self.normal } else { -self.normal };
                let v_n = vel_rel.dot(&outward_normal);
                let velocity_tangent = vel_rel - outward_normal * v_n;
                let mag_velocity_tangent = velocity_tangent.norm();
                return if mag_velocity_tangent <= -self.mu * v_n {
                    self.vel + Vector3::new(0.0, 0.0, 0.0)
                } else {
                    self.vel + ((1.0 + self.mu * v_n / mag_velocity_tangent) * velocity_tangent)
                };
            }
        }
        velocity
    }

    pub fn update_position(&mut self, delta_t: f32) {
        self.o += self.vel * delta_t;
    }

    pub fn get_material(&self, context: &Context) -> Gm<Mesh, ColorMaterial> {
        let m3d = self.model * vec4(self.o.x, self.o.y, self.o.z, 1.0);
        let model = vec3(m3d.x, m3d.y, m3d.z);

        let positions = vec![
            model,
            model + vec3(self.u.x, self.u.y, self.u.z),
            model + vec3(self.u.x + self.v.x, self.u.y + self.v.y, self.u.z + self.v.z),
            model,
            model + vec3(self.v.x, self.v.y, self.v.z),
            model + vec3(self.u.x + self.v.x, self.u.y + self.v.y, self.u.z + self.v.z),
        ];

        let colors = vec![
            self.color,
            Srgba::new(self.color.r + 10, self.color.g + 10, self.color.b + 10, self.color.a),
            Srgba::new(self.color.r + 20, self.color.g + 20, self.color.b + 20, self.color.a),
            self.color,
            Srgba::new(self.color.r + 30, self.color.g + 30, self.color.b + 30, self.color.a),
            Srgba::new(self.color.r + 40, self.color.g + 40, self.color.b + 40, self.color.a),
        ];

        let cpu_mesh = CpuMesh {
            positions: Positions::F32(positions),
            colors: Some(colors),
            ..Default::default()
        };

        return Gm::new(Mesh::new(&context, &cpu_mesh), ColorMaterial::default());
    }
}

pub struct Cube {
    pub sides: [Plane; 6],
}

impl Cube {
    pub fn new(o: Vector3<f32>, u: Vector3<f32>, v: Vector3<f32>, w: Vector3<f32>, mu: f32, vel: Vector3<f32>, model: Mat4, color: Srgba) -> Self {
        if u.dot(&v).abs() > 1e-6 {
            panic!("edge_u and edge_v must be orthogonal");
        }
        if u.dot(&w).abs() > 1e-6 {
            panic!("edge_u and edge_w must be orthogonal");
        }
        if v.dot(&w).abs() > 1e-6 {
            panic!("edge_v and edge_w must be orthogonal");
        }

        let ou = o + u;
        let ov = o + v;
        let ow = o + w;
        let sides = [
            Plane::new(o, u, v, mu, vel, model, color),
            Plane::new(o, u, w, mu, vel, model, color),
            Plane::new(o, v, w, mu, vel, model, color),
            Plane::new(ou, v, w, mu, vel, model, color),
            Plane::new(ov, u, w, mu, vel, model, color),
            Plane::new(ow, u, v, mu, vel, model, color),
        ];
        Cube {
            sides,
        }
    }

    fn collide(&self, position: Vector3<f32>, velocity: Vector3<f32>, delta_t: f32) -> Vector3<f32> {
        let mut new_velocity = velocity;
        for face in self.sides.iter() {
            new_velocity = face.collide(position, new_velocity, delta_t);
        }
        new_velocity
    }

    fn is_stationary(&self) -> bool {
        false
    }

    fn update_position(&mut self, delta_t: f32) {
        for face in self.sides.iter_mut() {
            face.update_position(delta_t);
        }
    }
}
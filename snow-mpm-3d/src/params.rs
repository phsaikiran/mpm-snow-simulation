#[derive(Debug)]
pub struct Params {
    pub hardening_coefficient: f32,
    pub critical_compression: f32,
    pub critical_stretch: f32,
    pub flip_pic_ration: f32,
    pub mu_0: f32,
    pub lambda_0: f32,
}

impl Params {
    pub fn new(young_modulus: f32, poisson_ration: f32, hardening_coefficient: f32, critical_compression: f32, critical_stretch: f32, flip_pic_ration: f32) -> Self {
        let mu_0 = young_modulus / (2.0 * (1.0 + poisson_ration));
        let lambda_0 = young_modulus * poisson_ration / ((1.0 + poisson_ration) * (1.0 - 2.0 * poisson_ration));

        Params {
            hardening_coefficient,
            critical_compression,
            critical_stretch,
            flip_pic_ration,
            mu_0,
            lambda_0,
        }
    }
}
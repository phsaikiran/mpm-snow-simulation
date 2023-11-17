use nalgebra::Vector2;

// Timestep
pub const DT: f64 = 0.0002;
// CRIT_COMPRESS = 1-1.9e-2 # Fracture threshold for compression
pub const CRIT_COMPRESS: f64 = 1.0 - 1.9e-2;
// CRIT_STRETCH = 1+7.5e-3 # Fracture threshold for stretching
pub const CRIT_STRETCH: f64 = 1.0 + 7.5e-3;
// HARDENING = 5.0 # How much plastic deformation strengthens material
pub const HARDENING: f64 = 5.0;
// YOUNGS_MODULUS = 1.5e5 # Young's modulus (springiness)
pub const YOUNGS_MODULUS: f64 = 1.5e5;
// POISSONS_RATIO = 0.2 # Poisson's ratio (transverse/axial strain ratio)
pub const POISSONS_RATIO: f64 = 0.2;
// BSPLINE_EPSILON = 1e-4
pub const BSPLINE_EPSILON: f64 = 1e-4;
// BSPLINE_RADIUS = 2
pub const BSPLINE_RADIUS: f64 = 2.0;
// PARTICLE_DIAM = .0072 # Diameter of each particle; smaller = higher resolution
pub const PARTICLE_DIAM: f64 = 0.0072;
// DENSITY = 100.0 # Density of snow in kg/m^2
pub const DENSITY: f64 = 100.0;
// GRAVITY = -9.8
pub const GRAVITY: Vector2<f64> = Vector2::new(0.0, 9.81);

// Hardening parameters
pub const LAMBDA: f64 = YOUNGS_MODULUS * POISSONS_RATIO / ((1.0 + POISSONS_RATIO) * (1.0 - 2.0 * POISSONS_RATIO));
pub const MU: f64 = YOUNGS_MODULUS / (2.0 + 2.0 * POISSONS_RATIO);

// Particle parameters
pub const PARTICLE_AREA: f64 = PARTICLE_DIAM * PARTICLE_DIAM;
pub const PARTICLE_MASS: f64 = DENSITY * PARTICLE_AREA;
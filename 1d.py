import numpy as np

# Parameters
num_particles = 100  # Number of material points
grid_resolution = 100  # Grid resolution
dx = 1.0 / grid_resolution  # Grid spacing
dt = 0.01  # Time step

# Particle properties
particle_mass = 1.0
particle_velocity = np.zeros(num_particles)
particle_position = np.linspace(0, 1, num_particles)  # Equally spaced initially

# Grid properties
grid_mass = np.zeros(grid_resolution)
grid_velocity = np.zeros(grid_resolution)

# Simulation loop
num_steps = 100
for step in range(num_steps):
    # Particle to Grid
    for p in range(num_particles):
        left_grid_index = int(particle_position[p] * (grid_resolution - 1))
        right_grid_index = left_grid_index + 1

        # Linear interpolation to assign mass and velocity to the grid
        grid_mass[left_grid_index] += particle_mass * (
                    1.0 - (particle_position[p] * (grid_resolution - 1) - left_grid_index))
        grid_mass[right_grid_index] += particle_mass * (particle_position[p] * (grid_resolution - 1) - left_grid_index)

        grid_velocity[left_grid_index] += particle_velocity[p]
        grid_velocity[right_grid_index] += particle_velocity[p]

    # Grid update
    for i in range(grid_resolution):
        if grid_mass[i] > 0:
            grid_velocity[i] /= grid_mass[i]

    # Grid to Particle
    for p in range(num_particles):
        left_grid_index = int(particle_position[p] * (grid_resolution - 1))
        right_grid_index = left_grid_index + 1

        # Linear interpolation to update particle velocity
        particle_velocity[p] = (
                grid_velocity[left_grid_index] * (
                    1.0 - (particle_position[p] * (grid_resolution - 1) - left_grid_index)) +
                grid_velocity[right_grid_index] * (particle_position[p] * (grid_resolution - 1) - left_grid_index)
        )

    # Update particle positions
    particle_position += dt * particle_velocity

# Print the final particle positions
print(particle_position)

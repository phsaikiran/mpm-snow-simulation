import numpy as np
import matplotlib.pyplot as plt

# Constants
N_PARTICLES = 1000  # Number of material points
GRID_SIZE_X = 100  # Grid size in the x-direction
GRID_SIZE_Y = 100  # Grid size in the y-direction
DX = 1.0  # Grid spacing

# Initialize particle positions and velocities
particle_positions = np.random.rand(N_PARTICLES, 2) * [GRID_SIZE_X - 1, GRID_SIZE_Y - 1]
particle_velocities = np.random.rand(N_PARTICLES, 2) * 0.0

# Simulation parameters
total_time_steps = 1000
dt = 0.01

# Main simulation loop
for step in range(total_time_steps):
    # Initialize grid values
    grid_mass = np.zeros((GRID_SIZE_X, GRID_SIZE_Y))
    grid_velocity = np.zeros((GRID_SIZE_X, GRID_SIZE_Y, 2))
    grid_density = np.zeros((GRID_SIZE_X, GRID_SIZE_Y))

    # Map particle data to the grid
    for i in range(N_PARTICLES):
        px, py = particle_positions[i]
        grid_x = int(px / DX)
        grid_y = int(py / DX)

        # Calculate the interpolation weights
        alpha_x = (px - grid_x * DX) / DX
        alpha_y = (py - grid_y * DX) / DX

        # Linear interpolation to map velocity to the grid
        if GRID_SIZE_X > grid_x >= 0 and GRID_SIZE_Y > grid_y >= 0:
            grid_velocity[grid_x, grid_y] += (1 - alpha_x) * (1 - alpha_y) * particle_velocities[i]
            if grid_x + 1 < GRID_SIZE_X:
                grid_velocity[grid_x + 1, grid_y] += alpha_x * (1 - alpha_y) * particle_velocities[i]
            if grid_y + 1 < GRID_SIZE_Y:
                grid_velocity[grid_x, grid_y + 1] += (1 - alpha_x) * alpha_y * particle_velocities[i]
            if grid_x + 1 < GRID_SIZE_X and grid_y + 1 < GRID_SIZE_Y:
                grid_velocity[grid_x + 1, grid_y + 1] += alpha_x * alpha_y * particle_velocities[i]

        # Map mass to the grid (in this example, we assume constant mass)
        if GRID_SIZE_X > grid_x >= 0 and GRID_SIZE_Y > grid_y >= 0:
            grid_mass[grid_x, grid_y] += 1.0 - alpha_x
            if grid_x + 1 < GRID_SIZE_X:
                grid_mass[grid_x + 1, grid_y] += alpha_x
            if grid_y + 1 < GRID_SIZE_Y:
                grid_mass[grid_x, grid_y + 1] += 1.0 - alpha_y
            if grid_x + 1 < GRID_SIZE_X and grid_y + 1 < GRID_SIZE_Y:
                grid_mass[grid_x + 1, grid_y + 1] += alpha_y

        # Map density to the grid (in this example, we assume constant density)
        if GRID_SIZE_X > grid_x >= 0 and GRID_SIZE_Y > grid_y >= 0:
            grid_density[grid_x, grid_y] += 1.0 - alpha_x
            if grid_x + 1 < GRID_SIZE_X:
                grid_density[grid_x + 1, grid_y] += alpha_x
            if grid_y + 1 < GRID_SIZE_Y:
                grid_density[grid_x, grid_y + 1] += 1.0 - alpha_y
            if grid_x + 1 < GRID_SIZE_X and grid_y + 1 < GRID_SIZE_Y:
                grid_density[grid_x + 1, grid_y + 1] += alpha_y

    # Update particle velocities based on the grid values
    for i in range(N_PARTICLES):
        px, py = particle_positions[i]
        grid_x = int(px / DX)
        grid_y = int(py / DX)

        alpha_x = (px - grid_x * DX) / DX
        alpha_y = (py - grid_y * DX) / DX

        # Bilinear interpolation to map velocity from the grid to the particle
        interpolated_velocity = 0.0
        if GRID_SIZE_X > grid_x >= 0 and GRID_SIZE_Y > grid_y >= 0:
            interpolated_velocity += (1 - alpha_x) * (1 - alpha_y) * grid_velocity[grid_x, grid_y]
            if grid_x + 1 < GRID_SIZE_X:
                interpolated_velocity += alpha_x * (1 - alpha_y) * grid_velocity[grid_x + 1, grid_y]
            if grid_y + 1 < GRID_SIZE_Y:
                interpolated_velocity += (1 - alpha_x) * alpha_y * grid_velocity[grid_x, grid_y + 1]
            if grid_x + 1 < GRID_SIZE_X and grid_y + 1 < GRID_SIZE_Y:
                interpolated_velocity += alpha_x * alpha_y * grid_velocity[grid_x + 1, grid_y + 1]

            # Even out density
            interpolated_velocity /= grid_density[grid_x, grid_y]

        # Update particle velocities (basic Euler time integration)
        particle_velocities[i] += dt * interpolated_velocity
        # Update based on gravity
        # particle_velocities[i, 1] -= 1000 * dt

    # Update particle positions
    particle_positions += dt * particle_velocities

    # Visualize the simulation
    plt.clf()
    plt.scatter(particle_positions[:, 0], particle_positions[:, 1], s=5)
    plt.xlim(0, GRID_SIZE_X)
    plt.ylim(0, GRID_SIZE_Y)
    plt.title(f'Time Step: {step}')
    plt.pause(0.01)

plt.show()

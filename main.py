import pygame

import border
import grid_node
import helpers
import water_particle
from mpm import MPM


def init_simulation():
    borders = border.init_borders()
    nodes = grid_node.init_nodes()
    particles = water_particle.init_particles()

    return MPM(borders, nodes, particles)


def step_simulation(mpm):
    mpm.particle_to_grid()


if __name__ == "__main__":
    mpm_sim = init_simulation()

    scr = helpers.init_screen()

    # Wait for user to close window
    running = True
    while running:
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = False

        step_simulation(mpm_sim)

        scr.fill("black")
        mpm_sim.draw(scr)
        pygame.display.flip()

        mpm_sim.reset()

    pygame.quit()

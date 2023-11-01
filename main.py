import pygame

import border
import grid_node
import helpers
import water_particle
from mpm import MPM
import time


def init_simulation():
    borders = border.init_borders()
    nodes = grid_node.init_nodes()
    particles = water_particle.init_particles(1)

    return MPM(borders, nodes, particles)


def step_simulation(mpm):
    mpm.particle_to_grid()
    mpm.update_grid()
    mpm.grid_to_particle()
    mpm.update_particles()


if __name__ == "__main__":
    mpm_sim = init_simulation()

    scr = helpers.init_screen()

    step_count = 0
    # Wait for user to close window
    running = True
    while running and step_count < 3000:
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = False

        print("Iteration", step_count)
        start = time.time()
        step_simulation(mpm_sim)
        end = time.time()
        print("step_simulation", (end - start) * 1000, "ms")

        start = time.time()
        if step_count % 1000 == 0:
            scr.fill("black")
            mpm_sim.draw(scr)
            pygame.display.flip()
        end = time.time()
        print("draw", (end - start) * 1000, "ms")

        start = time.time()
        mpm_sim.reset()
        step_count += 1
        end = time.time()
        print("reset", (end - start) * 1000, "ms")

    pygame.quit()

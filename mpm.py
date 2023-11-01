import numpy as np
import pygame

import helpers
from const import Const


class MPM:
    def __init__(self, borders, nodes, particles):
        self.borders = borders
        self.nodes = nodes
        self.particles = particles

    def reset(self):
        for node in self.nodes:
            node.reset()

    def draw(self, screen):
        for b in self.borders:
            b.draw(screen)

        for node in self.nodes:
            node.draw(screen)

        for particle in self.particles:
            particle.draw(screen)

    def particle_to_grid(self):
        for particle in self.particles:
            particle.constitutive_model()

            bl_node = (Const.X_GRID + 1) * int(particle.pos.y) + int(particle.pos.x)

            for j in range(-1, 3):
                for i in range(-1, 3):
                    node_index = bl_node + i * (Const.X_GRID + 1) + j

                    dist = particle.pos - self.nodes[node_index].pos
                    weight = helpers.get_weight(dist)
                    d_weight = helpers.get_weight_derivative(dist)

                    mass = particle.mass * weight
                    temp = np.matmul(np.ones((2, 2)), pygame.Vector2(2, 2))
                    # print("temp", temp)
                    vel = particle.mass * weight * (particle.vel + 3 * temp)
                    # print("vel", vel)

                    force = particle.ap * d_weight

                    self.nodes[node_index].mass += mass
                    self.nodes[node_index].vel += vel
                    self.nodes[node_index].force += force

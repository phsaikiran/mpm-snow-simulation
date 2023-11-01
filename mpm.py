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

        # for node in self.nodes:
        #     node.draw(screen)

        for particle in self.particles:
            particle.draw(screen)

    def particle_to_grid(self):
        for particle in self.particles:
            particle.constitutive_model()
            # print("Particle index", self.particles.index(particle))
            # print("particle.pos", particle.pos)
            # print("particle.vel", particle.vel)
            # print("particle.vel_field", particle.vel_field)
            # print("particle.ap", particle.ap, "particle.deformation_gradient", particle.deformation_gradient)

            bl_node = (Const.X_GRID + 1) * int(particle.pos.y) + int(particle.pos.x)

            for j in range(-1, 3):
                for i in range(-1, 3):
                    node_index = bl_node + i + (Const.X_GRID + 1) * j
                    # print("node_index", node_index, " for j, i", j, i)

                    dist = particle.pos - self.nodes[node_index].pos
                    weight = helpers.get_weight(dist)
                    d_weight = helpers.get_weight_derivative(dist)
                    # print("dist", dist, "weight", weight, "d_weight", d_weight)

                    mass = particle.mass * weight
                    vel = particle.mass * weight * (particle.vel + 3 * np.matmul(particle.vel_field, -dist))
                    # print("mass", mass, "vel", vel)

                    force = particle.ap * d_weight
                    # print("force", force)
                    # print()

                    self.nodes[node_index].mass += mass
                    # print("Adding", vel, "to node", node_index, "final", current_node.vel)
                    self.nodes[node_index].vel += vel
                    # print("Adding", vel, "to node", node_index, "final", self.nodes[node_index].vel)
                    self.nodes[node_index].force += force
                    # print()

    def update_grid(self):
        for node in self.nodes:
            if node.mass > 0:
                # print("Node index", self.nodes.index(node))
                # print("Node before update mass", node.mass, "vel", node.vel, "force", node.force)
                node.vel /= node.mass
                # print("node.vel", node.vel)
                node.force = Const.DT * (-1 * node.force / node.mass + Const.G)
                # print("node.force", node.force)
                node.vel += node.force
                # print("node.vel", node.vel)

                node.collision()
                node.friction()
                # print("Final node.vel_col", node.vel_col, "node.vel_fri", node.vel_fri)
                # print()
                # node.vel_fri = node.vel_col

    def grid_to_particle(self):
        for particle in self.particles:
            # print("Particle index", self.particles.index(particle))
            bl_node = (Const.X_GRID + 1) * int(particle.pos.y) + int(particle.pos.x)
            # print("bl_node", bl_node)

            particle.vel = pygame.Vector2(0, 0)
            particle.vel_field = np.zeros((2, 2))

            for j in range(-1, 3):
                for i in range(-1, 3):
                    node_index = bl_node + i + (Const.X_GRID + 1) * j
                    # print("node_index", node_index, " for j, i", j, i)

                    dist = particle.pos - self.nodes[node_index].pos
                    weight = helpers.get_weight(dist)
                    # print("dist", dist, "weight", weight)

                    particle.vel += weight * self.nodes[node_index].vel_fri
                    particle.vel_field += weight * np.outer(self.nodes[node_index].vel_fri, -dist)
                    # print("particle.vel", particle.vel, "particle.vel_field", particle.vel_field)
                    # print()

    def update_particles(self):
        for particle in self.particles:
            bl_node = (Const.X_GRID + 1) * int(particle.pos.y) + int(particle.pos.x)

            saved_pos = particle.pos
            particle.pos = pygame.Vector2(0, 0)
            t_mat = np.zeros((2, 2))

            for j in range(-1, 3):
                for i in range(-1, 3):
                    node_index = bl_node + i + (Const.X_GRID + 1) * j

                    dist = saved_pos - self.nodes[node_index].pos
                    weight = helpers.get_weight(dist)
                    d_weight = helpers.get_weight_derivative(dist)

                    particle.pos += weight * (self.nodes[node_index].pos + Const.DT * self.nodes[node_index].vel_col)
                    t_mat += np.outer(self.nodes[node_index].vel_col, d_weight)

            particle.update_deformation(t_mat)

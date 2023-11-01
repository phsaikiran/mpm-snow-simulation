import numpy as np
import pygame

import helpers
from const import Const


class GridNode:
    colliding_borders = []
    borders = []

    def __init__(self, pos, mass=0.0, vel=pygame.Vector2(0, 0), vel_col=pygame.Vector2(0, 0),
                 vel_fri=pygame.Vector2(0, 0), force=pygame.Vector2(0, 0)):
        self.pos = pos
        self.mass = mass
        self.vel = vel
        self.vel_col = vel_col
        self.vel_fri = vel_fri
        self.force = force

    def reset(self):
        self.mass = 0.0
        self.vel = pygame.Vector2(0, 0)
        self.vel_col = pygame.Vector2(0, 0)
        self.vel_fri = pygame.Vector2(0, 0)
        self.force = pygame.Vector2(0, 0)
        self.colliding_borders = []

    def draw(self, screen):
        scaled_pos = self.pos * Const.X_SCREEN / Const.X_GRID
        pygame.draw.circle(screen, pygame.Color(255, 0, 0), scaled_pos, 1)

    def __str__(self):
        return "GridNode(pos={}, mass={}, vel={}, vel_col={}, vel_fri={}, force={})".format(
            self.pos, self.mass, self.vel, self.vel_col, self.vel_fri, self.force
        )

    def collision(self):
        self.vel_col = self.vel
        self.vel_fri = pygame.Vector2(0, 0)
        self.colliding_borders = []

        for border_index, border in enumerate(self.borders):
            self.vel_col, self.colliding_borders = border.collision_with_grid_node(self.pos, self.vel_col,
                                                                                   self.colliding_borders, border_index)

    def friction(self):
        self.vel_fri = self.vel_col
        for border_index in self.colliding_borders:
            self.vel_fri = self.borders[border_index].add_friction(self.vel_fri, self.vel_col, self.vel)


def init_nodes():
    node_list = []
    for j in range(0, Const.Y_GRID + 1):
        for i in range(0, Const.X_GRID + 1):
            n = GridNode(pygame.Vector2(i, j))
            n.reset()
            node_list.append(n)

    return node_list


if __name__ == '__main__':
    scr = helpers.init_screen()
    for node in init_nodes():
        print(node)
        node.draw(screen=scr)

    pygame.display.flip()
    # Wait for user to close window
    running = True
    while running:
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = False
    pygame.quit()

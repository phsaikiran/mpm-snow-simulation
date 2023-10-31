import numpy as np
import pygame

import helpers
from const import Const


class GridNode:
    colliding_borders = []
    borders = []

    def __init__(self, index, pos=pygame.Vector2(0, 0), mass=0.0, vel=pygame.Vector2(0, 0),
                 vel_col=pygame.Vector2(0, 0), vel_fri=pygame.Vector2(0, 0), force=pygame.Vector2(0, 0)):
        self.index = index
        self.pos = pos
        self.mass = mass
        self.vel = vel
        self.vel_col = vel_col
        self.vel_fri = vel_fri
        self.force = force

    def draw(self, screen):
        pos_x = self.index.x * Const.X_SCREEN / Const.X_GRID
        pos_y = self.index.y * Const.Y_SCREEN / Const.Y_GRID
        pygame.draw.circle(screen, pygame.Color(255, 0, 0), pygame.Vector2(pos_x, pos_y), 2)

    def __str__(self):
        return "GridNode(index={}, pos={}, mass={}, vel={}, vel_col={}, vel_fri={}, force={})".format(
            self.index, self.pos, self.mass, self.vel, self.vel_col, self.vel_fri, self.force
        )


def init_nodes():
    node_list = []
    for j in range(0, Const.Y_GRID + 1):
        for i in range(0, Const.X_GRID + 1):
            node_list.append(GridNode(pygame.Vector2(i, j)))

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

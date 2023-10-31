import numpy as np
import pygame

import helpers
from const import Const


class Border:
    def __init__(self, border_type="sticky", normal=pygame.Vector2(1, 0), point1=pygame.Vector2(0, 0),
                 point2=pygame.Vector2(0, 0)):
        # sticky, separate, slide
        self.border_type = border_type
        self.normal = normal
        self.point1 = point1
        self.point2 = point2

    def draw(self, screen):
        scaled_point1 = self.point1 * Const.X_SCREEN / Const.X_GRID
        scaled_point2 = self.point2 * Const.X_SCREEN / Const.X_GRID
        pygame.draw.line(screen, pygame.Color(255, 0, 0), scaled_point1, scaled_point2)

    def collision_with_grid_node(self, node_pos, node_vel, collision_list, border_index):
        dist = self.normal.dot(node_pos - self.point1)

        if self.border_type == "sticky" and dist < 0:
            node_vel = pygame.Vector2(0, 0)
        else:
            future_pos = node_pos + node_vel * Const.DT
            future_dist = self.normal.dot(future_pos - self.point1)
            dist_change = future_dist - (0 if dist < 0 else dist)

            if (self.border_type == "separate" and dist_change < 0) or (self.border_type == "slide" and dist < 0):
                node_vel -= self.normal * dist_change / Const.DT
                collision_list.append(border_index)

        return node_vel, collision_list

    def add_friction(self, node_vel_fri, node_vel_col, node_vel):
        node_vel_tan = node_vel_col - self.normal * self.normal.dot(node_vel_fri)
        if node_vel_tan.length() > 1e-7:
            t = node_vel_tan / node_vel_tan.length()

            m1 = node_vel_tan.length()
            m2 = Const.MU * (node_vel_col - node_vel).length()

            node_vel_fri -= (m1 if m1 < m2 else m2) * t
        return node_vel_fri

    def __str__(self):
        return "Border(border_type={}, normal={}, point1={}, point2={})".format(
            self.border_type, self.normal, self.point1, self.point2
        )


def init_borders():
    all_borders = []

    # Left border
    normal = pygame.Vector2(1, 0)
    point1 = pygame.Vector2(Const.BORDER_OFFSET, Const.BORDER_OFFSET)
    point2 = pygame.Vector2(Const.BORDER_OFFSET, Const.Y_GRID - Const.BORDER_OFFSET)
    border1 = Border("sticky", normal, point1, point2)
    all_borders.append(border1)

    # Right border
    normal = pygame.Vector2(-1, 0)
    point1 = pygame.Vector2(Const.X_GRID - Const.BORDER_OFFSET, Const.BORDER_OFFSET)
    point2 = pygame.Vector2(Const.X_GRID - Const.BORDER_OFFSET, Const.Y_GRID - Const.BORDER_OFFSET)
    border2 = Border("sticky", normal, point1, point2)
    all_borders.append(border2)

    # Top border
    normal = pygame.Vector2(0, -1)
    point1 = pygame.Vector2(Const.BORDER_OFFSET, Const.BORDER_OFFSET)
    point2 = pygame.Vector2(Const.X_GRID - Const.BORDER_OFFSET, Const.BORDER_OFFSET)
    border3 = Border("sticky", normal, point1, point2)
    all_borders.append(border3)

    # Bottom border
    normal = pygame.Vector2(0, 1)
    point1 = pygame.Vector2(Const.BORDER_OFFSET, Const.Y_GRID - Const.BORDER_OFFSET)
    point2 = pygame.Vector2(Const.X_GRID - Const.BORDER_OFFSET, Const.Y_GRID - Const.BORDER_OFFSET)
    border4 = Border("sticky", normal, point1, point2)
    all_borders.append(border4)

    return all_borders


if __name__ == '__main__':
    scr = helpers.init_screen()
    for b in init_borders():
        b.draw(screen=scr)

    pygame.display.flip()
    # Wait for user to close window
    running = True
    while running:
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = False
    pygame.quit()

import pygame


class Point:
    def __init__(self, x, y, vel_x, vel_y):
        self.pos = pygame.Vector2(x, y)
        self.vel = pygame.Vector2(vel_x, vel_y)

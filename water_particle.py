import math

import pygame
import numpy as np

import helpers
from const import Const


class WaterParticle:
    ap = 0.0
    deformation_gradient = 1.0

    color = pygame.Color(0, 0, 255)
    radius = 1

    def __init__(self, vol=1.0, mass=1.0, pos=pygame.Vector2(0, 0), vel=pygame.Vector2(0, 0),
                 vel_field=np.zeros((2, 2))):
        self.vol = vol
        self.mass = mass
        self.pos = pos
        self.vel = vel
        self.vel_field = vel_field

    def constitutive_model(self):
        deformation_derivative = -Const.K * (1.0 / math.pow(self.deformation_gradient, Const.GAMMA) - 1.0)
        self.ap = deformation_derivative * self.vol * self.deformation_gradient

    def update_deformation(self, nodal_deformation):
        self.deformation_gradient = (1.0 + Const.DT * nodal_deformation.trace()) * self.deformation_gradient

    def draw(self, screen):
        scaled_pos = self.pos * Const.X_SCREEN / Const.X_GRID
        scaled_radius = self.radius * Const.X_SCREEN / Const.X_GRID
        pygame.draw.circle(screen, self.color, scaled_pos, scaled_radius)

    def __str__(self):
        return "WaterParticle(vol={}, mass={}, pos={}, vel={}, vel_field={})".format(
            self.vol, self.mass, self.pos, self.vel, self.vel_field
        )


def init_particles(n=10):
    particles = []
    # for pos_xy in [[2, 95.9987], [2, 94.9364], [2, 94.8067], [2, 93.6913], [2, 93.415], [2, 93.0201], [2, 92.6497],
    #                [2, 91.604]]:
    for i in range(n):
        vol = 1.14
        mass = 0.0005
        x_rand = Const.BORDER_OFFSET * (2 - np.random.random())
        y_rand = Const.BORDER_OFFSET * (2 - np.random.random())
        # print(x_rand, y_rand)
        pos = pygame.Vector2(Const.X_GRID // 2 + x_rand, Const.Y_GRID // 2 + y_rand)
        # pos = pygame.Vector2(pos_xy[0], pos_xy[1])
        vel = pygame.Vector2(0, 0)
        vel_field = np.zeros((2, 2))
        particles += [WaterParticle(vol, mass, pos, vel, vel_field)]
    return particles


if __name__ == '__main__':
    scr = helpers.init_screen()
    for p in init_particles():
        print(p)
        p.draw(screen=scr)

    pygame.display.flip()
    # Wait for user to close window
    running = True
    while running:
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = False
    pygame.quit()

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

    def draw(self, screen):
        scaled_pos = self.pos * Const.X_SCREEN / Const.X_GRID
        scaled_radius = self.radius * Const.X_SCREEN / Const.X_GRID
        pygame.draw.circle(screen, self.color, scaled_pos, scaled_radius)

    def __str__(self):
        return "WaterParticle(vol={}, mass={}, pos={}, vel={}, vel_field={})".format(
            self.vol, self.mass, self.pos, self.vel, self.vel_field
        )


def init_particles(n=10):
    for i in range(n):
        vol = 1
        mass = 1
        x_rand = Const.BORDER_OFFSET * (2 - np.random.random())
        y_rand = Const.BORDER_OFFSET * (2 - np.random.random())
        print(x_rand, y_rand)
        pos = pygame.Vector2(Const.X_GRID // 2 + x_rand, Const.Y_GRID // 2 + y_rand)
        vel = pygame.Vector2(0, 0)
        vel_field = np.zeros((2, 2))
        yield WaterParticle(vol, mass, pos, vel, vel_field)


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

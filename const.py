import pygame


class Const:
    X_GRID = 200
    Y_GRID = 100
    X_SCREEN = 1280
    Y_SCREEN = X_SCREEN * Y_GRID // X_GRID

    BORDER_OFFSET = 2

    DT = 0.001
    # Gravity
    G = pygame.Vector2(0, 9.8)
    # Friction
    MU = 0.3
    # Density
    RHO = 1.0
    # Bulk modulus
    K = 50.0
    GAMMA = 3.0

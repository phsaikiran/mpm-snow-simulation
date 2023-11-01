import matplotlib.pyplot as plt
import numpy as np
import pygame

from const import Const


def init_screen():
    pygame.init()
    screen = pygame.display.set_mode((Const.X_SCREEN, Const.Y_SCREEN))
    screen.fill("black")
    return screen


def bspline(x):
    x = abs(x)
    if 0 <= x < 1:
        return 0.5 * x * x * x - x * x + 2 / 3
    elif 1 <= x < 2:
        return -1 / 6 * x * x * x + x * x - 2 * x + 4 / 3
    # elif 2 <= x < 3:
    #     return 1 / 6 * x * x * x - 2 * x * x + 10 * x / 3 - 22 / 3
    else:
        return 0


def bspline_derivative(x):
    x_abs = abs(x)
    if 0 <= x_abs < 1:
        return 1.5 * x * x_abs - 2 * x
    elif 1 <= x_abs < 2:
        return -0.5 * x * x_abs + 2 * x - 2 * x / x_abs
    # elif 2 <= x < 3:
    #     return 0.5 * x * x_abs - 4 * x + 10 / x_abs
    else:
        return 0


def get_weight(dist: pygame.Vector2):
    return bspline(dist.x) * bspline(dist.y)


def get_weight_derivative(dist: pygame.Vector2):
    return pygame.Vector2(bspline_derivative(dist.x) * bspline(dist.y), bspline(dist.x) * bspline_derivative(dist.y))


if __name__ == "__main__":
    x_plot = np.linspace(-3, 3, 100)
    y = np.array([bspline(i) for i in x_plot])
    y_derivative = np.array([bspline_derivative(i) for i in x_plot])
    plt.plot(x_plot, y)
    plt.plot(x_plot, y_derivative)
    plt.xlabel("x")
    plt.ylabel("y")
    plt.title("B-Spline")
    plt.legend(["B-Spline", "B-Spline Derivative"])
    plt.show()

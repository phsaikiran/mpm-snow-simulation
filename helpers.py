import random
import pygame

from tqdm import tqdm
from const import Const
from point import Point


def get_points_and_points_grid(show_progress=False, screen=None):
    points: dict[int, Point] = {}
    points_grid: dict[tuple[int, int], list] = {}
    inverse_points_grid: dict[int, tuple[int, int]] = {}

    for i in range(Const.GRID_RESOLUTION):
        for j in range(Const.GRID_RESOLUTION):
            points_grid[(i, j)] = []

    if show_progress:
        print("Generating points without overlap")
    for i in range(Const.TOTAL_BALLS) if not show_progress else tqdm(range(Const.TOTAL_BALLS)):
        rand_x = random.randint(Const.CHECK_X_MIN, Const.CHECK_X_MAX)
        rand_y = random.randint(Const.CHECK_Y_MIN, Const.CHECK_Y_MAX)

        # Make sure no points are overlapping
        j_loop = 0
        while j_loop < i:
            diff = pygame.Vector2(rand_x, rand_y) - points[j_loop].pos
            if diff.length() < 2 * Const.BALL_SIZE:
                while diff.length() < 2 * Const.BALL_SIZE:
                    rand_x = random.randint(Const.CHECK_X_MIN, Const.CHECK_X_MAX)
                    rand_y = random.randint(Const.CHECK_Y_MIN, Const.CHECK_Y_MAX)
                    diff = pygame.Vector2(rand_x, rand_y) - points[j_loop].pos
                j_loop = 0
            else:
                j_loop += 1

        grid_x = int(rand_x / Const.X_SCREEN_SIZE * Const.GRID_RESOLUTION)
        grid_y = int(rand_y / Const.Y_SCREEN_SIZE * Const.GRID_RESOLUTION)
        points[i] = Point(rand_x, rand_y, random.randint(-100, 100), random.randint(-100, 100))
        points_grid[(grid_x, grid_y)] += [i]
        inverse_points_grid[i] = (grid_x, grid_y)

        if screen is not None:
            draw_points(screen, {i: points[i]}, {i: inverse_points_grid[i]})
            pygame.display.flip()

    return points, points_grid, inverse_points_grid


def draw_points(screen: pygame.Surface, points: dict[int, Point], inverse_points_grid: dict[int, tuple[int, int]]):
    for index, point in points.items():
        grid_x, grid_y = inverse_points_grid[index]
        # Color based on velocity from blue (slow) to red (fast)
        normalized_velocity = 100 if point.vel.length() > 100 else point.vel.length()
        normalized_velocity /= 100
        blue = int(255 * (1 - normalized_velocity))
        red = int(255 * normalized_velocity)
        color = pygame.Color(red, 0, blue)
        # color = "blue" if (grid_x + grid_y) % 2 == 0 else "red"
        pygame.draw.circle(screen, color, point.pos, Const.BALL_SIZE)
        # text = "{:.2f} {:.2f}".format(point.pos.x, point.pos.y)
        # text = "{}".format(point_index)
        # GAME_FONT.render_to(screen, point.pos, text, "white")


def draw_grid(screen: pygame.Surface):
    for i in range(Const.GRID_RESOLUTION):
        pygame.draw.line(screen, "gray", (i * Const.X_SCREEN_SIZE / Const.GRID_RESOLUTION, 0),
                         (i * Const.X_SCREEN_SIZE / Const.GRID_RESOLUTION, Const.Y_SCREEN_SIZE))
        pygame.draw.line(screen, "gray", (0, i * Const.Y_SCREEN_SIZE / Const.GRID_RESOLUTION),
                         (Const.X_SCREEN_SIZE, i * Const.Y_SCREEN_SIZE / Const.GRID_RESOLUTION))
    # Draw check border
    pygame.draw.line(screen, "gray", (Const.CHECK_X_MIN, Const.CHECK_Y_MIN), (Const.CHECK_X_MAX, Const.CHECK_Y_MIN))
    pygame.draw.line(screen, "gray", (Const.CHECK_X_MIN, Const.CHECK_Y_MAX), (Const.CHECK_X_MAX, Const.CHECK_Y_MAX))
    pygame.draw.line(screen, "gray", (Const.CHECK_X_MIN, Const.CHECK_Y_MIN), (Const.CHECK_X_MIN, Const.CHECK_Y_MAX))
    pygame.draw.line(screen, "gray", (Const.CHECK_X_MAX, Const.CHECK_Y_MIN), (Const.CHECK_X_MAX, Const.CHECK_Y_MAX))


def init_screen():
    pygame.init()
    screen = pygame.display.set_mode((Const.X_SCREEN_SIZE, Const.Y_SCREEN_SIZE))
    screen.fill("black")
    return screen


def test_get_points_and_points_grid():
    points, points_grid, inverse_points_grid = get_points_and_points_grid(show_progress=True)

    test_status = True
    print("Checking for overlap and wrong grid")
    for i in range(Const.GRID_RESOLUTION):
        for j in range(Const.GRID_RESOLUTION):
            points_index = points_grid[(i, j)]
            for point_index in points_index:
                point = points[point_index]
                grid_x = int(point.pos.x / Const.X_SCREEN_SIZE * Const.GRID_RESOLUTION)
                grid_y = int(point.pos.y / Const.Y_SCREEN_SIZE * Const.GRID_RESOLUTION)
                if grid_x != i or grid_y != j:
                    print("ERROR WRONG GRID: grid_x = {}, grid_y = {}".format(grid_x, grid_y))
                    test_status = False
                    break

                for other_point_index in points_index:
                    if point_index == other_point_index:
                        continue

                    diff = point.pos - points[other_point_index].pos
                    if diff.length() < Const.BALL_SIZE * 2:
                        print("ERROR OVERLAP: point_index = {}, other_point_index = {}".format(point_index,
                                                                                               other_point_index))
                        test_status = False
                        break

    if test_status:
        print("SUCCESS")

    screen = init_screen()
    draw_points(screen, points, inverse_points_grid)
    draw_grid(screen)
    pygame.display.flip()
    # Wait for the user to close the window
    running = True
    while running:
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = False
    pygame.quit()


if __name__ == "__main__":
    test_get_points_and_points_grid()

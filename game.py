# Example file showing a circle moving on screen
import pygame
import pickle
import helpers
import pygame.freetype
from const import Const


def start_simulation():
    screen = helpers.init_screen()
    clock = pygame.time.Clock()
    running = True
    dt = 0

    # GAME_FONT = pygame.freetype.SysFont('Mono', 16)

    points, points_grid, inverse_points_grid = helpers.get_points_and_points_grid(show_progress=True, screen=screen)

    points_store = []

    while running:
        # poll for events
        # pygame.QUIT event means the user clicked X to close your window
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = False

        # pygame.time.wait(10000)

        # fill the screen with a color to wipe away anything from last frame
        screen.fill("black")

        points_store += [points.copy()]

        helpers.draw_grid(screen)
        helpers.draw_points(screen, points, inverse_points_grid)

        # Check for collisions
        for i in range(Const.TOTAL_BALLS):
            for j in range(i + 1, Const.TOTAL_BALLS):
                point = points[i]
                other_point = points[j]

                diff = point.pos - other_point.pos
                if diff.length() <= Const.BALL_SIZE * 2:
                    # If balls are overlapping, move them apart
                    point.pos += diff.normalize() * (Const.BALL_SIZE * 2 - diff.length())
                    other_point.pos -= diff.normalize() * (Const.BALL_SIZE * 2 - diff.length())
                    # Elastic collision between two balls with equal mass
                    point.vel = point.vel.reflect(diff.normalize())
                    other_point.vel = other_point.vel.reflect(diff.normalize())

        for index, point in points.items():
            # Update velocity based on gravity
            # point.vel.y += 1000 * dt
            point.pos += point.vel * dt

            # Check if point is out of bounds
            if point.pos.x < Const.CHECK_X_MIN or point.pos.x > Const.CHECK_X_MAX or point.pos.y < Const.CHECK_Y_MIN or point.pos.y > Const.CHECK_Y_MAX:
                point.pos.x = min(max(point.pos.x, Const.CHECK_X_MIN), Const.CHECK_X_MAX)
                point.pos.y = min(max(point.pos.y, Const.CHECK_Y_MIN), Const.CHECK_Y_MAX)

            # Update velocity based on collisions with walls
            if point.pos.x <= Const.CHECK_X_MIN or point.pos.x >= Const.CHECK_X_MAX:
                point.vel.x *= -1 * 0.9
            if point.pos.y <= Const.CHECK_Y_MIN or point.pos.y >= Const.CHECK_Y_MAX:
                point.vel.y *= -1 * 0.9

        # flip() the display to put your work on screen
        pygame.display.flip()

        # limits FPS to 60
        # dt is delta time in seconds since last frame, used for framerate-
        # independent physics.
        dt = clock.tick(60) / 1000
        print("FPS: {:.4f} dt: {:.4f}".format(1 / dt, dt))
        # Put a cap on delta time
        if dt > 1 / 30:
            dt = 1 / 30
            print("Changing FPS: {:.4f} dt: {:.4f}".format(1 / dt, dt))

    pygame.quit()

    print("Saving points_store")
    with open("points_store.pkl", "wb") as f:
        pickle.dump(points_store, f)
    print("Done")


if __name__ == "__main__":
    start_simulation()

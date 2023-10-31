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

    GAME_FONT = pygame.freetype.SysFont('Mono', 16)

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

        # Material point method
        # Particle to Grid
        for p in range(Const.TOTAL_BALLS):
            left_grid_index = int(points[p].pos.x / Const.X_SCREEN_SIZE * (Const.GRID_RESOLUTION - 1))
            right_grid_index = left_grid_index + 1

            # Linear interpolation to assign mass and velocity to the grid
            points_grid[inverse_points_grid[p]].remove(p)
            inverse_points_grid[p] = (left_grid_index, int(points[p].pos.y / Const.Y_SCREEN_SIZE * (Const.GRID_RESOLUTION - 1)))
            points_grid[inverse_points_grid[p]].append(p)

        # Grid update
        # for i in range(Const.GRID_RESOLUTION):
        #     for j in range(Const.GRID_RESOLUTION):
        #         if len(points_grid[(i, j)]) > 0:
        #             grid_mass[i][j] /= len(points_grid[(i, j)])
        #             grid_velocity[i][j] /= len(points_grid[(i, j)])

        # Grid to Particle
        for p in range(Const.TOTAL_BALLS):
            left_grid_index = int(points[p].pos.x / Const.X_SCREEN_SIZE * (Const.GRID_RESOLUTION - 1))
            right_grid_index = left_grid_index + 1

            # Linear interpolation to update particle velocity
            points[p].vel = (
                    points[p].vel * (
                        1.0 - (points[p].pos.x / Const.X_SCREEN_SIZE * (Const.GRID_RESOLUTION - 1) - left_grid_index)) +
                    points[p].vel * (points[p].pos.x / Const.X_SCREEN_SIZE * (Const.GRID_RESOLUTION - 1) - left_grid_index)
            )

        # Update particle positions
        for p in range(Const.TOTAL_BALLS):
            points[p].pos += points[p].vel * dt

        

        # average_vel = pygame.Vector2(0, 0)
        # for point in points.values():
        #     average_vel += point.vel
        # average_vel /= len(points)
        # GAME_FONT.render_to(screen, (0, 0), "Average velocity: {:.4f}".format(average_vel.length()), "white")

        # Check for collisions
        for i in range(Const.TOTAL_BALLS):
            # Get all points in the same grid as point i
            grid_index = inverse_points_grid[i]
            nearby_points = points_grid[grid_index].copy()
            # Get points from neighboring grids
            for nearby_grid_offset_x in range(-1, 2):
                for nearby_grid_offset_y in range(-1, 2):
                    if (grid_index[0] + nearby_grid_offset_x < 0 or
                            grid_index[0] + nearby_grid_offset_x >= Const.GRID_RESOLUTION or
                            grid_index[1] + nearby_grid_offset_y < 0 or
                            grid_index[1] + nearby_grid_offset_y >= Const.GRID_RESOLUTION or
                            (nearby_grid_offset_x == 0 and nearby_grid_offset_y == 0)):
                        continue
                    new_grid_index = (grid_index[0] + nearby_grid_offset_x, grid_index[1] + nearby_grid_offset_y)
                    nearby_points.extend(points_grid[new_grid_index].copy())

            for other_point_index in nearby_points:
                if i == other_point_index:
                    continue

                point = points[i]
                other_point = points[other_point_index]

                diff = point.pos - other_point.pos
                if diff.length() == 0:
                    print("ERROR: diff.length() == 0")
                    continue
                if diff.length() <= Const.BALL_SIZE * 2:
                    # If balls are overlapping, move them apart
                    point.pos += diff.normalize() * (Const.BALL_SIZE * 2 - diff.length())
                    other_point.pos -= diff.normalize() * (Const.BALL_SIZE * 2 - diff.length())
                    # Elastic collision between two balls with equal mass
                    point.vel = point.vel.reflect(diff.normalize()) * 0.8
                    other_point.vel = other_point.vel.reflect(diff.normalize()) * 0.8

        for index, point in points.items():
            # Update velocity based on gravity
            point.vel.y += 1000 * dt
            point.pos += point.vel * dt

            # Check if point is out of bounds
            if point.pos.x < Const.CHECK_X_MIN or point.pos.x > Const.CHECK_X_MAX or point.pos.y < Const.CHECK_Y_MIN or point.pos.y > Const.CHECK_Y_MAX:
                point.pos.x = min(max(point.pos.x, Const.CHECK_X_MIN), Const.CHECK_X_MAX)
                point.pos.y = min(max(point.pos.y, Const.CHECK_Y_MIN), Const.CHECK_Y_MAX)

            # Update velocity based on collisions with walls
            if point.pos.x <= Const.CHECK_X_MIN or point.pos.x >= Const.CHECK_X_MAX:
                point.vel.x *= -1 * 0.8
            if point.pos.y <= Const.CHECK_Y_MIN or point.pos.y >= Const.CHECK_Y_MAX:
                point.vel.y *= -1 * 0.8

            # Update grid
            grid_x = int(point.pos.x / Const.X_SCREEN_SIZE * Const.GRID_RESOLUTION)
            grid_y = int(point.pos.y / Const.Y_SCREEN_SIZE * Const.GRID_RESOLUTION)
            points_grid[inverse_points_grid[index]].remove(index)
            inverse_points_grid[index] = (grid_x, grid_y)
            points_grid[inverse_points_grid[index]].append(index)

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

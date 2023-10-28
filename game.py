# Example file showing a circle moving on screen
import pygame
import pickle
import helpers
import pygame.freetype
from const import Const

# pygame setup
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
            point.vel.x *= -1 * 0.9
        if point.pos.y <= Const.CHECK_Y_MIN or point.pos.y >= Const.CHECK_Y_MAX:
            point.vel.y *= -1 * 0.9

        # # Update grid position
        # grid_x = int(point.pos.x / Const.X_SCREEN_SIZE * Const.GRID_RESOLUTION)
        # grid_y = int(point.pos.y / Const.Y_SCREEN_SIZE * Const.GRID_RESOLUTION)
        # if grid_x < 0 or grid_x >= Const.GRID_RESOLUTION or grid_y < 0 or grid_y >= Const.GRID_RESOLUTION:
        #     print("ERROR2: grid_x = {}, grid_y = {}".format(grid_x, grid_y))
        #     grid_x = min(max(grid_x, 0), Const.GRID_RESOLUTION - 1)
        #     grid_y = min(max(grid_y, 0), Const.GRID_RESOLUTION - 1)
        #
        # if grid_x != point.grid_x or grid_y != point.grid_y:
        #     points_grid[(point.grid_x, point.grid_y)].remove(index)
        #     points_grid[(grid_x, grid_y)] += [index]
        #     point.grid_x = grid_x
        #     point.grid_y = grid_y

    # for i in range(TOTAL_BALLS):
    #     point = points[i]
    #     # Color based on grid position
    #     grid_x = int(point.pos.x / X_SCREEN_SIZE * GRID_RESOLUTION)
    #     grid_y = int(point.pos.y / Y_SCREEN_SIZE * GRID_RESOLUTION)
    #     color = (grid_x * 255 / GRID_RESOLUTION, grid_y * 255 / GRID_RESOLUTION, 255)
    #     pygame.draw.circle(screen, color, point.pos, BALL_SIZE)
    #     # text = "{:.2f} {:.2f}".format(point.pos.x, point.pos.y)
    #     # GAME_FONT.render_to(screen, point.pos, text, "white")

    # Update the grid
    # for i in range(GRID_RESOLUTION):
    #     for j in range(GRID_RESOLUTION):
    #         points_index_to_check = points_grid[(i, j)]
    #         neighbors_points_index = []
    #         for k in [(-1, -1), (-1, 0), (-1, 1),
    #                   (0, -1), (0, 1), (1, -1),
    #                   (1, 0), (1, 1)]:
    #             grid_x = i + k[0]
    #             grid_y = j + k[1]
    #             if grid_x < 0 or grid_x >= GRID_RESOLUTION or grid_y < 0 or grid_y >= GRID_RESOLUTION:
    #                 # print("ERROR: grid_x = {}, grid_y = {}".format(grid_x, grid_y))
    #                 continue
    #
    #             neighbors_points_index += points_grid[(grid_x, grid_y)]
    #
    #         # print(points_index_to_check)
    #         # print()
    #         for pi in points_index_to_check:
    #             for pj in neighbors_points_index:
    #                 if pi == pj:
    #                     print("ERROR: pi = {}, pj = {}".format(pi, pj))
    #                     continue
    #
    #                 # Update point velocity based on collisions
    #                 diff = points[pi].pos - points[pj].pos
    #                 if diff.length() < BALL_SIZE * 2:
    #                     points[pi].vel = diff.normalize() * points[pi].vel.length()
    #
    #             # # Update velocity based on gravity
    #             # points[pi].vel.y += 1000 * dt
    #
    #             # Update velocity based on collisions with walls
    #             if points[pi].pos.x < BALL_SIZE * 2 or points[pi].pos.x > X_SCREEN_SIZE - BALL_SIZE * 2:
    #                 points[pi].vel.x *= -1
    #             if points[pi].pos.y < BALL_SIZE * 2 or points[pi].pos.y > Y_SCREEN_SIZE - BALL_SIZE * 2:
    #                 points[pi].vel.y *= -1
    #
    #             # Update position
    #             points[pi].pos += points[pi].vel * dt
    #
    #             # Update grid position
    #             grid_x = int(points[pi].pos.x / X_SCREEN_SIZE * GRID_RESOLUTION)
    #             grid_y = int(points[pi].pos.y / Y_SCREEN_SIZE * GRID_RESOLUTION)
    #             if grid_x < 0 or grid_x >= GRID_RESOLUTION or grid_y < 0 or grid_y >= GRID_RESOLUTION:
    #                 print("ERROR2: grid_x = {}, grid_y = {}".format(grid_x, grid_y))
    #                 grid_x = min(max(grid_x, 0), GRID_RESOLUTION - 1)
    #                 grid_y = min(max(grid_y, 0), GRID_RESOLUTION - 1)
    #
    #             # print("pi = {}, grid_x = {}, grid_y = {}".format(pi, grid_x, grid_y))
    #             if grid_x != i or grid_y != j:
    #                 points_grid[(i, j)].remove(pi)
    #                 points_grid[(grid_x, grid_y)] += [pi]

    # for i in range(TOTAL_BALLS):
    #     for j in range(TOTAL_BALLS):
    #         if i == j:
    #             continue
    #
    #         diff = points[i].pos - points[j].pos
    #         if diff.length() < BALL_SIZE * 2:
    #             points[i].vel = diff.normalize() * points[i].vel.length() * 0.8
    #             # points[j].vel = diff.normalize() * points[j].vel.length()
    #
    # for i in range(TOTAL_BALLS):
    #     point = points[i]
    #     point.pos += point.vel * dt
    #     # Update velocity based on gravity
    #     # point.vel.y += 1000 * dt
    #     if point.pos.x < BALL_SIZE * 2 or point.pos.x > X_SCREEN_SIZE - BALL_SIZE * 2:
    #         point.vel.x *= -0.8
    #     if point.pos.y < BALL_SIZE * 2 or point.pos.y > Y_SCREEN_SIZE - BALL_SIZE * 2:
    #         point.vel.y *= -0.8

    # new_ball_positions = []
    # new_velocities = []
    # for i in range(TOTAL_BALLS):
    #     point = points[i]
    #     pygame.draw.circle(screen, "white", point.pos, BALL_SIZE)
    #     # Update velocity based on collisions
    #     for other_pos, other_vel in zip(ball_positions, ball_velocities):
    #         if pos == other_pos:
    #             continue
    #         diff = pos - other_pos
    #         if diff.length() < BALL_SIZE * 2:
    #             vel = diff.normalize() * vel.length() * 0.8
    #             other_vel = diff.normalize() * other_vel.length()
    #
    #     pos += vel * dt
    #     # Update velocity based on gravity
    #     # vel.y += 1000 * dt
    #     if pos.x < BALL_SIZE * 2 or pos.x > X_SCREEN_SIZE - BALL_SIZE * 2:
    #         vel.x *= -0.8
    #     if pos.y < BALL_SIZE * 2 or pos.y > Y_SCREEN_SIZE - BALL_SIZE * 2:
    #         vel.y *= -0.8
    #
    #     new_ball_positions += [pos]
    #     new_velocities += [vel]
    # ball_positions = new_ball_positions
    # ball_velocities = new_velocities

    # keys = pygame.key.get_pressed()
    # if keys[pygame.K_w]:
    #     pos.y -= 300 * dt
    # if keys[pygame.K_s]:
    #     pos.y += 300 * dt
    # if keys[pygame.K_a]:
    #     pos.x -= 300 * dt
    # if keys[pygame.K_d]:
    #     pos.x += 300 * dt

    # flip() the display to put your work on screen
    pygame.display.flip()

    # limits FPS to 60
    # dt is delta time in seconds since last frame, used for framerate-
    # independent physics.
    dt = clock.tick(60) / 1000
    print("FPS: {:.4f} dt: {:.4f}".format(1 / dt, dt))
    # Limit to 30 FPS
    if dt > 1 / 30:
        dt = 1 / 30
        print("Changing FPS: {:.4f} dt: {:.4f}".format(1 / dt, dt))

pygame.quit()

print("Saving points_store")
with open("points_store.pkl", "wb") as f:
    pickle.dump(points_store, f)
print("Done")

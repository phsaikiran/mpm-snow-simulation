import pysph.solver.api as solver
import pysph.base.api as base
import pysph.sph.api as sph
import numpy as np
import matplotlib.pyplot as plt

# Create a fluid simulation
app = base.Application()
app.setup(
    solver=solver.Solver(dim=2, integrator_type=solver.EulerIntegrator),
    kernel=sph.CubicSplineKernel,
    real=False,
)

# Create a fluid particle array
fluid = base.create_particles(app, name='fluid', type=base.FluidParticle)
x, y = np.linspace(0, 1, 32), np.linspace(0, 1, 32)
x, y = np.meshgrid(x, y)
x, y = x.ravel(), y.ravel()
fluid.add_particles(x=x, y=y)

# Setup fluid properties
fluid.h[:] = 0.04  # Smoothing length
fluid.m[:] = 1.0  # Particle mass
fluid.rho[:] = 1000.0  # Initial density
fluid.cs[:] = 20.0  # Speed of sound
fluid.particle_volume[:] = 1.0

# Create a solver
s = app.solver

# Configure the solver
s.particles = [fluid]
s.configure_solver(dt=1e-5)

# Define a simple time-stepping loop
from pysph.sph.integrator import EPECIntegrator
integrator = EPECIntegrator(fluid=fluid)
integrator.set_time_step(s.get_time_step)

# Time loop
t = 0.0
tf = 0.1

while t < tf:
    s.update()
    t += s.get_time_step()

    # Visualize the fluid particles
    plt.clf()
    plt.scatter(fluid.x, fluid.y, s=10, c='b')
    plt.xlim(0, 1)
    plt.ylim(0, 1)
    plt.title(f'Time: {t:.3f} sec')
    plt.pause(0.01)
    plt.draw()

# Cleanup
s.cleanup()

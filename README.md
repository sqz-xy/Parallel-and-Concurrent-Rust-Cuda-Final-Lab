# 600086-Lab-I Parallel Particle Systems

## Concept

Implement two versions of a simple aerosol paint simulation, one on the CPU and the other on the GPU.

## Specification

The scene is composed of a white sheet of paper placed in the centre of a desk.  A number of aerosol cans stand on the desk in a circle that surrounds the paper.  Each can is initially facing towards the centre of the desk.

The aim of the application is to produce an image on the white paper, by simulating the paint as it is sprayed from each aerosol can, and lands on the paper.

### Spray

The aerosol spray is represented as a system of particles, where each particle is modelled as a simple projectile subject to both gravity and air resistance.

distance s = v t + 0.5 a t^2

and

acceleration a = g - k v^2

where

t = time
g = gravity (9.8 m/s^2)
k = drag
v = velocity

Initial values for velocity and drag are 1 m/s and 0.05, respectively.  These can be adjusted to achieve a good spray pattern.

It is recommended that the initial direction of the spray is parallel to the table and located at least 0.2m above the table.

The spray leaves the aerosol can in a cone.  Adjust the cone angle until the majority of the paint hits the paper.

### Particle system

The particle system should consist of 1000's or 10000's of particles.

No collision detection is required between particles.

### Colour

Each aerosol can contains a single colour of particles.

The colour of a particle is represented as a combination of Red, Green and Blue values in the range 0-1.

e.g. Bright Red is (1, 0, 0), Bright Green is (0, 1, 0), Bright Blue is (0, 0, 1), Black is (0, 0, 0) and White (1, 1, 1).

### Paper

The paper is represented as 2D array of at least 1000 x 1000 elements.

When a particle lands on the paper, one element (pixel) of the paper changes colour.

The new colour of the paper is calculated as

paper colour = (1 - b) m + b p

where

m is original paper colour
p is the aerosol colour
b is the blend coefficient (0.1)

### Desk

The desk has no role in the simulation other than as a virtual location to rest the paper and aerosol cans.

### Output

Once the simulation has completed, the paper array should be saved as a bitmap file or directly displayed on the screen.

You could use the Rust bmp crate to output the image.

## Parallel architecture

The threading requirements are specified below.  Note that no thread can implement more than one of the listed functions.

### CPU implementation

A pipeline architecture is required for the CPU implementation, consisting of 3 stages:

- particle creation: Created a batch of particles at the spray can nozzle, per time step.
- particle movement: Move a batch of the particles from the spray can nozzle to the virtual table / paper.
- paper colouring: Colour the paper with any particles from the batch that hit the paper.

A simulation run consists of multiple batches of particles.

Add suitable buffers and duplicate stages to the pipeline (as described in the lectures) to improve performance.

### GPU implementation

- Particle motion - implemented on 1 thread per particle
- Collision of particles with paper and colouring - implemented on 1 thread per collision

### Basic

(selected by a key-press)

1. At least 3 different coloured aerosol sprays location in different positions around the paper.
2. Measure the performance of the simulation, including:
   - How long did it take to complete the simulation?
   - How many particles were created?
   - How many particles hit the paper?
   - How many particles missed the paper?

### Advanced features

(selected by a key-press)

1. Visualization of aerosol particles rendered using OpenGL or DirectX, as discussed in previous labs (switch off when gathering performance metrics).
2. More creative uses of the aerosol sprays and paper

## Implementation

### CPU

A Rust implementation
Functionality requiring beta or nightly Rust builds are NOT permitted.

### GPU

A CUDA implementation.
The choice of language from which to host CUDA is left open e.g. C++, C# or Python

## Lab book entries

1. GPU design - Detail and reflect upon the design of your GPU implementation.  Focus on the parallel aspects of your solution.  Clearly explain how you have used threads, mutual exclusion and synchronization in your GPU implementation. (approx. 300-500 words)
2. CPU design - Detail and reflect upon design of your CPU implementation.  Focus on the parallel aspects of your solution. Clearly explain how you have used threads, mutual exclusion and synchronization in your CPU implementation.  (approx. 300-500 words)
3. Performance metrics - Compare the performance of your GPU and CPU implementations, both WITH and WITHOUT graphics.  Include a description of your performance benchmarks.  (approx. 300-500 words)

## Demonstration

You will be required to demonstrate your two solutions in FEN-177

## Submission

All submissions are via Canvas.

### Software - CUDA

Your code, in the form of a Visual Studio solution, source code, and working executables/DLLs, along with any required assets.

### Software - Rust

Your code, in the form of a Visual Studio Code, source code, and working executables, along with any required assets.

### Video

A short narrated video containing showing both the CPU and GPU implementations

### Lab book

The full lab book containing all labs, as a PDF.

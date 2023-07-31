/* Copyright (c) 2022, NVIDIA CORPORATION. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *  * Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *  * Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *  * Neither the name of NVIDIA CORPORATION nor the names of its
 *    contributors may be used to endorse or promote products derived
 *    from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE COPYRIGHT OWNER OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef _BICUBICTEXTURE_CU_
#define _BICUBICTEXTURE_CU_

#include <stdlib.h>
#include <stdio.h>
#include <string.h>

#include <helper_math.h>

// Other
#include <vec3.h>

 // includes, cuda
#include <helper_cuda.h>
#include <curand.h>
#include <curand_kernel.h>

// Particle
struct Particle 
{
    float3 Position;
    float3 Velocity;
    float3 Colour;
    bool HasCollided;
};

typedef unsigned int uint;
typedef unsigned char uchar;

cudaArray* d_imageArray = 0;
const int particle_count = 10000;

__device__ __managed__ uchar4 paper[1000][1000];
__device__ __managed__ Particle particles[particle_count];
__device__ __managed__ int col_count;
__device__ __constant__ float3 gravity{ 0.0f, 9.8f, 0.0f };
__device__ __constant__ float drag = 0.05f;

__device__ void print_vec3(vec3 pVecToPrint)
{
    printf("x = %f, y = %f, z = %f\n", pVecToPrint.x(), pVecToPrint.y(), pVecToPrint.z());
}

__global__ void collision_count(int pRunCount)
{
    int particles = particle_count * pRunCount;
    printf("Collision Count: %d\n", col_count);
    printf("Miss Count: %d\n", particles - col_count);
    printf("Total Particles: %d\n", col_count + (particles - col_count));
    col_count = 0;
}

__global__ void move_particle(float3 pColour, float3 pNozzlePos, float3 pPaperPos, float pRand) 
{
    uint x = __umul24(blockIdx.x, blockDim.x) + threadIdx.x;
    uint y = __umul24(blockIdx.y, blockDim.y) + threadIdx.y;
    uint i = x + y * gridDim.x * blockDim.x;

    // Create particle
    Particle p = Particle();
    p.Colour = pColour;
    p.Position = pNozzlePos;
    p.Velocity = float3(pPaperPos - pNozzlePos);
    p.HasCollided = false;

    curandState state;
    curand_init(pRand, i, 0, &state);

    // Random numbers for spray
    float randAngle = (curand_uniform(&state) - 0.5f) / 10.0f;
    float randX = (curand_uniform(&state) - 0.5f) / 10.0f;
    float randY = curand_uniform(&state)/ 2.0f;
    float randZ = (curand_uniform(&state) - 0.5f) / 10.0f;

    // Rotates the spray vector
    float rotX = (cosf(randAngle) * p.Velocity.x) - (sinf(randAngle) * p.Velocity.z);
    float rotZ = (sinf(randAngle) * p.Velocity.x) + (cosf(randAngle) * p.Velocity.z);

    p.Velocity = float3{ (rotX + randX), (p.Velocity.y + randY), (rotZ + randZ) };

    //printf("%d ", i);
    particles[i] = p;

    float time = 0.016;

    while (particles[i].Position.y >= 0.0f)
    {
        // Movement
        float3 vel = particles[i].Velocity * particles[i].Velocity;
        vel = vel * drag;
        float3 acceleration = gravity - vel;

        acceleration = acceleration * 0.5f;
        float timeSquared = time * time;
        acceleration = acceleration * timeSquared;

        float3 dist = particles[i].Velocity * time;
        dist = dist + acceleration;

        if (dist.y > 0)    
            dist = float3{ dist.x, -dist.y, dist.z };
      
        while (particles[i].Position.y >= pPaperPos.y)
        {
            particles[i].Position += dist;
        }
        particles[i].HasCollided = true;
    }
}

__global__ void collide_particle(float3 pPaperPos)
{
    uint x = __umul24(blockIdx.x, blockDim.x) + threadIdx.x;
    uint y = __umul24(blockIdx.y, blockDim.y) + threadIdx.y;
    uint i = x + y * gridDim.x * blockDim.x;
    
    float paperWidth = 1.0f, paperHeight = 1.0f;

    float xLowerBound = -(paperWidth / 2.0f) + pPaperPos.x, 
          xUpperBound = (paperWidth / 2.0f) + pPaperPos.x, 
          zLowerBound = -(paperWidth / 2.0f) + pPaperPos.z, 
          zUpperBound = (paperWidth / 2.0f) + pPaperPos.z;

    while (true)
    {
        if (particles[i].HasCollided)
        {
            // If particle is within the bounds of the paper
            if (particles[i].Position.x >= xLowerBound &&
                particles[i].Position.x <= xUpperBound &&
                particles[i].Position.z >= zLowerBound &&
                particles[i].Position.z <= zUpperBound)
            {
                float pixelPosX = (particles[i].Position.x * 1000.0f), pixelPosZ = (particles[i].Position.z * 1000.0f);
                pixelPosX += (paperWidth / 2.0f) * 1000.0f;
                pixelPosZ += (paperWidth / 2.0f) * 1000.0f;

                uchar4 originalColour = paper[static_cast<int>(pixelPosX)][static_cast<int>(pixelPosZ)];
                float3 paperColour = float3{ static_cast<float>(originalColour.x), static_cast<float>(originalColour.y), static_cast<float>(originalColour.z) };
                paperColour = paperColour * (1.0f - 0.5f);

                float3 newColour = particles[i].Colour * 0.5f;
                newColour = newColour + paperColour;
         
                paper[static_cast<int>(pixelPosX)][static_cast<int>(pixelPosZ)] = make_uchar4(newColour.x, newColour.y, newColour.z, 1.0f);
                atomicAdd(&col_count, 1);
                return;
            }
            return;
        }    
    }
}

__global__ void d_render(uchar4* d_output, uint width, uint height) {
    uint x = __umul24(blockIdx.x, blockDim.x) + threadIdx.x;
    uint y = __umul24(blockIdx.y, blockDim.y) + threadIdx.y;
    uint i = __umul24(y, width) + x;

    if ((x < width) && (y < height)) {
       d_output[i] = paper[x][y];
    }
}

__global__ void clear(uint width, uint height)
{
    uint x = __umul24(blockIdx.x, blockDim.x) + threadIdx.x;
    uint y = __umul24(blockIdx.y, blockDim.y) + threadIdx.y;
    uint i = __umul24(y, width) + x;

    paper[x][y] = make_uchar4(0xff, 0xff, 0xff, 0xff);
}


extern "C" void freeTexture() {

    checkCudaErrors(cudaFreeArray(d_imageArray));
}

// render image using CUDA
extern "C" void render(int width, int height,  dim3 blockSize, dim3 gridSize,
     uchar4 * output) {

    d_render << <gridSize, blockSize >> > (output, width, height);

    getLastCudaError("Drawing Kernel Failed");
}

// Simulate particles
extern "C" void simulate_particles(float3 pColour, float3 pPaperPos, float3 pNozzlePos, float pRand, int pRunCount)
{
    cudaDeviceSynchronize();

    cudaEvent_t start, stop;

    cudaEventCreate(&start);
    cudaEventCreate(&stop);

    // Ensure only the movement and collision kernels are timed
    cudaEventRecord(start);

    float rnd = 0.0f;
    for (int i = 0; i < pRunCount; i++)
    {
        rnd = rand();
        move_particle << <particle_count/4, 4 >> > (pColour, pNozzlePos, pPaperPos, rnd);
        collide_particle << <particle_count/4, 4 >> > (pPaperPos);
    }
    
    cudaEventRecord(stop);
    cudaEventSynchronize(stop);

    collision_count << <1, 1 >> > (pRunCount);

    float ms = 0.0f;
    cudaEventElapsedTime(&ms, start, stop);
    printf("Time Elapsed: %f ms\n", ms);

    cudaDeviceSynchronize();

    getLastCudaError("Simulation Kernel Failed");
}

extern "C" void clear_paper(int width, int height, dim3 blockSize, dim3 gridSize)
{
    clear << <gridSize, blockSize >> > (width, height);
    getLastCudaError("Paper Clearing");
}

#endif
use rand::Rng;
use concurrent_queue::ConcurrentQueue;
use atomic_counter::AtomicCounter;

use crate::Vector3f;
use crate::Particle;
use crate::paper::Paper;

use std::sync::{Arc, Mutex};

// Concurrent queue
pub struct Pipe {
    queue: ConcurrentQueue<Particle>,
    lock: Arc<Mutex<i32>>
}

impl Pipe {
    pub fn new() -> Pipe {
        Pipe { 
            queue: (ConcurrentQueue::unbounded()),
            lock: Arc::new(Mutex::new(0))
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.queue.is_empty();
    }

    pub fn push(&self, value: Particle) {
        let _ = self.queue.push(value);
    }

    pub fn pop(&self) -> Option<Particle> {
        // Lock to make sure that only one thread pops at a time
        let guard = self.lock.lock().unwrap();

        if self.is_empty() {
            return None;
        }
        else {
            return Some(self.queue.pop().unwrap());
        }
    }

    pub fn close(&self) {
        self.queue.close();
    }
}

pub fn spawning_stage(pipe_in: Arc<Pipe>, pipe_out: Arc<Pipe>) {
    while true {
        let data_option = pipe_in.pop();

        if data_option.is_none() {
            continue;
        }
        
        let data = data_option.unwrap();

        // Close the input queue and push the closing particle to the next pipe
        if data.particle_code == -1 {
            //println!("Closing spawning stage");
            pipe_out.push(data);
            pipe_in.close();
            return;
        }

        if data.particle_code == -9999 {
            pipe_out.push(data);
            continue;
        }

        // Push the number of particles specified
        if data.particle_code > 0 {
            //println!("Spawning {} Particles", data.particle_code);
            for _ in 0..data.particle_code {

                let mut particle_vel = data.target - data.position;

                let angle : f32 = rand::thread_rng().gen_range(-0.1..0.1);
                let x = (angle.cos() * particle_vel.x) - (angle.sin() * particle_vel.z);
                let z = (angle.sin() * particle_vel.x) + (angle.cos() * particle_vel.z);

                particle_vel.x = x;
                particle_vel.z = z;

                let rand_x = rand::thread_rng().gen_range(-0.1..0.1);
                let rand_y = rand::thread_rng().gen_range(-0.1..0.1);
                let rand_z = rand::thread_rng().gen_range(-0.1..0.1);

                particle_vel.x += rand_x;
                particle_vel.y += rand_y;
                particle_vel.z += rand_z;

                //particle_vel.multiply_f32(1.5); //Increased vel for better spray
                let child_particle = Particle::new(data.position, particle_vel, data.colour, data.target, 0);
                pipe_out.push(child_particle);
            }
        }

        // Deal with 0 particles
    }
}

pub fn moving_stage(pipe_in: Arc<Pipe>, pipe_out: Arc<Pipe>, p_paper_pos: Vector3f) {
    let gravity = Vector3f::new(0.0, -9.8, 0.0);
    let drag = 0.05;
    let mut delta_t: f32 = -0.5;

    while true {
        let data_option = pipe_in.pop();

        if data_option.is_none() {
            continue;
        }

        let mut data = data_option.unwrap();

        // Close the input queue and push the closing particle to the next pipe
        if data.particle_code == -1 {
            pipe_out.push(data);
            pipe_in.close();
            return;
        }

        if data.particle_code == -9999 {
            pipe_out.push(data);
            continue;
        }

        // Movement loop, if particle goes below the paper y, then pass it to next stage
        while data.position.y > p_paper_pos.y && data.particle_code > -1 {

            delta_t += 0.005;
            if delta_t > 0.7 {
                delta_t = -1.4;
            }

            // Acceleration
            let mut v_2 = data.velocity * data.velocity;
            v_2.multiply_f32(drag);
            let mut acceleration = gravity - v_2;

            // Distance
            acceleration.multiply_f32(0.5);
            let time_sq = delta_t.abs() * delta_t.abs();
            acceleration.multiply_f32(time_sq as f32);

            let mut distance = data.velocity;
            distance.multiply_f32(delta_t.abs() as f32);
            distance = distance + acceleration;

            // Add distance
            data.position = data.position + distance;
        }
       
        pipe_out.push(data);
    }
}

pub fn paper_stage(pipe_in: Arc<Pipe>, pipe_out: Arc<Pipe>, mut paper: Paper) {
    let col_counter = atomic_counter::RelaxedCounter::new(0);
    let miss_counter = atomic_counter::RelaxedCounter::new(0);
    let particle_counter = atomic_counter::RelaxedCounter::new(0);

    while true {
        let data_option = pipe_in.pop();

        if data_option.is_none() {
            continue;
        }
        
        let mut data = data_option.unwrap();

        // Close the input queue and push the closing particle to the next pipe
        if data.particle_code == -1 {
            println!("Collisions: {}\nMisses: {}\nTotal Particles: {}\n", col_counter.get(), miss_counter.get(), particle_counter.get());
            pipe_out.push(data);
            pipe_in.close();          
            return;
        }

        if data.particle_code == -9999 {
            pipe_out.push(data);
            paper.save("Paper.bmp");
            continue;
        }

        particle_counter.inc();

        // Collision
        let x_lower_bound = -(paper.width/2.0) + paper.position.x;
        let x_upper_bound = (paper.width/2.0) + paper.position.x;

        let z_lower_bound = -(paper.height/2.0) + paper.position.z;
        let z_upper_bound = (paper.height/2.0) + paper.position.z;

        if data.position.x >= x_lower_bound && data.position.x <= x_upper_bound && data.position.z >= z_lower_bound && data.position.z <= z_upper_bound {

            // Remove DP
            let x_round = ((data.position.x * 1000.0) + (paper.width / 2.0) * 1000.0) as i32;
            let z_round = ((data.position.z * 1000.0) + (paper.height / 2.0) * 1000.0) as i32;

            // Blending
            let current_colour = paper.get_pixel(x_round as f32, z_round as f32);
            let mut new_colour = current_colour;
            new_colour.multiply_f32(1.0 - 0.1);
            data.colour.multiply_f32(0.1);
            new_colour = new_colour + data.colour;

            paper.set_pixel(x_round as f32, z_round as f32, new_colour);
            col_counter.inc();
            pipe_out.push(data);
        }
        else {
            miss_counter.inc();
            pipe_out.push(data);   
        }
    }
}
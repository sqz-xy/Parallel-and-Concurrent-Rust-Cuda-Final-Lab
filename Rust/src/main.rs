use inputbot::{KeybdKey::*};

use std::io::stdin;

use std::{thread};
use std::sync::Arc;

pub mod vector3;
use vector3::Vector3f;

pub mod particle;
use particle::Particle;

pub mod paper;
use paper::Paper;

pub mod pipeline;
use pipeline::{Pipe, spawning_stage, moving_stage, paper_stage};

use std::time::SystemTime;


const key_control: bool = true;
const particle_amount: i32 = 100000;
const run_count: i32 = 100;
const save_image: bool = true;

fn spray_colour(input_pipe: Arc<Pipe>, nozzle_pos: Vector3f, colour: Vector3f, paper_pos: Vector3f, count: i32)  {
    println!("Particle Code: {}\nNozzle Position: {}\nColour: {}", count, nozzle_pos, colour);
    input_pipe.push(Particle::new(nozzle_pos, Vector3f::empty(), colour, paper_pos, count));
}

fn spray_novel(input_pipe: Arc<Pipe>) {
    println!("Novel Spray");

    let mut bone_one_pos = Vector3f::new(0.3, 0.1, 0.3);
    let mut bone_two_pos = Vector3f::new(-0.3, 0.1, 0.3);

    let centre = Vector3f::empty();
    let mut centre_two =  Vector3f::new(0.0, 0.2, 0.0);
    let gray = Vector3f::new(0.2, 0.2, 0.2);

    input_pipe.push(Particle::new(bone_one_pos, Vector3f::empty(), gray, centre, 100000));
    input_pipe.push(Particle::new(bone_two_pos, Vector3f::empty(), gray, centre, 100000));

    centre_two.y = 20.0;
    
    input_pipe.push(Particle::new(centre_two, Vector3f::empty(), gray, centre, 100000));

    let black = Vector3f::empty();
    let eye_one_pos = Vector3f::new(-0.1, 0.1, -0.2); 
    let eye_two_pos = Vector3f::new(0.1, 0.1, -0.2); 
    let eye_one_target = Vector3f::new(-0.1, 0.0, -0.2); 
    let eye_two_target = Vector3f::new(0.1, 0.0, -0.2); 

    input_pipe.push(Particle::new(eye_one_pos, Vector3f::empty(), black, eye_one_target, 100000));
    input_pipe.push(Particle::new(eye_two_pos, Vector3f::empty(), black, eye_two_target, 100000));
    
    let mouth_pos = Vector3f::new(0.2, 0.1, 0.0); 
    let mouth_pos_two = Vector3f::new(-0.2, 0.1, 0.0); 

    input_pipe.push(Particle::new(mouth_pos, Vector3f::empty(), black, centre, 1000000));
    input_pipe.push(Particle::new(mouth_pos_two, Vector3f::empty(), black, centre, 1000000));

}

fn main() {

    // Having multiple stage 2's with locking seems to be the same speed as having one

    /* 
    Scale:
            1.0 : 1m
            0.1 : 10cm
    */

    /*
    Variables:
            key_control : Bool, if true, keyboard control will be enabled, else, a simulation will run using the parameters below
            particle_amount : Number of particles per spray
            run_count : Number of sprays to do
            save_image : whether to save the image at the end of the simulation

            (
                Within the simulation, one of each spray will be fired.
                If particle_amount = 1000, and run_count = 1:
                3000 particles would be sprayed, this number doesn't include saving or terminating particles.
            )
    
    */

    /*
    Particle Codes:
            -1 : Close Pipeline
            -9999 : Save Image
            0 : Do Nothing
            1 or Above : Spawn n number of particles in first stage   
    */

    /*
    Controls (set "key_control" to true for keyboard control instead of normal simulation):
            w : red spray
            a : blue spray
            d : green spray
            s : save image
            esc : close pipeline
    */

    let dist = 0.6;

    let vec1 = Vector3f::new(-dist, 0.2, dist);

    let vec2 = Vector3f::new(dist, 0.2, -dist);

    let vec3 = Vector3f::new(dist, 0.2, dist);

    let red = Vector3f::new(1.0, 0.0, 0.0);
    let green = Vector3f::new(0.0, 1.0, 0.0);
    let blue = Vector3f::new(0.0, 0.0, 1.0);

    // Paper is at 0,0
    let paper_pos = Vector3f::empty();
    let mut paper = Paper::new(paper_pos, 1.0, 1.0);
    paper.clear();

    // Initialise pipes
    let pipe_one_arc = Arc::new(Pipe::new());
    let pipe_two_arc = Arc::new(Pipe::new());
    let pipe_three_arc = Arc::new(Pipe::new());
    let pipe_four_arc = Arc::new(Pipe::new());
    
    // Build Pipeline
    let spawning_input = pipe_one_arc.clone();
    let spawning_output = pipe_two_arc.clone();

    let moving_input = pipe_two_arc.clone();
    let moving_input_2 = pipe_two_arc.clone();
    let moving_output = pipe_three_arc.clone();
    let moving_output_2 = pipe_three_arc.clone();

    let paper_input = pipe_three_arc.clone();
    let paper_output = pipe_four_arc.clone();

    // Keyboard Input
    let red_input = pipe_one_arc.clone();
    let blue_input = pipe_one_arc.clone();
    let green_input = pipe_one_arc.clone();
    let term_input = pipe_one_arc.clone();
    let save_input = pipe_one_arc.clone();
    let novel_input = pipe_one_arc.clone();

    let handler = thread::spawn(move || {spawning_stage(spawning_input, spawning_output)});                         // Stage 1
    let handler2 = thread::spawn(move || {moving_stage(moving_input, moving_output, paper_pos)});      // Stage 2
    let handler3 = thread::spawn(move || {moving_stage(moving_input_2, moving_output_2, paper_pos)});  // Stage 2
    let handler4 = thread::spawn(move || {paper_stage(paper_input, paper_output, paper)});                         // Stage 3
    
    // Push Input
    let input_pipe = pipe_one_arc.clone();
    let mut start = SystemTime::now();

    if key_control {
        // Key control
        WKey.bind(move || {spray_colour(red_input.clone(), vec1, red, paper_pos, particle_amount)});
        AKey.bind(move || {spray_colour(blue_input.clone(), vec2, blue, paper_pos, particle_amount)});
        DKey.bind(move || {spray_colour(green_input.clone(), vec3, green, paper_pos, particle_amount)});
        NKey.bind(move || {spray_novel(novel_input.clone())});
        SKey.bind(move || {spray_colour(save_input.clone(), Vector3f::empty(), Vector3f::empty(), Vector3f::empty(), -9999)});
        EscapeKey.bind(move || {spray_colour(term_input.clone(), Vector3f::empty(), Vector3f::empty(), Vector3f::empty(),  -1)});
    }
    else {
        // Simulation run
        start = SystemTime::now();

        for _ in 0..run_count {
            input_pipe.push(Particle::new(vec1, vec1, red, paper_pos, particle_amount));
            input_pipe.push(Particle::new(vec2, vec1, green, paper_pos, particle_amount));
            input_pipe.push(Particle::new(vec3, vec1, blue, paper_pos, particle_amount));
        }

        // Saving and closing particle
        if save_image {
            input_pipe.push(Particle::new(vec1, Vector3f::empty(), Vector3f::empty(), Vector3f::empty(), -9999)); // This one affects performance (Saving)
        }
        input_pipe.push(Particle::new(vec1, Vector3f::empty(), Vector3f::empty(), Vector3f::empty(), -1));
    }
      
    let mut run = true;
    let output_pipe = pipe_four_arc.clone();

    while run {
        let val = output_pipe.pop();

        // runs once at the start

        // For keyboard control
        if key_control {
            inputbot::handle_input_events();
        }

        if !val.is_none() {
            if val.unwrap().particle_code == -1 {
                run = false;
            }
        }    
    }
    
    //let _ = handler.join();
    //let _ = handler2.join();
    //let _ = handler3.join();
    //let _ = handler4.join();
    
    // End the timer once the pipeline has terminated
    println!("Simulation time = {} millis\nSimulation time = {} micros",  start.elapsed().unwrap().as_millis(), start.elapsed().unwrap().as_micros());
    println!("Press Enter to close");
    stdin().read_line(&mut String::new()).unwrap();
}

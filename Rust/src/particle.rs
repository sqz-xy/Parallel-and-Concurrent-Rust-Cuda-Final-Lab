use std::fmt::{self, Display};

use crate::Vector3f;

#[derive(Debug, Copy, Clone)]
pub struct Particle {
    pub position: Vector3f,
    pub velocity: Vector3f,
    pub target: Vector3f,
    pub colour: Vector3f,
    pub particle_code: i32
}

impl Particle {
    pub fn new (p_pos: Vector3f, p_vel: Vector3f, p_col: Vector3f, p_target: Vector3f, p_code: i32) -> Particle {
        Particle {
            position: p_pos,
            velocity: p_vel,
            target: p_target,
            colour: p_col,
            particle_code: p_code
        }
    }

    pub fn empty() -> Particle {
        Particle { 
            position: (Vector3f::empty()), 
            velocity: (Vector3f::empty()), 
            colour: (Vector3f::empty()),
            target: (Vector3f::empty()),
            particle_code: (0) 
        }
    }

    pub fn code_only(p_code: i32) -> Particle {           
        let mut temp = Particle::empty();
        temp.particle_code = p_code;
        return temp;       
    }
}

// Override output
impl Display for Particle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Position: {},\nVelocity: {},\nColour: {},\nParticle Code: {}\n", self.position, self.velocity, self.colour, self.particle_code)
    }
}
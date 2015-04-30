

use std;
use std::hash::{Hash, Hasher };
use std::mem::{transmute};


#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub pos: [f32; 3]
}

impl PartialEq for Vertex {
    fn eq(&self, v: &Vertex) -> bool { 
        (self.pos[0] - v.pos[0]).abs()  <= std::f32::EPSILON && 
        (self.pos[1] - v.pos[1]).abs()  <= std::f32::EPSILON && 
        (self.pos[2] - v.pos[2]).abs()  <= std::f32::EPSILON
    } 
}

impl Eq for Vertex {
}

// Implement Hash since there is no default for f32. We'll just hash the bits
// since we know the f32s will all be canonical from reading.
impl Hash for Vertex {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        let x: u32 = unsafe { transmute(self.pos[0]) };
        x.hash(state);
        let y: u32 = unsafe { transmute(self.pos[1]) };
        y.hash(state);
        let z: u32 = unsafe { transmute(self.pos[2]) };
        z.hash(state); 
    }
}


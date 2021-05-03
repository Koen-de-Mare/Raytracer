use crate::vector::*;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Vector // must be a unit vector
}

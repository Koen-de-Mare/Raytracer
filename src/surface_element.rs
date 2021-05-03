use crate::vector::*;
use crate::material::*;

#[derive(Copy, Clone, Debug)]
pub struct SurfaceElement {
    pub position: Vector,
    pub normal: Vector, // must be a unit vector
    pub material: Material,
}

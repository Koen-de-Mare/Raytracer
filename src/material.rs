use crate::color::*;

#[derive(Copy, Clone, Debug)]
pub struct Material {
    pub diffuse_color: Color,
    pub emmitance_color: Color,
}

use crate::vec3::Vec3;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct DirectionType;

pub type Direction = Vec3<DirectionType>;

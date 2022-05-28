use nalgebra::Vector3;

#[repr(transparent)]
pub struct Normal3<T>(pub(crate) Vector3<T>);

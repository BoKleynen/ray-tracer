use nalgebra::Vector3;

#[repr(transparent)]
pub struct Normal3<T>(pub(crate) Vector3<T>);

impl<T> From<Vector3<T>> for Normal3<T> {
    fn from(v: Vector3<T>) -> Self {
        Normal3(v)
    }
}

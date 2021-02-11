use crate::camera::PerspectiveCamera;
use crate::shape::Shape;

pub struct World {
    shapes: Vec<Box<dyn Shape>>,
    camera: PerspectiveCamera,
}

impl World {
    pub fn camera(&self) -> &PerspectiveCamera {
        &self.camera
    }

    pub fn shapes(&self) -> &[Box<dyn Shape>] {
        self.shapes.as_slice()
    }
}

pub struct WorldBuilder {
    shapes: Vec<Box<dyn Shape>>,
    camera: Option<PerspectiveCamera>,
}

impl WorldBuilder {
    pub fn new() -> Self {
        let shapes = Vec::new();
        let camera = None;

        Self { shapes, camera }
    }

    pub fn add_shape(mut self, shape: Box<dyn Shape>) -> Self {
        self.shapes.push(shape);
        self
    }

    pub fn camera(mut self, camera: PerspectiveCamera) -> Self {
        self.camera = Some(camera);
        self
    }

    pub fn build(self) -> Option<World> {
        let shapes = self.shapes;
        let camera = self.camera?;

        let world = World { shapes, camera };
        Some(world)
    }
}

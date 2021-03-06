use super::*;

// Represent a connection between two nodes
// Contains the coordinate of tangent line and distance
#[derive(Debug)]
pub struct Connection {
    pub neighbor: Rc<RefCell<Vertex>>, // Connected node through a tangent
    pub distance: f32,
    // starting and ending vertices must be above threshold to take the connection
    pub threshold: f32,
}

impl Connection {
    pub fn new(neighbor: Rc<RefCell<Vertex>>, distance: f32, threshold: f32) -> Self {
        Self {
            neighbor,
            distance,
            threshold,
        }
    }
}

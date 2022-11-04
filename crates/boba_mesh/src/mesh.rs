use crate::Vertex;

pub struct BobaMesh<'mesh> {
    vertices: &'mesh [Vertex],
    indices: &'mesh [u16],
    index_length: u32,
}

impl<'a> BobaMesh<'a> {
    pub fn new(vertices: &'a [Vertex], indices: &'a [u16]) -> Self {
        Self {
            vertices,
            indices,
            index_length: indices.len() as u32,
        }
    }

    pub fn vertices(&self) -> &[Vertex] {
        self.vertices
    }

    pub fn indices(&self) -> &[u16] {
        self.indices
    }

    pub fn index_length(&self) -> u32 {
        self.index_length
    }
}

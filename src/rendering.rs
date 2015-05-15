extern crate glium;


#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

pub struct RenderData {
    vertices: glium::VertexBuffer<Vertex>,
    indices:  glium::IndexBuffer,
}

impl RenderData {
    pub fn new<D: glium::backend::Facade, T: glium::index::IntoIndexBuffer>(display: &D, vs: Vec<Vertex>, is: T) -> Self {
        RenderData {
            vertices: glium::VertexBuffer::new(display, vs),
            indices: glium::IndexBuffer::new(display, is),
        }
    }

    pub fn get_vs(&self) -> &glium::VertexBuffer<Vertex> {
        return &self.vertices;
    }

    pub fn get_is(&self) -> &glium::IndexBuffer {
        return &self.indices;
    }
}

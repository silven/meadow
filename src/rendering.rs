extern crate glium;


#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

#[derive(Copy, Clone)]
pub struct PosOnlyVertex {
    pub position: [f32; 3],
}

#[derive(Copy, Clone)]
pub struct GrassAttrs {
    pub offset: [f32; 3],
    pub rand_factor: f32,
}

implement_vertex!(Vertex, position, tex_coords);
implement_vertex!(PosOnlyVertex, position);
implement_vertex!(GrassAttrs, offset, rand_factor);


pub struct RenderData<V: glium::vertex::Vertex + Send + Copy + 'static> {
    vertices: glium::VertexBuffer<V>,
    indices:  glium::IndexBuffer,
}

impl<V: glium::vertex::Vertex + Send + Copy + 'static> RenderData<V> {
    pub fn new<D: glium::backend::Facade, T: glium::index::IntoIndexBuffer>(display: &D, vs: Vec<V>, is: T) -> Self {
        RenderData {
            vertices: glium::VertexBuffer::new(display, vs),
            indices: glium::IndexBuffer::new(display, is),
        }
    }

    pub fn get_vb(&self) -> &glium::VertexBuffer<V> {
        return &self.vertices;
    }

    pub fn get_ib(&self) -> &glium::IndexBuffer {
        &self.indices
    }
}

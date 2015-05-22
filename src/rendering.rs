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
}

implement_vertex!(Vertex, position, tex_coords);
implement_vertex!(PosOnlyVertex, position);
implement_vertex!(GrassAttrs, offset);


enum Indices {
    Buffer(glium::IndexBuffer),
    NoIndices(glium::index::NoIndices),
}


pub struct RenderData<V: glium::vertex::Vertex> {
    vertices: glium::VertexBuffer<V>,
    indices:  Indices,
}

impl<V: glium::vertex::Vertex + Send + Copy + 'static> RenderData<V> {
    pub fn new<D: glium::backend::Facade, T: glium::index::IntoIndexBuffer>(display: &D, vs: Vec<V>, is: T) -> Self {
        RenderData {
            vertices: glium::VertexBuffer::new(display, vs),
            indices: Indices::Buffer(glium::IndexBuffer::new(display, is)),
        }
    }

    pub fn new2<D: glium::backend::Facade>(display: &D, vs: Vec<V>, p: glium::index::PrimitiveType) -> Self {
        RenderData {
            vertices: glium::VertexBuffer::new(display, vs),
            indices: Indices::NoIndices(glium::index::NoIndices(p)),
        }
    }

    pub fn get_vb(&self) -> &glium::VertexBuffer<V> {
        return &self.vertices;
    }

    pub fn get_ib(&self) -> &glium::IndexBuffer {
        match self.indices {
            Indices::Buffer(ref b) => b,
            _ => panic!("Bad choice!"),
        }
    }

    pub fn get_is(&self) -> &glium::index::NoIndices {
        match self.indices {
            Indices::NoIndices(ref p) => p,
            _ => panic!("Bad choice!"),
        }
    }
}

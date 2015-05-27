#![feature(collections)]

use glium;
use glutin;

use rendering::Vertex;

pub struct Terrain {
    vbo: glium::VertexBuffer<Vertex>,
    wireframe: glium::IndexBuffer,
    program: glium::Program,
    level: u16,
}

fn xy_to_index(x: u16, y: u16) -> u16 {
    return (x * 65 + y) as u16;
}

fn p_to_index(p: P) -> u16 {
    return xy_to_index(p.x, p.y);
}

#[derive(Copy, Clone)]
struct P {
    x: u16,
    y: u16,
}

fn p(x: u16, y: u16) -> P {
    P{x: x, y: y}
}

fn p2(p: P, x: u16, y: u16) -> P {
    P{x: p.x+x, y: p.y+y}
}

#[derive(Copy, Clone)]
struct Square {
    top: P,
    w:  u16,
}

fn s(p: P, w: u16) -> Square {
    Square{top: p, w: w}
}


//
//   0   1  2  3  4
//   5   6  7  8  9
//   10 11 12 13 14
//   15 16 17 18 19
//   20 21 22 23 24
//
// =>



fn triangle(a: P, b: P, c: P) -> [u16; 6] {
    return [
        p_to_index(a),
        p_to_index(b),

        p_to_index(b),
        p_to_index(c),

        p_to_index(c),
        p_to_index(a),
    ];
}

fn subdivide(idx: &mut Vec<u16>, level: u16, sq: Square) {

    let size = sq.w/2;
    let tl = sq.top;
    let tr = p(sq.top.x + sq.w, sq.top.y);

    let c  = p(sq.top.x + size, sq.top.y + size);

    let br = p(sq.top.x + sq.w, sq.top.y + sq.w);
    let bl = p(sq.top.x, sq.top.y + sq.w);

    if level > 0 {
        subdivide(idx, level-1, s(p2(tl,    0,    0), size));
        subdivide(idx, level-1, s(p2(tl, size,    0), size));
        subdivide(idx, level-1, s(p2(tl,    0, size), size));
        subdivide(idx, level-1, s(p2(tl, size, size), size));
    } else {
        idx.push_all(&triangle(c, tl, bl));
        idx.push_all(&triangle(c, bl, br));
        idx.push_all(&triangle(c, br, tr));
        idx.push_all(&triangle(c, tr, tl));
    }
}

impl Terrain {

    pub fn new<F: glium::backend::Facade>(display: &F, vertices: Vec<Vertex>, indices: Vec<u16>, program: glium::Program) -> Self {
        Terrain {
            vbo: glium::VertexBuffer::new(display, vertices),
            wireframe: glium::IndexBuffer::new(display, glium::index::LinesList(indices)),
            program: program,
            level: 4,
        }
    }

    pub fn update(&mut self, event: &glutin::Event) {
        match event {
            &glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::T)) => {
                self.level += 1;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::G)) => {
                self.level -= 1;
            },
            _ => {},
        };
    }

    pub fn render<F: glium::backend::Facade, S: glium::Surface, U: glium::uniforms::Uniforms>(&self, display: &F, frame: &mut S, uniforms: &U, params: &glium::DrawParameters) {
        let resolution = 65;
        let mut optimised = Vec::with_capacity((resolution * resolution) as usize);
        subdivide(&mut optimised, self.level, Square{top: p(0, 0), w: resolution-1});

        let indicies = glium::IndexBuffer::new(display, glium::index::LinesList(optimised));

        frame.draw(&self.vbo, &indicies, &self.program, uniforms, params).unwrap();
    }
}



extern crate num;

extern crate rand;

use glium;
use glutin;

use rendering::Vertex;
use rendering::PosOnlyVertex;
use rendering::GrassAttrs;
use rand::Rng;

use heightmap::NoiseContext;

pub struct Terrain {
    terrain_vbo: glium::VertexBuffer<Vertex>,
    grass_vbo: glium::VertexBuffer<PosOnlyVertex>,
    grass_indices: glium::index::NoIndices,
    grass_attrs: glium::VertexBuffer<GrassAttrs>,

    terrain_program: glium::Program,
    grass_program: glium::Program,
    level: usize,
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


fn full_triangle(a: P, b: P, c: P) -> [u16; 3] {
    return [
        p_to_index(a),
        p_to_index(b),
        p_to_index(c),
    ];
}

fn wireframe_triangle(a: P, b: P, c: P) -> [u16; 6] {
    return [
        p_to_index(a),
        p_to_index(b),

        p_to_index(b),
        p_to_index(c),

        p_to_index(c),
        p_to_index(a),
    ];
}

fn subdivide(idx: &mut Vec<u16>, level: usize, sq: Square) {
    let size = sq.w/2;
    let tl = sq.top;
    let tr = p(sq.top.x + sq.w, sq.top.y);

    let br = p(sq.top.x + sq.w, sq.top.y + sq.w);
    let bl = p(sq.top.x, sq.top.y + sq.w);

    if level > 0 {
        subdivide(idx, level-1, s(p2(tl,    0,    0), size));
        subdivide(idx, level-1, s(p2(tl, size,    0), size));
        subdivide(idx, level-1, s(p2(tl,    0, size), size));
        subdivide(idx, level-1, s(p2(tl, size, size), size));
    } else {
        idx.push_all(&full_triangle(tl, bl, br));
        idx.push_all(&full_triangle(tl, tr, br));
    }
}

static MAX_GRASS_PER_SQUARE: usize = 100;
const WORLD_SIZE: u16 = 65;

impl Terrain {

    pub fn new<F: glium::backend::Facade>(display: &F, noise_data: &NoiseContext, terrain_program: glium::Program, grass_program: glium::Program) -> Self {
        let mut vertices = Vec::new();
        let mut attrs = Vec::new();

        let mut grass_jitter = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..MAX_GRASS_PER_SQUARE {
            let jitter = rng.gen::<(f32, f32, f32)>();
            grass_jitter.push(jitter);
        }

        let scale = 1;
        for x in 0..WORLD_SIZE {
            for z in 0..WORLD_SIZE {
                let px = (x * scale) as f32;
                let pz = (z * scale) as f32;

                let height_value = noise_data.get_height(px, pz);

                vertices.push(Vertex {
                    position: [ px, height_value, pz],
                    tex_coords: [px, pz]
                });

                let grass_per = 100;
                for &(jitter_x, jitter_z, personal) in grass_jitter.iter().take(grass_per) {
                    let gx = px + scale as f32 * (jitter_x);
                    let gz = pz + scale as f32 * (jitter_z);
                    let grass_height_value = noise_data.get_height(gx, gz);

                    attrs.push(GrassAttrs { offset: [ gx, grass_height_value, gz ], rand_factor: personal});
                }
            }
        }

        let grass_points = glium::VertexBuffer::new(display, vec![
            PosOnlyVertex { position: [ 0.0, 0.0, 0.0] },
        ]);

        Terrain {
            terrain_vbo: glium::VertexBuffer::new(display, vertices),
            grass_vbo: grass_points,
            grass_indices: glium::index::NoIndices(glium::index::PrimitiveType::Points),
            grass_attrs: glium::VertexBuffer::new(display, attrs),
            terrain_program: terrain_program,
            grass_program: grass_program,
            level: 6,
        }
    }

    pub fn update(&mut self, event: &glutin::Event) {
        //
        match event {
            &glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::T)) => {
                if  num::pow(2, self.level + 1) < (WORLD_SIZE - 1) {
                    self.level += 1;
                }
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::G)) => {
                if self.level > 1 {
                    self.level -= 1;
                }
            },
            _ => {},
        };
    }

    pub fn render<F: glium::backend::Facade, S: glium::Surface, U: glium::uniforms::Uniforms>(&self, display: &F, frame: &mut S, uniforms: &U, params: &glium::DrawParameters) {
        let mut optimised = Vec::with_capacity((WORLD_SIZE * WORLD_SIZE) as usize);
        subdivide(&mut optimised, self.level, Square{top: p(0, 0), w: WORLD_SIZE-1});

        let indicies = glium::IndexBuffer::new(display, glium::index::TrianglesList(optimised));

        frame.draw(&self.terrain_vbo, &indicies, &self.terrain_program, uniforms, params).unwrap();
        frame.draw((&self.grass_vbo, self.grass_attrs.per_instance_if_supported().unwrap()), &self.grass_indices, &self.grass_program, uniforms, params).unwrap();
    }
}


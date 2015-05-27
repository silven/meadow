#![feature(collections)]

extern crate glutin;
extern crate rand;

#[macro_use]
extern crate glium;

extern crate image;
use std::io::Cursor;

use glium::Surface;
use std::f32;

mod support;
mod programs;
mod rendering;
mod heightmap;
mod terrain;
mod skybox;

use rendering::Vertex;
use rendering::PosOnlyVertex;
use rendering::GrassAttrs;
use rand::Rng;

use glium::backend::Facade;


extern crate cgmath;
use cgmath::FixedArray;
use glium::draw_parameters::LinearBlendingFactor;

static NOISE_SAMPLES: usize = 128;
static MAX_GRASS_PER: usize = 100;

fn main() {
    use glium::DisplayBuild;

    // building the display, ie. the main object
    let display = glium::glutin::WindowBuilder::new()
        .with_depth_buffer(24)
        .with_dimensions(1920, 1080)
        .with_title(format!("Meadow"))
        .build_glium()
        .unwrap();

    // building a texture with "OpenGL" drawn on it
    let image = image::load(Cursor::new(&include_bytes!("textures/opengl.png")[..]), image::PNG).unwrap();
    let opengl_texture = glium::texture::CompressedTexture2d::new(&display, image);

    let noise_data = heightmap::noise(NOISE_SAMPLES);

    let grass_png = image::load(Cursor::new(&include_bytes!("textures/grass.png")[..]), image::PNG).unwrap();
    let grass_texture = glium::texture::CompressedTexture2d::new(&display, grass_png);

    let mask_png = image::load(Cursor::new(&include_bytes!("textures/grass_mask.png")[..]), image::PNG).unwrap();
    let mask_texture = glium::texture::CompressedTexture2d::new(&display, mask_png);

    let mut terrain_data = Vec::new();
    let mut attrs = vec![];

    let resolution = 65usize;
    let scale = 1usize;

    let mut grass_jitter = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..MAX_GRASS_PER {
        let jitter = rng.gen::<(f32, f32, f32)>();
        grass_jitter.push(jitter);
    }

    for x in 0..resolution {
        for z in 0..resolution {
            let fx = x as f32 / (resolution) as f32;
            let fz = z as f32 / (resolution) as f32;

            let px = (x * scale) as f32;
            let pz = (z * scale) as f32;

            let height_value = noise_data.get_height(px, pz);

            terrain_data.push(Vertex {
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

    let mut index_data = Vec::new();
    let mut wireframe = Vec::new();
    for i in 0..(resolution-1) {
        for j in 0..(resolution-1) {
            // Triangle list
            let i1 = i     * resolution + j;
            let i2 = (i+1) * resolution + j;
            let i3 = i     * resolution + j+1;
            let i4 = (i+1) * resolution + j+1;
            index_data.push(i1 as u16);
            index_data.push(i2 as u16);
            index_data.push(i3 as u16);
            index_data.push(i2 as u16);
            index_data.push(i3 as u16);
            index_data.push(i4 as u16);

            wireframe.push(i1 as u16);
            wireframe.push(i2 as u16);
            wireframe.push(i1 as u16);
            wireframe.push(i3 as u16);
            wireframe.push(i2 as u16);
            wireframe.push(i3 as u16);
            wireframe.push(i2 as u16);
            wireframe.push(i4 as u16);
            wireframe.push(i3 as u16);
            wireframe.push(i4 as u16);
        }
    }

    //let terrain_vbo = glium::VertexBuffer::new(&display, terrain_data);
    //let terrain_indicies = glium::IndexBuffer::new(&display,glium::index::TrianglesList(index_data));
    //let terrain_wireframe = glium::IndexBuffer::new(&display, glium::index::LinesList(wireframe));

    let pm = programs::ProgramManager::new();
    let mut program = pm.create(&display, &programs::ShaderBundle::new("simple.vs", "terrain.fs", None, None, None)).unwrap();

    let mut t = terrain::Terrain::new(&display, terrain_data, wireframe, program);

    let grass_points = glium::VertexBuffer::new(&display, vec![
        PosOnlyVertex { position: [ 0.0,  0.0, 0.0] },
    ]);
    let grass_indices = glium::index::NoIndices(glium::index::PrimitiveType::Points);
    let grass_attrs = glium::VertexBuffer::new(&display, attrs);

    let quad = rendering::RenderData::new(&display,
            vec![
                Vertex { position: [-1.0, -1.0, 0.0], tex_coords: [0.0, 0.0] },
                Vertex { position: [1.0, -1.0, 0.0], tex_coords: [1.0, 0.0] },
                Vertex { position: [1.0, 1.0, 0.0], tex_coords: [1.0, 1.0] },
                Vertex { position: [-1.0, 1.0, 0.0], tex_coords: [0.0, 1.0] },
            ],
            glium::index::TrianglesList(vec![0u16, 1, 2, 0, 2, 3])
        );

    let (w, h) = display.get_window().unwrap().get_inner_size().unwrap();
    let mut camera = support::camera::CameraState::new(w, h);

    let texture1 = glium::texture::Texture2d::new_empty(&display, glium::texture::UncompressedFloatFormat::F32F32F32F32, w, h);
    let depthtexture = glium::texture::DepthTexture2d::new_empty(&display, glium::texture::DepthFormat::F32, w, h);
    let output = &[("output1", &texture1)];
    let mut framebuffer = glium::framebuffer::MultiOutputFrameBuffer::with_depth_buffer(&display, output, &depthtexture);

    // compiling shaders and linking them together

    let mut composition_program = pm.create(&display, &programs::ShaderBundle::new("null.vs", "simple.fs", None, None, None)).unwrap();
    let mut grass_program = pm.create(&display, &programs::ShaderBundle::new("grass.vs", "grass.fs", Some("grass.gs"), None, None)).unwrap();


    let default_blending = Some(glium::BlendingFunction::Addition { source: LinearBlendingFactor::SourceAlpha, destination: LinearBlendingFactor::OneMinusSourceAlpha });

    let mut wireframe_mode = true;

    // the main loop
    let mut tick_number = 0;
    support::start_loop(|| {
        tick_number += 1;
        camera.update(&noise_data);

        // building the uniforms
        let uniforms = uniform! {
            // Camera uniforms
            persp_matrix: camera.get_perspective().into_fixed(),
            view_matrix: camera.get_view().into_fixed(),

            windforce: cgmath::vec3((tick_number as f32 / 100.0).sin() / 2.0, 0.0, 0.0).into_fixed(),
            //texture_unit: &noise_texture,
            texture_unit: &grass_texture,
            grass_texture_unit: &grass_texture,
            mask_texture_unit: &mask_texture
        };

        // draw parameters
        let mut params = glium::DrawParameters {
            depth_test: glium::DepthTest::IfLess,
            depth_write: true,
            backface_culling: glium::BackfaceCullingMode::CullingDisabled,

            .. std::default::Default::default()
        };

        // Skybox is rendered first?

        // First pass rendering
        framebuffer.clear_color_and_depth((0.8, 0.95, 0.99, 0.0), 1.0);

        skybox::render();

        t.render(&display, &mut framebuffer, &uniforms, &params);

        //framebuffer.draw((&grass_points, grass_attrs.per_instance_if_supported().unwrap()), &grass_indices, &grass_program, &uniforms, &params).unwrap();



        // Final rendering
        let composition_uniforms = uniform! {
            texture_unit: &texture1,
        };

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
        target.draw(quad.get_vb(), quad.get_ib(), &composition_program, &composition_uniforms, &std::default::Default::default()).unwrap();
        target.finish();

        // polling and handling the events received by the window
        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => return support::Action::Stop,

                glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::Key1)) => {
                    wireframe_mode = true;
                },

                glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::Key2)) => {
                    wireframe_mode = false;
                },

                ev => {
                    t.update(&ev);
                    camera.process_input(&display.get_window().unwrap(), &ev);
                },
            }
        }
        support::Action::Continue
    });
}

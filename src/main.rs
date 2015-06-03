#![feature(collections)]

#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate image;
extern crate glutin;
extern crate rand;

mod support;
mod programs;
mod rendering;
mod heightmap;
mod terrain;

use rendering::Vertex;
use std::io::Cursor;
use glium::Surface;
use cgmath::FixedArray;

static NOISE_SAMPLES: usize = 128;


fn main() {
    use glium::DisplayBuild;

    let display = glium::glutin::WindowBuilder::new()
        .with_depth_buffer(24)
        .with_dimensions(1920, 1080)
        .with_title(format!("Meadow"))
        .build_glium()
        .unwrap();

    let noise_data = heightmap::NoiseContext::new(NOISE_SAMPLES);

    // Load textures from disk
    let grass_png = image::load(Cursor::new(&include_bytes!("textures/grass.png")[..]), image::PNG).unwrap();
    let grass_texture = glium::texture::CompressedTexture2d::new(&display, grass_png);

    let mask_png = image::load(Cursor::new(&include_bytes!("textures/grass_mask.png")[..]), image::PNG).unwrap();
    let mask_texture = glium::texture::CompressedTexture2d::new(&display, mask_png);

    let pm = programs::ProgramManager::new();
    let terrain_program = pm.create(&display, &programs::ShaderBundle::new("simple.vs", "terrain.fs", None, None, None)).unwrap();
    let grass_program = pm.create(&display, &programs::ShaderBundle::new("grass.vs", "grass.fs", Some("grass.gs"), None, None)).unwrap();

    let mut terrain = terrain::Terrain::new(&display, &noise_data, terrain_program, grass_program);

    let (w, h) = display.get_window().unwrap().get_inner_size().unwrap();
    let mut camera = support::camera::CameraState::new(w, h);

    // Setup deferred rendering
    let quad = rendering::RenderData::new(&display,
            vec![
                Vertex { position: [-1.0, -1.0, 0.0], tex_coords: [0.0, 0.0] },
                Vertex { position: [1.0, -1.0, 0.0], tex_coords: [1.0, 0.0] },
                Vertex { position: [1.0, 1.0, 0.0], tex_coords: [1.0, 1.0] },
                Vertex { position: [-1.0, 1.0, 0.0], tex_coords: [0.0, 1.0] },
            ],
            glium::index::TrianglesList(vec![0u16, 1, 2, 0, 2, 3])
        );

    let texture1 = glium::texture::Texture2d::new_empty(&display, glium::texture::UncompressedFloatFormat::F32F32F32F32, w, h);
    let depthtexture = glium::texture::DepthTexture2d::new_empty(&display, glium::texture::DepthFormat::F32, w, h);
    let output = &[("output1", &texture1)];
    let mut framebuffer = glium::framebuffer::MultiOutputFrameBuffer::with_depth_buffer(&display, output, &depthtexture);
    let composition_program = pm.create(&display, &programs::ShaderBundle::new("null.vs", "simple.fs", None, None, None)).unwrap();

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

            // Calculate the wind somehow
            windforce: cgmath::vec3((tick_number as f32 / 100.0).sin() / 2.0, 0.0, 0.0).into_fixed(),

            texture_unit: &grass_texture,
            grass_texture_unit: &grass_texture,
            mask_texture_unit: &mask_texture
        };

        // draw parameters
        let params = glium::DrawParameters {
            depth_test: glium::DepthTest::IfLess,
            depth_write: true,
            backface_culling: glium::BackfaceCullingMode::CullingDisabled,

            .. std::default::Default::default()
        };

        // First pass rendering
        framebuffer.clear_color_and_depth((0.8, 0.95, 0.99, 0.0), 1.0);
        terrain.render(&display, &mut framebuffer, &uniforms, &params);

        // Final rendering to quad
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
                ev => {
                    terrain.update(&ev);
                    camera.process_input(&display.get_window().unwrap(), &ev);
                },
            }
        }
        support::Action::Continue
    });
}

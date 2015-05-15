extern crate glutin;

#[macro_use]
extern crate glium;

extern crate image;
use std::io::Cursor;

use glium::Surface;

mod support;
mod programs;
mod rendering;
use rendering::Vertex;


extern crate cgmath;
use cgmath::FixedArray;


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
    let image = image::load(Cursor::new(&include_bytes!("textures/opengl.png")[..]),
        image::PNG).unwrap();
    let opengl_texture = glium::texture::CompressedTexture2d::new(&display, image);

    let render_objects = vec![
        rendering::RenderData::new(&display,
            vec![
                Vertex { position: [ 0.0, 0.0, 0.0], tex_coords: [1.0, 0.0] },
                Vertex { position: [ 0.0, 0.0, 10.0], tex_coords: [1.0, 1.0] },
                Vertex { position: [ 10.0, 0.0, 10.0], tex_coords: [0.0, 1.0] },
                Vertex { position: [ 10.0, 0.0, 0.0], tex_coords: [0.0, 0.0] },
            ],
            glium::index::TriangleStrip(vec![1 as u16, 0, 2, 3]),
        ),
        rendering::RenderData::new(&display,
            vec![
            Vertex { position: [ 5.0, -5.0, 5.0], tex_coords: [0.0, 1.0] },
            Vertex { position: [ 0.0,  5.0, 5.0], tex_coords: [1.0, 1.0] },
            Vertex { position: [-5.0, -5.0, 5.0], tex_coords: [0.0, 0.0] },
            ],
            glium::index::TrianglesList(vec![0u16, 1, 2]),
        ),
    ];

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
    let pm = programs::ProgramManager::new();
    //let simple = programs::ShaderBundle::new("simple.vs", "simple.fs", Some("simple.gs"), Some("simple.tc"), Some("simple.te"));
    let simple = programs::ShaderBundle::new("simple.vs", "simple.fs", None, None, None);
    let composition = programs::ShaderBundle::new("null.vs", "simple.fs", None, None, None);

    let mut program = pm.create(&display, &simple).unwrap();
    let mut composition_program = pm.create(&display, &composition).unwrap();


    // the main loop
    support::start_loop(|| {
        camera.update();

        // building the uniforms
        let uniforms = uniform! {
            // Camera uniforms
            persp_matrix: camera.get_perspective().into_fixed(),
            view_matrix: camera.get_view().into_fixed(),

            texture_unit: &opengl_texture,
        };

        // draw parameters
        let params = glium::DrawParameters {
            depth_test: glium::DepthTest::IfLess,
            depth_write: true,
            backface_culling: glium::BackfaceCullingMode::CullCounterClockWise,
            .. std::default::Default::default()
        };

        // First pass rendering
        framebuffer.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
        for ro in render_objects.iter() {
            framebuffer.draw(ro.get_vs(), ro.get_is(), &program, &uniforms, &params).unwrap();
        }



        // Final rendering
        let composition_uniforms = uniform! {
            texture_unit: &texture1,
        };

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
        target.draw(quad.get_vs(), quad.get_is(), &composition_program, &composition_uniforms, &std::default::Default::default()).unwrap();
        target.finish();

        // polling and handling the events received by the window
        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => return support::Action::Stop,

                glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::R)) => {
                    match pm.create(&display, &simple) {
                        Ok(p) => {
                            program = p;
                            println!("Successfully recompiled program");
                        },
                        Err(msg) => println!("Could not recompile program: '{:?}'", msg)
                    };
                },
                ev => camera.process_input(&display.get_window().unwrap(), &ev),
            }
        }
        support::Action::Continue
    });
}

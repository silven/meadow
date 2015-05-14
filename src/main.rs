extern crate glutin;

#[macro_use]
extern crate glium;

extern crate image;
use std::io::Cursor;

use glium::Surface;

mod support;
mod programs;

extern crate cgmath;
use cgmath::FixedArray;


#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

fn main() {
    use glium::DisplayBuild;

    // building the display, ie. the main object
    let display = glium::glutin::WindowBuilder::new()
        .with_depth_buffer(24)
        .with_dimensions(800, 500)
        .with_title(format!("Glium Deferred Example"))
        .build_glium()
        .unwrap();

    // building a texture with "OpenGL" drawn on it
    let image = image::load(Cursor::new(&include_bytes!("textures/opengl.png")[..]),
        image::PNG).unwrap();
    let opengl_texture = glium::texture::CompressedTexture2d::new(&display, image);

    // building the vertex buffer, which contains all the vertices that we will draw
    let plane = glium::VertexBuffer::new(&display,
        vec![
            Vertex { position: [ 0.0, 0.0, 0.0], tex_coords: [1.0, 0.0] },
            Vertex { position: [ 0.0, 0.0, 10.0], tex_coords: [1.0, 1.0] },
            Vertex { position: [ 10.0, 0.0, 10.0], tex_coords: [0.0, 1.0] },
            Vertex { position: [ 10.0, 0.0, 0.0], tex_coords: [0.0, 0.0] },
        ]
    );

    let vertex_buffer_2 = glium::VertexBuffer::new(&display,
        vec![
            Vertex { position: [ 5.0, -5.0, 5.0], tex_coords: [0.0, 1.0] },
            Vertex { position: [ 0.0,  5.0, 5.0], tex_coords: [1.0, 1.0] },
            Vertex { position: [-5.0, -5.0, 5.0], tex_coords: [0.0, 0.0] },
        ]
    );

    // building the index buffer
    let plane_index_buffer = glium::IndexBuffer::new(&display,
        glium::index::TriangleStrip(vec![1 as u16, 0, 2, 3]));

    let index_buffer = glium::IndexBuffer::new(&display,
        glium::index::TrianglesList(vec![0u16, 1, 2]));

    // compiling shaders and linking them together
    let mut pm = programs::ProgramManager::new();
    //let simple = programs::ShaderBundle::new("simple.vs", "simple.fs", Some("simple.gs"), Some("simple.tc"), Some("simple.te"));
    let simple = programs::ShaderBundle::new("simple.vs", "simple.fs", None, None, None);
    let mut program = pm.create(&display, &simple).unwrap();

    // level of tessellation
    let mut tess_level_inner: i32 = 5;
    let mut tess_level_outer: i32 = 5;
    println!("The current tessellation levels are {}/{} ; use the arrow keys to change them!", tess_level_inner, tess_level_outer);

    let (w, h) = display.get_window().unwrap().get_inner_size().unwrap();
    let mut camera = support::camera::CameraState::new(w, h);

    // the main loop
    support::start_loop(|| {
        camera.update();

        // building the uniforms
        let uniforms = uniform! {
            tess_level_inner: tess_level_outer,
            tess_level_outer: tess_level_inner,
            // Camera uniforms
            persp_matrix: camera.get_perspective().into_fixed(),
            view_matrix: camera.get_view().into_fixed(),

            texture: &opengl_texture,
        };

        // draw parameters
        let params = glium::DrawParameters {
            depth_test: glium::DepthTest::IfLess,
            depth_write: true,
            backface_culling: glium::BackfaceCullingMode::CullCounterClockWise,
            .. std::default::Default::default()
        };

        // drawing a frame
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
        target.draw(&plane, &plane_index_buffer, &program, &uniforms, &params).unwrap();
        target.draw(&vertex_buffer_2, &index_buffer, &program, &uniforms, &params).unwrap();
        target.finish();

        // polling and handling the events received by the window
        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => return support::Action::Stop,
                // Up and down alters the inner tessellation level
                glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::Up)) => {
                    tess_level_inner += 1;
                },
                glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::Down)) => {
                    if tess_level_inner >= 2 {
                        tess_level_inner -= 1;
                    }
                },

                // Left and Right alters the outer tessellation level
                glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::Right)) => {
                    tess_level_outer += 1;
                },
                glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::Left)) => {
                    if tess_level_outer >= 2 {
                        tess_level_outer -= 1;
                    }
                },

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

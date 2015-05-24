extern crate glutin;
extern crate cgmath;

use self::cgmath::perspective;
use self::cgmath::Matrix4;
use self::cgmath::Point;
use self::cgmath::Point3;
use self::cgmath::Vector;
use self::cgmath::Vector3;
use self::cgmath::Quaternion;
use self::cgmath::Rotation3;
use self::cgmath::{rad, deg};
use self::cgmath::EuclideanVector;

pub struct CameraState {
    fov: f32,
    aspect_ratio: f32,
    position: Point3<f32>,
    direction: Vector3<f32>,

    mouse_pressed: bool,
    mouse_x: i32,
    mouse_y: i32,

    moving_left: bool,
    moving_right: bool,
    moving_forward: bool,
    moving_backward: bool,
}

impl CameraState {
    pub fn new(width: u32, height: u32) -> CameraState {
        CameraState {
            fov: 45.0,
            aspect_ratio: width as f32 / height as f32,

            position: Point3{x: 5.0, y: 5.0, z: 5.0},
            direction: Vector3{x: 1.0, y: 0.0, z: 1.0}.normalize(),

            mouse_pressed: false,
            mouse_x: 0,
            mouse_y: 0,

            moving_left: false,
            moving_right: false,
            moving_forward: false,
            moving_backward: false,
        }
    }

    pub fn get_perspective(&self) -> Matrix4<f32> {
        let zfar = 100.0;
        let znear = 0.1;
        return perspective(deg(self.fov), self.aspect_ratio, znear, zfar);
    }

    pub fn get_view(&self) -> Matrix4<f32> {
        let point_to_look_at: Point3<f32> = self.position.add_v(&self.direction);
        return Matrix4::look_at(&self.position, &point_to_look_at, &Vector3::unit_y());
    }

    pub fn update(&mut self, heightmap: &super::super::heightmap::NoiseContext) {
        let speed = 0.1;

        self.direction = self.direction.normalize();
        let left = Vector3::unit_y().cross(&self.direction);

        if self.moving_left {
            self.position.add_self_v(&left.mul_s(speed));
        }

        if self.moving_right {
            self.position.add_self_v(&left.mul_s(-speed));
        }

        if self.moving_forward {
            self.position.add_self_v(&self.direction.mul_s(speed));
        }

        if self.moving_backward {
            self.position.add_self_v(&self.direction.mul_s(-speed));
        }

        self.position.y = 2.0 + heightmap.get_height(self.position.x, self.position.z);

    }

    pub fn process_input(&mut self, window: &glutin::Window, event: &glutin::Event) {
        match event {

            &glutin::Event::KeyboardInput(state, _, Some(glutin::VirtualKeyCode::A)) => {
                self.moving_left = match state {
                    glutin::ElementState::Pressed => true,
                    glutin::ElementState::Released => false,
                };
            },

            &glutin::Event::KeyboardInput(state, _, Some(glutin::VirtualKeyCode::D)) => {
                self.moving_right = match state {
                    glutin::ElementState::Pressed => true,
                    glutin::ElementState::Released => false,
                };
            },

            &glutin::Event::KeyboardInput(state, _, Some(glutin::VirtualKeyCode::W)) => {
                self.moving_forward = match state {
                    glutin::ElementState::Pressed => true,
                    glutin::ElementState::Released => false,
                };
            },

            &glutin::Event::KeyboardInput(state, _, Some(glutin::VirtualKeyCode::S)) => {
                self.moving_backward = match state {
                    glutin::ElementState::Pressed => true,
                    glutin::ElementState::Released => false,
                };
            },

            &glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::P)) => {
                self.fov += 1.0;
            },

            &glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::O)) => {
                self.fov -= 1.0;
            },

            &glutin::Event::MouseInput(state, _) => {
                self.mouse_pressed = match state {
                    glutin::ElementState::Pressed => true,
                    glutin::ElementState::Released => false,
                };
            },

            &glutin::Event::Resized(width, height) => {
                self.aspect_ratio = width as f32 / height as f32;
            },

            &glutin::Event::MouseMoved((x, y)) => {
                if self.mouse_pressed {
                    let (w, h) = window.get_inner_size().unwrap();
                    let dx = -(x - self.mouse_x) as f32 / w as f32;
                    let dy = (y - self.mouse_y) as f32 / h as f32;

                    let rot_x: Quaternion<f32> = Rotation3::from_angle_y(rad(dx / 1.0));
                    let rot_y: Quaternion<f32> = Rotation3::from_angle_x(rad(dy / 1.0));
                    let rot = rot_x.mul_q(&rot_y);

                    self.direction = rot.mul_v(&self.direction);
                }

                self.mouse_x = x;
                self.mouse_y = y;
            },
            _ => {}
        }
    }
}

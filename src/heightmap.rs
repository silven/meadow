extern crate cgmath;
use cgmath::Vector2;

extern crate rand;
use rand::Rng;

use std::f32::consts::PI;

fn lerp(a: f32, b: f32, f: f32) -> f32 {
    a * (1.0 - f) + b * f
}

fn smooth(v: f32) -> f32 {
    v * v * (3.0 - 2.0 * v)
}

fn random_gradient<R: Rng>(r: &mut R) -> Vector2<f32> {
    let v = PI * 2.0 * r.gen::<f32>();
    Vector2 { x: v.cos(), y: v.sin() }
}

fn gradient(orig: Vector2<f32>, grad: Vector2<f32>, p: Vector2<f32>) -> f32 {
    (p.x - orig.x) * grad.x + (p.y - orig.y) * grad.y
}

pub struct NoiseContext {
    rgradients: Vec<Vec<Vector2<f32>>>,
}

impl NoiseContext {
    fn new(size: usize) -> Self {
        let mut rng = rand::thread_rng();

        let mut grad_data = Vec::with_capacity(size + 1);
        for x in 0..(size+1) {
            let mut row_data = Vec::with_capacity(size + 1);
            for y in 0..(size+1) {
                let gradient = random_gradient(&mut rng);
                row_data.push(gradient);
            }
            grad_data.push(row_data);
        }

        return NoiseContext {
            rgradients: grad_data,
        };
    }

    fn get_gradients(&self, x: f32, y: f32) -> ([Vector2<f32>; 4], [Vector2<f32>; 4]) {
        let x0f = x.floor();
        let y0f = y.floor();
        let x1f = x0f + 1.0;
        let y1f = y0f + 1.0;

        let x0 = x0f as i32;
        let y0 = y0f as i32;
        let x1 = x0 + 1;
        let y1 = y0 + 1;

        ([self.get_gradient(x0, y0), self.get_gradient(x1, y0),
          self.get_gradient(x0, y1), self.get_gradient(x1, y1)],
         [Vector2 { x: x0f, y: y0f }, Vector2 { x: x1f, y: y0f },
          Vector2 { x: x0f, y: y1f }, Vector2 { x: x1f, y: y1f }])
    }

    fn get_gradient(&self, x: i32, y: i32) -> Vector2<f32> {
        //let yidx = self.permutations[y as usize];
        //let xidx = self.permutations[x as usize];
        return self.rgradients[y as usize][x as usize];
    }

    pub fn get(&self, x: f32, y: f32) -> f32 {
        let p = Vector2 {x: x, y: y};
        let (gradients, origins) = self.get_gradients(x, y);

        let v0 = gradient(origins[0], gradients[0], p);
        let v1 = gradient(origins[1], gradients[1], p);
        let v2 = gradient(origins[2], gradients[2], p);
        let v3 = gradient(origins[3], gradients[3], p);

        let fx = smooth(x - origins[0].x);
        let vx0 = lerp(v0, v1, fx);
        let vx1 = lerp(v2, v3, fx);
        let fy = smooth(y - origins[0].y);

        lerp(vx0, vx1, fy)
    }
}

pub fn noise(samples: usize) -> NoiseContext {
    return NoiseContext::new(samples);
}

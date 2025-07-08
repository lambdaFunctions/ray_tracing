use sdl2::rect::Point;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

static RAY_CONTINUITY: f32 = 20.0;
static RAY_LIMIT: f32 = 700.0; // TODO: Make it a % of screen length.
static ANGLE_SHIFT: f32 = 0.10;

pub struct Ray {
    pub color: Color,
    pub origin: (f32, f32), // x and y pair
    pub angle: f32,
    pub end: (f32, f32),
    pub points: Vec<(f32, f32)>,
}

impl Ray {
    pub fn new(color: Color, position: &(f32, f32), angle: f32) -> Ray {
        let end: (f32, f32) = Self::get_ray_end(angle, &position);
        let points: Vec<(f32, f32)> = Self::get_points(angle, &position, end);

        Ray {
            color: color,
            origin: (position.0, position.1),
            angle: angle,
            end: end,
            points: points,
        }
    }

    fn get_ray_x_end(position: &(f32, f32)) -> f32 {
        RAY_LIMIT + position.0
    }

    fn get_ray_end(angle: f32, position: &(f32, f32)) -> (f32, f32) {
        let x: f32 = Self::get_ray_x_end(&position);
        let y: f32 = angle * (RAY_LIMIT - position.0) + position.1;

        (x, y)
    }

    fn get_points(angle: f32, position: &(f32, f32), end: (f32, f32)) -> Vec<(f32, f32)> {
        let mut points: Vec<(f32, f32)> = vec![];

        let mut x: f32 = position.0;

        while x <= end.0 {
            x += RAY_CONTINUITY;
            let y: f32 = angle * (x - &position.0) + position.1;
            
            points.push((x, y));
        }
        points
    }
    
    pub fn render(&self, canvas: &mut Canvas<Window>) {
        for point in &self.points {
            let _ = canvas.draw_point(
                Point::new(point.0 as i32, point.1 as i32)
            );
        }
    }
}


use sdl2::rect::Point;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Body {
    pub color: Color,
    pub position: (f32, f32),
    pub radius: f32,
    pub coordenates: Vec<(f32, f32)>,
    pub center: (f32, f32),
    pub glow: Option<bool>,
}

impl Body {
    pub fn new(
        color: Color,
        position: (f32, f32),
        radius: f32,
        glow: Option<bool>,
    ) -> Body {
        let center: (f32, f32) = (position.0, position.1);

        let coordenates: Vec<(f32, f32)> = Self::get_coordenates(radius, center);

        Body {
            color: color,
            position: position,
            radius: radius,
            coordenates: coordenates,
            center: center,
            glow: glow,
        }
    }

    fn get_coordenates(radius: f32, center: (f32, f32)) -> Vec<(f32, f32)> {
        let (cx, cy) = center;

        let mut coordenates: Vec<(f32, f32)> = vec![];

        for y in -radius as i32..=radius as i32{
            let y_pwr: f32 = y.pow(2) as f32;
            let x_span = (radius.powf(2.0) - y_pwr).sqrt();

            for x in -x_span as i32..=x_span as i32 {
                coordenates.push((cx + x as f32, cy + y as f32))
            }
        }
        coordenates
        // let mut coordenates: Vec<(f32, f32)> = vec![];
    
        // for degree in 0..360 {
        //     let rad = (degree as f64).to_radians();
        //     let x = cx + (rad.cos() * radius as f64) as f32;
        //     let y = cy + (rad.sin() * radius as f64) as f32;

        //     coordenates.push((x, y));
        // }
        // coordenates
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        for (x, y) in &self.coordenates {
            canvas.set_draw_color(self.color);
            self.draw_pixel(canvas, (*x, *y));
        }

        if self.glow.is_some() {
            self.render_glow(canvas, self.center);
        }

        self.fill_color(canvas, self.center, self.radius, self.color);
    }

    fn render_glow(&self, canvas: &mut Canvas<Window>, center: (f32, f32)) {
        let tones: Vec<(u8, u8, u8)> = vec![
            (26, 26, 26),
            (51, 51, 51),
            (77, 77, 77),
            (102, 102, 102),
            (128, 128, 128),
            (153, 153, 153),
            (179, 179, 179),
            (204, 204, 204),
            (230, 230, 230),
        ];

        let delta: f32 = 3.0;
        let mut radius: f32 = self.radius.clone() * 3.0;

        for tone in tones {
            self.fill_color(canvas, center, radius, Color::RGB(tone.0, tone.1, tone.2));
            radius -= delta;
        } 
    }

    fn fill_color(
        &self,
        canvas: &mut Canvas<Window>,
        center: (f32, f32),
        radius: f32,
        color: Color,
    ) {
        let (cx, cy) = center;
        canvas.set_draw_color(color);
    
        for y in -radius as i32..=radius as i32{
            let y_pwr: f32 = y.pow(2) as f32;
            let x_span = (radius.powf(2.0) - y_pwr).sqrt();

            for x in -x_span as i32..=x_span as i32 {
                let _ = canvas.draw_point(
                    Point::new((cx + x as f32) as i32, (cy + y as f32) as i32)
                );
            }
        }
    }

    fn draw_pixel(&self, canvas: &mut Canvas<Window>, coordenates: (f32, f32)) {
        let _ = canvas.draw_point(
            Point::new(coordenates.0 as i32, coordenates.1 as i32)
        );
    }

    pub fn draw_lighted_pixels(
        &self, canvas: &mut Canvas<Window>, collision_coordenates: Vec<(f32, f32)>
    ) {
        let pink: Color = Color::RGB(255, 0, 127);
        let radius: f32 = 10.0;

        for collision_coordenate in collision_coordenates {
            // let light_coordenates: Vec<(f32, f32)> = Self::get_coordenates(
            //     radius, collision_coordenate
            // );

            self.fill_color(
                canvas,
                collision_coordenate,
                radius,
                pink,
            );
        }
    }

    pub fn draw_lighted_pixels2(
        &self, canvas: &mut Canvas<Window>, collision_coordenates: (f32, f32)
    ) {
        // let light_orange: Color = Color::RGB(255, 155, 127);
        // let light_blue: Color = Color::RGB(52, 204, 235);
        // let pink: Color = Color::RGB(255, 0, 127);

        // TODO: O raio do circulo que fara sombra sobre o corpo devera ser
        // baseado na distancia da fonte de luz ate o objeto.
        let radius: f32 = 10.0;

        let light_coordenates: Vec<(f32, f32)> = Self::get_coordenates(
            radius, collision_coordenates
        );

        // self.fill_color(
        //     canvas,
        //     collision_coordenates,
        //     radius,
        //     pink,
        // );

        // for body_coordenate in Self::get_coordenates(self.radius, self.position) {
        for body_coordenate in &self.coordenates {
            for light_coordenate in &light_coordenates {
                if body_coordenate.0 <= light_coordenate.0 
                && body_coordenate.1 >= light_coordenate.1 - radius
                && body_coordenate.1 <= light_coordenate.1 + radius
                // && body_coordenate.1 >= (collision_coordenates.1 - radius) && body_coordenate.1 <= (collision_coordenates.1 + radius)
                {
                    self.draw_pixel(canvas, (body_coordenate.0, body_coordenate.1));
                }
            }
        }

        // for coordenate in Self::get_inner_coordenates(self.radius, self.position) {
        //     if (coordenate.0 >= collision_coordenates.0 && coordenate.0 <= (collision_coordenates.0 + radius))
        //     && (coordenate.1 >= (collision_coordenates.1 - radius) && coordenate.1 <= (collision_coordenates.1 + radius))
        //     {
        //         self.draw_pixel(canvas, (coordenate.0, coordenate.1));
        //     }
        // }
    }
}


use sdl2::rect::Point;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

static GLOW_FACTOR: f32 = 1.3;

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

    fn get_inner_coordenates(radius: f32, center: (f32, f32)) -> Vec<(f32, f32)> {
        let (cx, cy) = center;
        let mut inner_coordenates: Vec<(f32, f32)> = vec![];

        for y in -radius as i32..=radius as i32{
            let y_pwr: f32 = y.pow(2) as f32;
            let x_span = (radius.powf(2.0) - y_pwr).sqrt();

            for x in -x_span as i32..=x_span as i32 {
                inner_coordenates.push(
                    (cx + x as f32, cy + y as f32)
                )
            }
        }
        inner_coordenates
    }

    fn get_coordenates(radius: f32, center: (f32, f32)) -> Vec<(f32, f32)> {
        let (cx, cy) = center;

        let mut coordenates: Vec<(f32, f32)> = vec![];
    
        for degree in 0..360 {
            let rad = (degree as f64).to_radians();
            let x = cx + (rad.cos() * radius as f64) as f32;
            let y = cy + (rad.sin() * radius as f64) as f32;

            coordenates.push((x, y));
        }
        coordenates
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        for (x, y) in &self.coordenates {
            canvas.set_draw_color(self.color);
            let _ = canvas.draw_point(Point::new(*x as i32, *y as i32));
        }

        if self.glow.is_some() {
            self.render_glow(canvas, self.center);
        }

        self.fill_color(canvas, self.center, self.radius, self.color);
        // TODO: Remover esse `.clone` para melhorar a performance.
        // self.fill_color(canvas, self.coordenates.clone());
    }

    fn render_glow(&self, canvas: &mut Canvas<Window>, center: (f32, f32)) {
        let (cx, cy) = center;

        let tones: Vec<(u8, u8, u8)> = vec![
            (230, 230, 230),
            (204, 204, 204),
            (179, 179, 179),
            (153, 153, 153),
            (128, 128, 128),
            (102, 102, 102),
            (77, 77, 77),
            (51, 51, 51),
            (26, 26, 26),
        ];

        for tone in tones {
            let mut coordenates: Vec<(f32, f32)> = vec![];

            let color: Color = Color::RGB(tone.0, tone.1, tone.2);

            canvas.set_draw_color(color);

            let mut cur_radius: f32 = self.radius;
            let mut prev_radius: f32 = self.radius;
            let mut prev_x: f32 = 0.0;
            let mut prev_y: f32 = 0.0;
           
            for degree in 0..360 {
                let rad = (degree as f64).to_radians();
                cur_radius = cur_radius * GLOW_FACTOR;
                let mut cur_x: f32 = cx + (rad.cos() * self.radius as f64) as f32 * GLOW_FACTOR;
                let mut cur_y: f32 = cy + (rad.sin() * self.radius as f64) as f32 * GLOW_FACTOR;
 
                if prev_x == 0.0 {
                    prev_radius = cur_radius;
                    prev_x = cur_x;
                    prev_y = cur_y;
                } else {
                    cur_radius = prev_radius * GLOW_FACTOR;
                    cur_x = prev_x * GLOW_FACTOR;
                    cur_y = prev_y * GLOW_FACTOR;
                }
                let _ = canvas.draw_point(Point::new(cur_x as i32, cur_y as i32));
                coordenates.push((cur_x, cur_y));
            }
            self.fill_color(canvas, center, cur_radius, color); 
            // self.fill_color(canvas, coordenates); 
        }
    }

    // fn fill_color(
    //     &self,
    //     canvas: &mut Canvas<Window>,
    //     coordenates: Vec<(f32, f32)>,
    // ) {
    //     for coord in coordenates {
    //         let _ = canvas.draw_point(
    //             Point::new(coord.0 as i32, coord.1 as i32)
    //         );
    //     }
    // }

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
        &self, canvas: &mut Canvas<Window>, collision_coordenates: (f32, f32)
    ) {
        // canvas.set_draw_color(Color::RGB(52, 204, 235));
        canvas.set_draw_color(Color::RGB(255, 0, 127));

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
        //     Color::RGB(255, 155, 127),
        // );

        for body_coordenate in Self::get_inner_coordenates(self.radius, self.position) {
            for light_coordenate in &light_coordenates {
                if body_coordenate.0 <= light_coordenate.0 
                && body_coordenate.1 >= light_coordenate.1 - radius && body_coordenate.1 <= light_coordenate.1 + radius
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


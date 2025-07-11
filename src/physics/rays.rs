use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::Body;

pub struct Ray {
    pub points: Vec<(f32, f32)>,
}

impl Ray {
    pub fn new(position: &(f32, f32)) -> Ray {
        let points: Vec<(f32, f32)> = Self::get_points(&position);

        Ray {
            points: points,
        }
    }

    fn get_points(position: &(f32, f32)) -> Vec<(f32, f32)> {
        let mut points: Vec<(f32, f32)> = vec![];

        let (cx, cy) = *position;
        
        let end_circle: u16 = 360;
        // Defines how much rays we will have
        let step: u16 = 2; // TODO: Usar 5 para mais de um impacto de raio na ball.

        // TODO: Fazer disso um % do tamanho do radius do light source.
        let mut radius: f32 = 20.0;
        let step_radius: f32 = 9.0;
        // TODO: Fazer disto uma equacao que represente ate onde a luz
        // e' propagada dada a forca do brilho da light source.
        let final_radius: f32 = 300.0; 

        while radius < final_radius {
            let mut degree: u16 = 0;

            while degree < end_circle {
                let rad = (degree as f64).to_radians();
                let x = cx + (rad.cos() * radius as f64) as f32;
                let y = cy + (rad.sin() * radius as f64) as f32;

                points.push((x, y));

                degree += step;
            }
            radius += step_radius;
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

    pub fn handle_collisions(&mut self, body: &Body) -> Vec<(f32, f32)> {
        let mut coordenates: Vec<(f32, f32)> = vec![];
        let mut new_points: Vec<(f32, f32)> = self.points.clone();

        for body_coordenate in &body.coordenates {
            for light_coordenate in self.points.clone() {
                if body_coordenate.0 <= light_coordenate.0 
                && body_coordenate.1 >= light_coordenate.1
                && body_coordenate.1 <= light_coordenate.1
                {
                    coordenates.push(*body_coordenate);
                    new_points.retain(|&x| x != *body_coordenate);
                }
            }
        }
        self.points = new_points;
        coordenates
    }

    // pub fn handle_collisions(&self, bodies: Vec<&Body>) -> (f32, f32) {
    //     let mut coordenates: (f32, f32) = (0.0, 0.0);
    //     let mut stop: bool = false;

    //     for body in bodies {
    //         for body_coordenate in &body.coordenates {
    //             for light_coordenate in self.points.clone() {
    //                 if body_coordenate.0 <= light_coordenate.0 
    //                 && body_coordenate.1 >= light_coordenate.1
    //                 && body_coordenate.1 <= light_coordenate.1
    //                 {
    //                     coordenates = *body_coordenate;
    //                     stop = true;
    //                     break
    //                 }
    //             }
    //             if stop == true { break }
    //         }
    //         // TODO: Quando lidando com mais bodies, preciso tirar esse break
    //         // e tambem fazer a funcao retornar coordenadas para cada corpo
    //         if stop == true { break }
    //     }
    //     coordenates
    // }
}


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
        let step: u16 = 1; // Defines how much rays we will have

        // TODO: Fazer disso um % do tamanho do radius do light source.
        let mut radius: f32 = 20.0;

        let step_radius: f32 = 9.0;
        
        // TODO: Fazer disto uma equacao que represente ate onde a luz
        // e' propagada dada a forca do brilho da light source.
        // let final_radius: f32 = 285.0; 
        let final_radius: f32 = 295.0; 

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
        // let mut new_points: Vec<(f32, f32)> = self.points.clone();

        for light_coordenate in &self.points {
            for body_coordenate in &body.circunference {
                if light_coordenate.0 > body_coordenate.0
                && light_coordenate.1 > body_coordenate.1
                {
                    coordenates.push(*body_coordenate);
                    // new_points.retain(|&x| x != *light_coordenate);
                }
            }
        }
        // self.points = new_points;
        coordenates
    }

    // fn get_limits(&self, coordenates: Vec<(f32, f32)>) {
    //     let mut upper_limit: (f32, f32) = (0.0, 0.0); 
    //     let mut lower_limit: (f32, f32) = (0.0, 0.0); 
    //     let mut right_limit: (f32, f32) = (0.0, 0.0); 
    //     let mut left_limit: (f32, f32) = (0.0, 0.0); 

    //     let mut sample: Vec<(f32, f32)> = coordenates.clone();
    //     sample.sort();

    //     if let Some(max_upper_y) = sample.iter().max_by_key(|t| t.1) {
    //         upper_limit = max_upper_y;
    //     }
    //     

    //    // Pegar o maior valor de y e colocar no upper, menor e colocar no lower
    //    // Fazer o mesmo com x pra esquerda e pra direita
    //    // retornar esses limites como retorno da funcao
    //    // Criar uma outra funcao que verifica se as coordenadas da luz/corpo
    //    // estao dentro dos limites e, se sim, usar na funcao acima
    //    // para tirar e colocar nos vetores.
    // }
}


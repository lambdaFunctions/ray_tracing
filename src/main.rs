use std::time::Duration;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

mod view;
use view::background;
mod physics;
use physics::bodies::Body;
use physics::rays::Ray;

static SCREEN_WIDTH: u32 = 1000;
static SCREEN_HEIGHT: u32 = 800;

fn main() {
    let sdl_context: sdl2::Sdl = sdl2::init().unwrap();

    let video_subsystem = sdl_context.video();
    let window = video_subsystem
        .expect("REASON")
        .window("Ray Tracing", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas: sdl2::render::Canvas<sdl2::video::Window> =
        window.into_canvas().build().unwrap();

    let background_view: background::Background = background::Background {
        screen_area: Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT),
        clear_color: Color::RGB(0, 0, 0),
    };

    let mut running: bool = true;
    let mut event_queue: sdl2::EventPump = sdl_context.event_pump().unwrap();

    let ball: Body = Body::new(Color::RGB(52, 177, 235), (500.0, 400.0), 30.0, None);

    let light_source: Body = Body::new(
        Color::RGB(255, 255, 255),
        (200.0, 400.0),
        15.0,
        Some(true),
    );

    let ray: Ray = Ray::new(&light_source.position, 0.0);

    while running {
        background_view.render(&mut canvas);

        for event in event_queue.poll_iter() {
            match event {
                // Event::MouseMotion { x, y, .. } => {
                //     let _mouse_position: (i32, i32) = (x, y);
                // },
                Event::Quit { .. } => {
                    running = false;
                }
                _ => {}
            }
        }

        ball.render(&mut canvas);
        light_source.render(&mut canvas);
        ray.render(&mut canvas);

        let collision_coordenates: (f32, f32) = ray.handle_collisions(vec![&ball]);
        // ball.handle_ray_collision(&ray);

        ball.draw_lighted_pixels(&mut canvas, collision_coordenates);

        canvas.present();

        ::std::thread::sleep(Duration::from_millis(5));
    }
}

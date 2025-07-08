use std::{time::Duration};

use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;

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
    let window = video_subsystem.expect("REASON").window(
        "Ray Tracing", SCREEN_WIDTH, SCREEN_HEIGHT
    )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas: sdl2::render::Canvas<sdl2::video::Window> = window.into_canvas()
        .build()
        .unwrap();

    let background_view: background::Background = background::Background {
        screen_area: Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT),
        clear_color: Color::RGB(0, 0, 0),
    };

    let mut running: bool = true;
    let mut event_queue: sdl2::EventPump = sdl_context.event_pump().unwrap();    

    let ball: Body = Body::new(
        Color::RGB(0, 255, 255),
        (500.0, 400.0),
        30.0,
        None,
    );

    let light_source: Body = Body::new(
        Color::RGB(255, 255, 255),
        (200.0, 400.0),
        15.0,
        Some(true),
    );

    let ray: Ray = Ray::new(
        Color::RGB(255, 255, 255), &light_source.position, 0.0 
    );

    while running {
        background_view.render(&mut canvas);

        for event in event_queue.poll_iter() {
            match event {
                Event::MouseMotion { x, y, .. } => {
                    let _mouse_position: (i32, i32) = (x, y);
                },
                Event::Quit {..} => {
                    running = false;
                },
                _ => {}
            }
        }

        ball.render(&mut canvas);
        light_source.render(&mut canvas);
        ray.render(&mut canvas);

        // light_source.change_position(mouse_position.0 as f32, mouse_position.0 as f32);

        canvas.present();

        ::std::thread::sleep(Duration::from_millis(5));
   }
}


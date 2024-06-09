extern crate sdl2;
extern crate serde;
extern crate serde_json;

mod drawing;
mod physics;
mod game;
mod graph;
mod camera;
mod grid;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::{MouseButton, MouseWheelDirection};
use std::time::Duration;
use game::Game;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("2D Game Engine", 1920, 1200)
        .position_centered()
        .fullscreen_desktop()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Initialize game
    let mut game = Game::new("nodes.json", 1920.0, 1200.0);

    'running: loop {
        let mouse_state = event_pump.mouse_state();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                Event::MouseButtonDown { x, y, mouse_btn, .. } => {
                    game.handle_mouse_button_down(x, y, mouse_btn);
                }
                Event::MouseButtonUp { mouse_btn, .. } => {
                    game.handle_mouse_button_up(mouse_btn);
                }
                Event::MouseMotion { x, y, .. } => {
                    game.handle_mouse_motion(x, y);
                }
                Event::MouseWheel { x, y, .. } => {
                    game.handle_mouse_wheel(mouse_state.x(), mouse_state.y(), y);
                },
                _ => {}
            }
        }

        game.update();

        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();

        drawing::draw_circles(&mut canvas, &game.circles, &game.camera);
        drawing::draw_edges(&mut canvas, &game.graph.edges, &game.circles, &game.camera);

        canvas.present();
        std::thread::sleep(Duration::from_millis(16));
    }
}

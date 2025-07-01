use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod vehicles;
use vehicles::*;

use road_intersection::*;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("road_intersection", 900, 700)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::BLACK);
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut all_vehicles = Vehicles::new();
    // let mut last_spawn_time = Instant::now();
    

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown { keycode: Some(key), .. } => {
                        match key {
                            Keycode::Up => {
                                all_vehicles.add_vehicle(Direction::North);
                            }
                            Keycode::Down => {
                                all_vehicles.add_vehicle(Direction::South);
                            }
                            Keycode::Left => {
                                all_vehicles.add_vehicle(Direction::East);
                            }
                            Keycode::Right => {
                                all_vehicles.add_vehicle(Direction::West);
                            }
                            Keycode::R => {
                                all_vehicles.add_random_vehicle();
                            }
                            _ => {}
                    }
                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        drwa_roads(&mut canvas);
        all_vehicles.draw_vehicles(&mut canvas);
        // draw_vehicles(&all_vehicles, &mut canvas);


        // canvas.set_draw_color(Color::RED);
        // canvas.fill_rect(Rect::new(50, 50, 50, 50)).unwrap();

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

    }
}

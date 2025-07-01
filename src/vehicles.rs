use sdl2::{render::Canvas, video::Window};

use sdl2::rect::Rect;
use sdl2::pixels::Color;
use rand::Rng;

#[derive(Debug)]
pub struct Vehicles {
    pub vehicles: Vec<Vehicle>
}

#[derive(Debug)]
pub struct Vehicle {
    // pub color: Color,
    // pub rect: Rect,
    pub x: i32,
    pub y: i32,
    pub route: Route,
    pub direction: Direction,
}

// pub struct Vehicle {
//     pub color: Color,
//     pub rect: Rect,
//     pub direction: Direction,
//     pub route: Route,
//     pub velocity: f32,
//     pub x: f32,
//     pub y: f32,
//     pub in_intersection: bool,
//     pub completed_turn: bool,
//  }

#[derive(Debug, PartialEq)]
pub enum Route {
Left,
Right,
Straight,
}

#[derive(Debug, PartialEq)]
pub enum Direction {
North,
South,
East,
West,
}

impl Vehicles {
    pub fn new() -> Self {
        Vehicles {
            vehicles: Vec::new(),
        }
    }

    pub fn add_vehicle(&mut self, direction: Direction) {
        let route = random_route();
        if direction == Direction::South {
            self.vehicles.push(Vehicle{
                route: route,
                direction: direction,
                x: 400,
                y: 0
            });
        } else if  direction == Direction::North {
            self.vehicles.push(Vehicle{
                route: route,
                direction: direction,
                x: 450,
                y: 650
            });
        } else if  direction == Direction::West {
            self.vehicles.push(Vehicle{
                route: route,
                direction: direction,
                x: 0,
                y: 350
            });
        } else if  direction == Direction::East {
            self.vehicles.push(Vehicle{
                route: route,
                direction: direction,
                x: 850,
                y: 300
            });
        }
    }

    pub fn draw_vehicles(&mut self, canvas: &mut Canvas<Window>) {
        if self.vehicles.len() == 0 { return }

        for vehicle in &mut self.vehicles {

        if vehicle.route == Route::Left {
            canvas.set_draw_color(Color::YELLOW);
        } else if vehicle.route == Route::Right {
            canvas.set_draw_color(Color::MAGENTA);
        } else {
            canvas.set_draw_color(Color::BLUE);
        }

        canvas.fill_rect(Rect::new(vehicle.x, vehicle.y,50, 50)).unwrap();

        if vehicle.direction == Direction::North {
            vehicle.y -= 2;
        } else if vehicle.direction == Direction::South {
            vehicle.y += 2;
        } else if vehicle.direction == Direction::East {
            vehicle.x -= 2;
        } else if vehicle.direction == Direction::West {
            vehicle.x += 2;
        }
        }

    }

    pub fn add_random_vehicle(&mut self) {

    }
}

fn random_route() -> Route {
    let rng = rand::thread_rng().gen_range(0..3);
    if rng == 0 {
        return Route::Left;
    } else if rng == 1 {
        return Route::Right;
    }
    Route::Straight
}
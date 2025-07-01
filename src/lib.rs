use sdl2::{render::Canvas, video::Window};
use sdl2::pixels::Color;
use sdl2::rect::Point;

pub fn drwa_roads(canvas: &mut Canvas<Window>) {
        let (width, height) = canvas.output_size().unwrap();
        let height=  height as i32;
        let width=  width as i32;

        let center_x = width / 2;
        let center_y = height / 2;

        canvas.set_draw_color(Color::WHITE);

        canvas.draw_line(Point::new(center_x, 0), Point::new(center_x, height)).unwrap();
        canvas.draw_line(Point::new(center_x + 50, 0), Point::new(center_x + 50, height)).unwrap();
        canvas.draw_line(Point::new(center_x - 50, 0), Point::new(center_x - 50, height)).unwrap();

        canvas.draw_line(Point::new(0, center_y), Point::new(width, center_y)).unwrap();
        canvas.draw_line(Point::new(0, center_y + 50), Point::new(width, center_y + 50)).unwrap();
        canvas.draw_line(Point::new(0, center_y - 50), Point::new(width, center_y - 50)).unwrap();

}
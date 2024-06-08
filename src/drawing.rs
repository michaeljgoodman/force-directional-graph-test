use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Point;

use crate::graph::Circle;
use crate::camera::Camera;

pub fn draw_circles(canvas: &mut Canvas<Window>, circles: &[Circle], camera: &Camera) {
    canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
    for circle in circles {
        let (screen_x, screen_y) = camera.world_to_screen(circle.x, circle.y);
        let scaled_radius = (circle.radius * camera.scale) as i32;
        let mut x = scaled_radius - 1;
        let mut y = 0;
        let mut dx = 1;
        let mut dy = 1;
        let mut err = dx - (scaled_radius * 2);

        while x >= y {
            canvas.draw_point(Point::new(screen_x as i32 + x, screen_y as i32 + y)).unwrap();
            canvas.draw_point(Point::new(screen_x as i32 + y, screen_y as i32 + x)).unwrap();
            canvas.draw_point(Point::new(screen_x as i32 - y, screen_y as i32 + x)).unwrap();
            canvas.draw_point(Point::new(screen_x as i32 - x, screen_y as i32 + y)).unwrap();
            canvas.draw_point(Point::new(screen_x as i32 - x, screen_y as i32 - y)).unwrap();
            canvas.draw_point(Point::new(screen_x as i32 - y, screen_y as i32 - x)).unwrap();
            canvas.draw_point(Point::new(screen_x as i32 + y, screen_y as i32 - x)).unwrap();
            canvas.draw_point(Point::new(screen_x as i32 + x, screen_y as i32 - y)).unwrap();

            if err <= 0 {
                y += 1;
                err += dy;
                dy += 2;
            }
            if err > 0 {
                x -= 1;
                dx += 2;
                err += dx - (scaled_radius * 2);
            }
        }
    }
}

pub fn draw_edges(canvas: &mut Canvas<Window>, edges: &[crate::graph::Edge], circles: &[Circle], camera: &Camera) {
    canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));
    for edge in edges {
        let source_circle = circles.iter().find(|c| c.id == edge.source).unwrap();
        let target_circle = circles.iter().find(|c| c.id == edge.target).unwrap();
        let (source_screen_x, source_screen_y) = camera.world_to_screen(source_circle.x, source_circle.y);
        let (target_screen_x, target_screen_y) = camera.world_to_screen(target_circle.x, target_circle.y);
        canvas.draw_line(
            Point::new(source_screen_x as i32, source_screen_y as i32),
            Point::new(target_screen_x as i32, target_screen_y as i32)
        ).unwrap();
    }
}

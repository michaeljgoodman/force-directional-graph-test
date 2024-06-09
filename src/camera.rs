pub struct Camera {
    pub x: f64,
    pub y: f64,
    pub scale: f64,
    pub screen_width: f64,
    pub screen_height: f64,
}

impl Camera {
    pub fn new(screen_width: f64, screen_height: f64) -> Camera {
        Camera {
            x: 0.0,
            y: 0.0,
            scale: 1.0,
            screen_width,
            screen_height,
        }
    }

    pub fn world_to_screen(&self, world_x: f64, world_y: f64) -> (f64, f64) {
        (
            (world_x - self.x) * self.scale + self.screen_width / 2.0,
            (world_y - self.y) * self.scale + self.screen_height / 2.0,
        )
    }

    pub fn screen_to_world(&self, screen_x: f64, screen_y: f64) -> (f64, f64) {
        (
            (screen_x - self.screen_width / 2.0) / self.scale + self.x,
            (screen_y - self.screen_height / 2.0) / self.scale + self.y,
        )
    }

    pub fn pan(&mut self, dx: f64, dy: f64) {
        self.x -= dx / self.scale;
        self.y -= dy / self.scale;
    }

    pub fn zoom(&mut self, factor: f64, cursor_x: f64, cursor_y: f64) {
        // Calculate the world coordinates under the cursor before zoom
        let (world_x_before, world_y_before) = self.screen_to_world(cursor_x, cursor_y);

        // Apply the zoom scale factor
        self.scale *= factor;

        // Calculate the world coordinates under the cursor post zoom
        let (world_x_after, world_y_after) = self.screen_to_world(cursor_x, cursor_y);

        // Adjust the camera position so that the location under the cursor is consistent with the world coordinates you were pointing at
        self.x += world_x_before - world_x_after;
        self.y += world_y_before - world_y_after;
    }
}

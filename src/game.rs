use rand::Rng;
use sdl2::mouse::MouseButton;
use crate::graph::{Graph, Circle};
use crate::physics::{apply_gravity_and_repulsion, apply_spring_forces, apply_center_attraction};
use crate::camera::Camera;
use crate::grid::SpatialGrid;

pub struct Game {
    pub graph: Graph,
    pub circles: Vec<Circle>,
    pub camera: Camera,
    pub dragging_circle: Option<usize>,
    pub dragging_pan: bool,
    pub last_mouse_x: i32,
    pub last_mouse_y: i32,
    pub gravity_strength: f64,
    pub repulsion_strength: f64,
    pub spring_strength: f64,
    pub rest_length: f64,
    pub max_velocity: f64,
    pub attraction_strength: f64,
    pub grid: SpatialGrid,
    pub force_scale: f64,
}

impl Game {
    pub fn new(data_file: &str, screen_width: f64, screen_height: f64) -> Game {
        let data = std::fs::read_to_string(data_file).expect("Unable to read file");
        let graph: Graph = serde_json::from_str(&data).expect("JSON was not well-formatted");

        let mut rng = rand::thread_rng();
        let center_x = screen_width / 2.0;
        let center_y = screen_height / 2.0;

        let circles: Vec<Circle> = graph.nodes.iter().map(|node| {
            let x = center_x + rng.gen_range(-100.0..100.0);
            let y = center_y + rng.gen_range(-100.0..100.0);
            Circle::new(node.id, x, y, node.radius as f64, 0.0, 0.0) // Initialize with zero velocity
        }).collect();

        Game {
            graph,
            circles,
            camera: Camera::new(screen_width, screen_height),
            dragging_circle: None,
            dragging_pan: false,
            last_mouse_x: 0,
            last_mouse_y: 0,
            gravity_strength: 0.0001,
            repulsion_strength: 1000.0,
            spring_strength: 0.01,
            rest_length: 100.0,
            max_velocity: 5.0,
            attraction_strength: 0.001,
            grid: SpatialGrid::new(100.0), // Initialize grid with cell size
            force_scale: 0.01, // Start with a low force scale
        }
    }

    pub fn handle_mouse_button_down(&mut self, x: i32, y: i32, mouse_btn: MouseButton) {
        if mouse_btn == MouseButton::Left {
            self.last_mouse_x = x;
            self.last_mouse_y = y;
            let (world_x, world_y) = self.camera.screen_to_world(x as f64, y as f64);
            let mut found_circle = false;
            for circle in &mut self.circles {
                if circle.contains_point(world_x, world_y) {
                    circle.dragging = true;
                    self.dragging_circle = Some(circle.id);
                    found_circle = true;
                    break;
                }
            }
            if !found_circle {
                self.dragging_pan = true;
            }
        }
    }

    pub fn handle_mouse_button_up(&mut self, mouse_btn: MouseButton) {
        if mouse_btn == MouseButton::Left {
            if let Some(id) = self.dragging_circle {
                for circle in &mut self.circles {
                    if circle.id == id {
                        circle.dragging = false;
                        self.dragging_circle = None;
                        break;
                    }
                }
            }
            self.dragging_pan = false;
        }
    }

    pub fn handle_mouse_motion(&mut self, x: i32, y: i32) {
        if let Some(id) = self.dragging_circle {
            for circle in &mut self.circles {
                if circle.id == id {
                    let (world_x, world_y) = self.camera.screen_to_world(x as f64, y as f64);
                    circle.x = world_x;
                    circle.y = world_y;
                    break;
                }
            }
        } else if self.dragging_pan {
            let dx = x - self.last_mouse_x;
            let dy = y - self.last_mouse_y;
            self.camera.pan(dx as f64, dy as f64);
            self.last_mouse_x = x;
            self.last_mouse_y = y;
        }
    }

    pub fn handle_mouse_wheel(&mut self, mouse_x: i32, mouse_y: i32, y: i32) {
        let zoom_factor = if y > 0 { 1.1 } else { 0.9 };
        self.camera.zoom(zoom_factor, mouse_x as f64, mouse_y as f64);
    }

    pub fn update(&mut self) {
        // Clear and populate the spatial grid
        self.grid.clear();
        for (i, circle) in self.circles.iter().enumerate() {
            self.grid.insert(i, circle.x, circle.y);
        }

        // Apply physics using the grid
        apply_gravity_and_repulsion(&mut self.circles, self.gravity_strength * self.force_scale, self.repulsion_strength * self.force_scale, &self.grid);
        apply_spring_forces(&mut self.circles, &self.graph.edges, self.spring_strength * self.force_scale, self.rest_length);

        let center_x = 0.0;
        let center_y = 0.0;
        apply_center_attraction(&mut self.circles, center_x, center_y, self.attraction_strength * self.force_scale);

        for circle in &mut self.circles {
            circle.update(self.max_velocity);
        }

        // Gradually increase the force scale
        if self.force_scale < 1.0 {
            self.force_scale += 0.01;
        }
    }
}

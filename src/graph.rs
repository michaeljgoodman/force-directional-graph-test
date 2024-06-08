use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Node {
    pub id: usize,
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub vx: f64,
    pub vy: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Edge {
    pub source: usize,
    pub target: usize,
}

#[derive(Serialize, Deserialize)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

#[derive(Clone)]
pub struct Circle {
    pub id: usize,
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub vx: f64,
    pub vy: f64,
    pub dragging: bool,
}

impl Circle {
    pub fn new(id: usize, x: f64, y: f64, radius: f64, vx: f64, vy: f64) -> Circle {
        Circle { id, x, y, radius, vx, vy, dragging: false }
    }

    pub fn apply_force(&mut self, fx: f64, fy: f64) {
        self.vx += fx;
        self.vy += fy;
    }

    pub fn update(&mut self, max_velocity: f64) {
        if !self.dragging {
            self.x += self.vx;
            self.y += self.vy;

            // Apply damping
            self.vx *= 0.95;
            self.vy *= 0.95;

            // Cap the velocity
            let speed = (self.vx * self.vx + self.vy * self.vy).sqrt();
            if speed > max_velocity {
                self.vx = (self.vx / speed) * max_velocity;
                self.vy = (self.vy / speed) * max_velocity;
            }
        }
    }

    pub fn contains_point(&self, x: f64, y: f64) -> bool {
        let dx = self.x - x;
        let dy = self.y - y;
        dx * dx + dy * dy <= self.radius * self.radius
    }
}

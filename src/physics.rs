use crate::graph::{Circle, Edge};

pub fn apply_gravity_and_repulsion(circles: &mut [Circle], gravity_strength: f64, repulsion_strength: f64) {
    let circles_copy = circles.to_vec(); // Create a copy of the circles vector for safe iteration

    for circle in circles {
        if !circle.dragging {
            // Apply gravity between nodes
            for other in &circles_copy {
                if circle.id != other.id {
                    let dx = other.x - circle.x;
                    let dy = other.y - circle.y;
                    let distance = (dx * dx + dy * dy).sqrt();
                    if distance > 0.0 {
                        let force = gravity_strength * circle.radius * other.radius / (distance * distance);
                        circle.apply_force(force * dx / distance, force * dy / distance);
                    }
                }
            }

            // Apply repulsion from other circles
            for other in &circles_copy {
                if circle.id != other.id {
                    let dx = circle.x - other.x;
                    let dy = circle.y - other.y;
                    let distance = (dx * dx + dy * dy).sqrt();
                    let min_distance = circle.radius + other.radius; // Minimum distance scaled by radius
                    if distance > 0.0 && distance < min_distance {
                        let force = repulsion_strength / (distance * distance);
                        circle.apply_force(force * dx / distance, force * dy / distance);
                    } else if distance > 0.0 {
                        let force = repulsion_strength / (distance * distance);
                        circle.apply_force(force * dx / distance, force * dy / distance);
                    }
                }
            }
        }
    }
}

pub fn apply_spring_forces(circles: &mut [Circle], edges: &[Edge], spring_strength: f64, rest_length: f64) {
    for edge in edges {
        let source_circle = circles.iter().find(|c| c.id == edge.source).unwrap();
        let target_circle = circles.iter().find(|c| c.id == edge.target).unwrap();
        let dx = target_circle.x - source_circle.x;
        let dy = target_circle.y - source_circle.y;
        let distance = (dx * dx + dy * dy).sqrt();
        let displacement = distance - rest_length;
        let force = spring_strength * displacement;
        let fx = force * dx / distance;
        let fy = force * dy / distance;
        if let Some(source_circle) = circles.iter_mut().find(|c| c.id == edge.source) {
            source_circle.apply_force(fx, fy);
        }
        if let Some(target_circle) = circles.iter_mut().find(|c| c.id == edge.target) {
            target_circle.apply_force(-fx, -fy);
        }
    }
}

pub fn apply_center_attraction(circles: &mut [Circle], center_x: f64, center_y: f64, attraction_strength: f64) {
    for circle in circles {
        if !circle.dragging {
            let dx = center_x - circle.x;
            let dy = center_y - circle.y;
            let distance = (dx * dx + dy * dy).sqrt();
            if distance > 0.0 {
                let force = attraction_strength * distance;
                circle.apply_force(force * dx / distance, force * dy / distance);
            }
        }
    }
}

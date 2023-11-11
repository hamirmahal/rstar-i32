use rstar::{PointDistance, RTreeObject, AABB};

struct Circle {
    origin: [f32; 2],
    radius: f32,
}

impl RTreeObject for Circle {
    type Envelope = AABB<[f32; 2]>;

    fn envelope(&self) -> Self::Envelope {
        let corner_1 = [self.origin[0] - self.radius, self.origin[1] - self.radius];
        let corner_2 = [self.origin[0] + self.radius, self.origin[1] + self.radius];
        AABB::from_corners(corner_1, corner_2)
    }
}

impl PointDistance for Circle {
    fn distance_2(&self, point: &[f32; 2]) -> f32 {
        let d_x = self.origin[0] - point[0];
        let d_y = self.origin[1] - point[1];
        let distance_to_origin = (d_x * d_x + d_y * d_y).sqrt();
        let distance_to_ring = distance_to_origin - self.radius;
        let distance_to_circle = f32::max(0.0, distance_to_ring);
        // We must return the squared distance!
        distance_to_circle * distance_to_circle
    }

    // This implementation is not required but more efficient since it
    // omits the calculation of a square root
    fn contains_point(&self, point: &[f32; 2]) -> bool {
        let d_x = self.origin[0] - point[0];
        let d_y = self.origin[1] - point[1];
        let distance_to_origin_2 = d_x * d_x + d_y * d_y;
        let radius_2 = self.radius * self.radius;
        distance_to_origin_2 <= radius_2
    }
}

fn main() {
    println!("Starting...");
    let circle = Circle {
        origin: [1.0, 0.0],
        radius: 1.0,
    };

    assert_eq!(circle.distance_2(&[-1.0, 0.0]), 1.0);
    assert_eq!(circle.distance_2(&[-2.0, 0.0]), 4.0);
    assert!(circle.contains_point(&[1.0, 0.0]));
    println!("Done");
}

use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub latitude: f64,
    pub longitude: f64,
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq)]
pub struct Region {
    pub boundaries: Vec<Point>, // List of points defining the polygon
}

impl Region {
    pub fn contains(&self, point: &Point) -> bool {
        let mut inside = false;
        let mut j = self.boundaries.len() - 1;
        for i in 0..self.boundaries.len() {
            let xi = self.boundaries[i].latitude;
            let yi = self.boundaries[i].longitude;
            let xj = self.boundaries[j].latitude;
            let yj = self.boundaries[j].longitude;

            let intersect = ((yi > point.longitude) != (yj > point.longitude))
                && (point.latitude < (xj - xi) * (point.longitude - yi) / (yj - yi) + xi);
            if intersect {
                inside = !inside;
            }
            j = i;
        }
        inside
    }
}
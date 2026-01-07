use crate::*;
use aea_macros::Generable;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq, Generable)]
pub struct Point {
    pub latitude: f64,
    pub longitude: f64,
}

impl Default for Point {
    fn default() -> Self {
        Self {
            latitude: 0.0,
            longitude: 0.0,
        }
    }
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq)]
pub struct RegionRole {
    pub members: HashSet<AccountId>,
    pub active_count: u64,
    pub boundaries: Vec<Point>, // List of points defining the polygon
    pub name: String,              // Region name (e.g., "San Francisco", "District 42")
    pub osm_place_id: Option<u64>, // OpenStreetMap place ID for updates
    pub osm_type: Option<String>,  // OSM type (e.g., "city", "district", "neighbourhood")
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq)]
pub struct RegionRoleInput {
    pub boundaries: Vec<Point>,
    pub name: String,
    pub osm_place_id: Option<u64>,
    pub osm_type: Option<String>,
}

impl RegionRole {
    pub fn from_input(input: RegionRoleInput) -> Self {
        Self {
            members: HashSet::new(),
            active_count: 0,
            boundaries: input.boundaries,
            name: input.name,
            osm_place_id: input.osm_place_id,
            osm_type: input.osm_type,
        }
    }

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

    pub fn join(&mut self, account_id: AccountId) {
        self.members.insert(account_id);
        self.active_count += 1;
    }

    pub fn leave(&mut self, account_id: AccountId) {
        self.members.remove(&account_id);
        self.active_count -= 1;
    }

    /// Convert from OpenStreetMap GeoJSON format
    /// GeoJSON coordinates are in [longitude, latitude] order
    pub fn from_osm_geojson(
        geojson_coordinates: Vec<Vec<f64>>,
        name: String,
        place_id: Option<u64>,
        osm_type: Option<String>,
    ) -> Result<Self, String> {
        if geojson_coordinates.is_empty() {
            return Err("No coordinates provided".to_string());
        }

        let boundaries: Vec<Point> = geojson_coordinates
            .iter()
            .map(|coord| {
                if coord.len() < 2 {
                    return Err("Invalid coordinate format".to_string());
                }
                Ok(Point {
                    longitude: coord[0], // GeoJSON is [lon, lat]
                    latitude: coord[1],
                })
            })
            .collect::<Result<Vec<_>, String>>()?;

        // Ensure polygon is closed (first point equals last point)
        let needs_closure = boundaries.len() > 2 
            && (boundaries.first() != boundaries.last());
        
        let final_boundaries = if needs_closure {
            let mut closed = boundaries;
            closed.push(closed[0].clone());
            closed
        } else {
            boundaries
        };

        Ok(Self {
            members: HashSet::new(),
            active_count: 0,
            boundaries: final_boundaries,
            name,
            osm_place_id: place_id,
            osm_type,
        })
    }
}
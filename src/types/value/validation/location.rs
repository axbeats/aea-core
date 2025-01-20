use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq)]
pub struct LocationValidation {
    pub allowed_regions: Option<HashSet<String>>,    // Regions that are allowed
    pub disallowed_regions: Option<HashSet<String>>, // Regions that are not allowed
    pub min_latitude: Option<f64>,
    pub max_latitude: Option<f64>,
    pub min_longitude: Option<f64>,
    pub max_longitude: Option<f64>,
    pub allowed_countries: Option<HashSet<String>>,
    pub disallowed_cities: Option<HashSet<String>>,
    pub allowed_time_zones: Option<HashSet<String>>,

}

impl LocationValidation {
    pub fn validate(
        &self,
        region: &str,
        latitude: f64,
        longitude: f64,
        country: &str,
        city: &str,
        time_zone: &str,
    ) -> Result<(), String> {
        // Check allowed regions
        if let Some(allowed_regions) = &self.allowed_regions {
            if !allowed_regions.contains(region) {
                return Err(format!("Region '{}' is not allowed.", region));
            }
        }

        // Check disallowed regions
        if let Some(disallowed_regions) = &self.disallowed_regions {
            if disallowed_regions.contains(region) {
                return Err(format!("Region '{}' is disallowed.", region));
            }
        }

        // Check latitude constraints
        if let Some(min_lat) = self.min_latitude {
            if latitude < min_lat {
                return Err(format!("Latitude {} is below the minimum allowed latitude of {}.", latitude, min_lat));
            }
        }

        if let Some(max_lat) = self.max_latitude {
            if latitude > max_lat {
                return Err(format!("Latitude {} exceeds the maximum allowed latitude of {}.", latitude, max_lat));
            }
        }

        // Check longitude constraints
        if let Some(min_lon) = self.min_longitude {
            if longitude < min_lon {
                return Err(format!("Longitude {} is below the minimum allowed longitude of {}.", longitude, min_lon));
            }
        }

        if let Some(max_lon) = self.max_longitude {
            if longitude > max_lon {
                return Err(format!("Longitude {} exceeds the maximum allowed longitude of {}.", longitude, max_lon));
            }
        }

        // Check allowed countries
        if let Some(allowed_countries) = &self.allowed_countries {
            if !allowed_countries.contains(country) {
                return Err(format!("Country '{}' is not allowed.", country));
            }
        }

        // Check disallowed cities
        if let Some(disallowed_cities) = &self.disallowed_cities {
            if disallowed_cities.contains(city) {
                return Err(format!("City '{}' is disallowed.", city));
            }
        }

        // Check allowed time zones
        if let Some(allowed_time_zones) = &self.allowed_time_zones {
            if !allowed_time_zones.contains(time_zone) {
                return Err(format!("Time zone '{}' is not allowed.", time_zone));
            }
        }

        // If all checks pass, return Ok
        Ok(())
    }
}

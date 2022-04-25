use std::f64::consts::PI;

use serde::Deserialize;
use serde_json;
use serde_json::Value;

#[derive(Deserialize, Debug)]
struct GeoPolygon {
    path: Vec<(f64, f64)>,
}

#[derive(Deserialize, Debug)]
struct GeoCircle {
    lat: f64,
    lng: f64,
    rad: f64,
}

#[derive(Deserialize, Debug)]
struct GeoPoint {
    lat: f64,
    lng: f64,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum GeoShape {
    Circle(GeoCircle),
    Polygon(GeoPolygon),
    Point(GeoPoint),
    Null,
}

impl GeoPolygon {
    fn contains(&self, point: &GeoPoint) -> bool {
        if self.path.len() == 0 {
            return false;
        }

        let mut crossings = 0;
        let path = self
            .path
            .iter()
            .map(|v| GeoPoint { lat: v.0, lng: v.1 })
            .collect::<Vec<_>>();
        // Iterate over path in pairs of two, pairing the last element with the first one.
        for (a, b) in path
            .iter()
            .zip(path.iter().cycle().skip(1))
            .collect::<Vec<_>>()
        {
            if ray_crosses_segment(&point, a, b) {
                crossings += 1;
            }
        }
        crossings % 2 == 1
    }
}

impl GeoCircle {
    fn contains(&self, point: &GeoPoint) -> bool {
        const R: f64 = 6371.0;
        let d_lat = (point.lat - self.lat) * (PI / 180.0);
        let d_lng = (point.lng - self.lng) * (PI / 180.0);
        let mut a = (d_lat / 2.0).sin().powf(2.0);
        a += (self.lat * (PI / 180.0)).cos() * (point.lat * (PI / 180.0)).cos();
        a *= (d_lng / 2.0).sin().powf(2.0);

        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
        let distance = R * c * 1000.0;
        distance <= self.rad
    }
}

impl GeoShape {
    fn contains(&self, shape: &GeoShape) -> bool {
        match (self, shape) {
            (GeoShape::Polygon(a), GeoShape::Point(b)) => a.contains(b),
            (GeoShape::Circle(a), GeoShape::Point(b)) => a.contains(b),
            _ => false,
        }
    }
}

pub fn is_within_region(a: Value, b: Value) -> bool {
    let shape_a: GeoShape = serde_json::from_value(a).unwrap();
    let shape_b: GeoShape = serde_json::from_value(b).unwrap_or(GeoShape::Null);
    shape_b.contains(&shape_a)
}

pub fn is_within_regions(a: Value, b: Value) -> bool {
    let shape_a: GeoShape = serde_json::from_value(a).unwrap_or(GeoShape::Null);
    match b {
        Value::Array(arr) => arr.into_iter().any(|v| {
            serde_json::from_value(v)
                .unwrap_or(GeoShape::Null)
                .contains(&shape_a)
        }),
        _ => false,
    }
}

fn ray_crosses_segment<'a>(point: &GeoPoint, mut a: &'a GeoPoint, mut b: &'a GeoPoint) -> bool {
    if a.lat > b.lat {
        (a, b) = (b, a)
    }
    let (mut px, mut py) = (point.lng, point.lat);
    let (mut ax, ay) = (a.lng, a.lat);
    let (mut bx, by) = (b.lng, b.lat);
    if px < 0.0 {
        px += 360.0
    }
    if ax < 0.0 {
        ax += 360.0;
    }
    if bx < 0.0 {
        bx += 360.0;
    }
    if py == ay || py == by {
        py += 0.00000001;
    }
    if py > by || py < ay || px >= ax.max(bx) {
        return false;
    }
    if px < ax.min(bx) {
        return true;
    }

    let red = if ax != bx {
        (by - ay) / (bx - ax)
    } else {
        f64::INFINITY
    };
    let blue = if ax != px {
        (py - ay) / (px - ax)
    } else {
        f64::INFINITY
    };
    blue >= red
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn within_rectangle() {
        assert_eq!(
            GeoPolygon {
                path: vec![
                    (-229.283638, 30.9477275),
                    (-219.5717239, 35.037446),
                    (-218.4291458, 41.053078),
                    (-230.5580521, 33.2554844),
                    (-229.283638, 30.947727)
                ]
            }
            .contains(&GeoPoint {
                lat: -222.4721146,
                lng: 35.8964206
            }),
            true
        );
        assert_eq!(
            GeoPolygon {
                path: vec![
                    (-229.283638, 30.9477275),
                    (-219.5717239, 35.037446),
                    (-218.4291458, 41.053078),
                    (-230.5580521, 33.2554844),
                    (-229.283638, 30.947727)
                ]
            }
            .contains(&GeoPoint {
                lat: -231.4721146,
                lng: 35.8964206
            }),
            false
        );
    }

    #[test]
    fn within_star() {
        assert_eq!(
            GeoPolygon {
                path: vec![
                    (10.0, 10.0),
                    (11.0, 12.0),
                    (12.0, 10.0),
                    (15.0, 10.0),
                    (13.0, 9.0),
                    (14.0, 7.0),
                    (11.0, 8.0),
                    (8.0, 7.0),
                    (9.0, 9.0),
                    (7.0, 10.0),
                ]
            }
            .contains(&GeoPoint { lat: 7.5, lng: 9.9 }),
            true
        );
        assert_eq!(
            GeoPolygon {
                path: vec![
                    (10.0, 10.0),
                    (11.0, 12.0),
                    (12.0, 10.0),
                    (15.0, 10.0),
                    (13.0, 9.0),
                    (14.0, 7.0),
                    (11.0, 8.0),
                    (8.0, 7.0),
                    (9.0, 9.0),
                    (7.0, 10.0),
                ]
            }
            .contains(&GeoPoint { lat: 7.0, lng: 9.9 }),
            false
        );
    }

    #[test]
    fn within_circle() {
        assert_eq!(
            GeoCircle {
                lat: -27.5813671,
                lng: -48.48469,
                rad: 300.0
            }
            .contains(&GeoPoint {
                lat: -27.50836,
                lng: -48.48099
            }),
            false
        );
    }
}

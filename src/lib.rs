/// The main crate for lodestone-point
///
/// ## Overview
/// 
/// Takes a position (latlng coordinate) and returns a new Point GeoJson feature.
/// Inspired by [turf-point](https://github.com/Turfjs/turf-point).

// Standard lib packages
use std::str::FromStr;

// Third party packages
extern crate geojson;
extern crate rustc_serialize;

use rustc_serialize::json::{self, ToJson};
use geojson::{Error, Feature, Geometry, Position, Value, FromObject};

pub struct FeaturePoint {
  feature: Feature
}

impl FeaturePoint {
  pub fn new(pos: Position) -> Self {
    assert_eq!(pos.len(), 2);

    let geometry = Geometry::new(Value::Point(pos));
    let properties = json::Object::new();

    FeaturePoint {
      feature: Feature {
        bbox: None,
        crs: None,
        geometry: geometry,
        id: None,
        properties: Some(properties),
      }
    }
  }
  
  pub fn coordinates(&self) -> &Vec<f64> {
    type Err = Error;
    
    match self.feature.geometry.value {
      Value::Point(ref coords) => coords,
      _ => unreachable!("Type other than Value::Point should not be possible"),
    }
  }
}

impl FromStr for FeaturePoint {
  type Err = Error;
  fn from_str(s: &str) -> Result<Self, Self::Err> {

    let decoded_json = match json::Json::from_str(s) {
      Ok(j) => j,
      Err(..) => return Err(Error::new("Encountered malformed JSON")),
    };
    
    let object = match decoded_json {
      json::Json::Object(object) => object,
      _ => return Err(Error::new("Attempted to create GeoJSON from JSON that is not an object")),
    };

    Self::from_object(&object)
  }
}

impl FromObject for FeaturePoint {
  fn from_object(object: &json::Object) -> Result<Self, Error> {
    let feature = Feature::from_object(object).unwrap();
    Ok(FeaturePoint {
      feature: feature
    })
  }
}

impl ToJson for FeaturePoint {
  fn to_json(&self) -> json::Json {
    self.feature.to_json()
  }
}

impl ToString for FeaturePoint {
  fn to_string(&self) -> String {
    self.to_json().to_string()
  }
}

impl PartialEq for FeaturePoint {
  fn eq(&self, other: &Self) -> bool {
    self.coordinates() == other.coordinates()
  }
}

#[cfg(test)]
mod tests {
  use rustc_serialize::json::{self, ToJson};
  use super::FeaturePoint;

  #[test]
  fn test_valid_coordinates() {
    
    let expected_json = "{\"geometry\":{\"coordinates\":[-122.4167,37.7833],\"type\":\"Point\"},\"properties\":{},\"type\":\"Feature\"}";

    let coords = vec![-122.4167, 37.7833];
    let point = FeaturePoint::new(coords);
    let point_str = json::encode(&point.to_json()).unwrap();

    assert_eq!(point_str, expected_json);
  }

  #[test]
  fn test_coordinates() {
    let expected = vec![-122.4167, 37.7833];
    let coords = vec![-122.4167, 37.7833];
    let point = FeaturePoint::new(coords);

    assert_eq!(&expected, point.coordinates());
  }

  #[test]
  #[should_panic]
  fn test_invalid_coordinates() {
    let coords = vec![1.0];
    FeaturePoint::new(coords);
  }

  #[test]
  fn test_eq() {
    let point1 = FeaturePoint::new(vec![1.0, 1.0]);
    let point2 = FeaturePoint::new(vec![1.0, 1.0]);

    assert_eq!(point1 == point2, true);
  }
}

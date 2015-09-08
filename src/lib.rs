/// The main crate for lodestone-point
///
/// ## Overview
/// 
/// Takes coordinates and properties (optional) and returns a new Point GeoJson feature.
/// Inspired by [turf-point](https://github.com/Turfjs/turf-point/blob/master/index.js).

// Third party packages
extern crate geojson;
extern crate rustc_serialize;

use rustc_serialize::json;
use geojson::{Feature, GeoJson, Geometry, Value};

pub extern fn point(
    coordinates: Vec<f64>) -> GeoJson {
    // properties: BTreeMap<String, json::Json>) -> GeoJson {

  assert_eq!(coordinates.len(), 2);

  let geometry = Geometry::new(Value::Point(coordinates));
  // let properties = BTreeMap::new();
  let properties = json::Object::new();

  GeoJson::Feature(Feature {
    bbox: None,
    crs: None,
    geometry: geometry,
    id: None,
    properties: Some(properties),
  })
}

#[cfg(test)]
mod test {
  use rustc_serialize::json::{self, ToJson};
  use super::point;

  #[test]
  fn test_valid_coordinates() {
    
    let expected_json = "{\"geometry\":{\"coordinates\":[-122.4167,37.7833],\"type\":\"Point\"},\"properties\":{},\"type\":\"Feature\"}";

    let coords = vec![-122.4167, 37.7833];
    let geojson = point(coords);
    let point_str = json::encode(&geojson.to_json()).unwrap();

    assert_eq!(point_str, expected_json);
  }

  #[test]
  #[should_panic]
  fn test_invalid_coordinates() {
    let coords = vec![1.0];
    point(coords);
  }
}

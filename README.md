# lodestone-point

lodestone point module

### Parameters

| parameter     | type              | description                                                    |
| ------------- | ----------------- | -------------------------------------------------------------- |
| `position` | Vec<f64\>         | longitude, latitude position (each in decimal degrees)         |


### Example

```rs
use lodestone::{FeaturePoint}

let point = FeaturePoint::new([1.0,1.0]);
//
```

**Returns** `FeaturePoint`, a Point GeoJson feature

## Installation

Requires [rust](https://www.rust-lang.org/).

```
[dependencies]
lodestone-point = "0.1.0"
```

## Tests

```sh
$ cargo test
```


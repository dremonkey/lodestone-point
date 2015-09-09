# lodestone-point

lodestone point module


### `lodestone.point(coordinates)`

Takes coordinates and returns a new Point feature.


### Parameters

| parameter     | type              | description                                                    |
| ------------- | ----------------- | -------------------------------------------------------------- |
| `coordinates` | Vec<f64\>         | longitude, latitude position (each in decimal degrees)         |


### Example

```rs
var pt1 = lodestone.point([-75.343, 39.984]);

//=pt1
```

**Returns** `Feature.<Point>`, a Point GeoJson feature

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


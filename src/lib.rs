//! # Web Mercator Tiles
//!
//! Rust crate to convert lon/lat coordinates to Web Mercator tiles.<br>
//! See https://wiki.openstreetmap.org/wiki/Slippy_map_tilenames for details.
//!
//! # Warning
//!
//! All the functions provided by this crate **do not** check the validity of the input.
//! However, since they are based on equations, they will still return an (invalid) result.

use std::f64::consts::PI;

/// Convert lon/lat coordinates to a Web Mercator tile at a given zoom level.
///
/// # Arguments
///
/// * `lon`  - longitude coordinate (W-E), in degrees
/// * `lat`  - latitude  coordinate (N-S), in degrees
/// * `zoom` - zoom level
pub fn lonlat2tile(lon: f64, lat: f64, zoom: u8) -> (u32, u32) {
	let lat_rad = lat.to_radians();
	let z = 2f64.powf(zoom as f64);
	let x = ((lon + 180f64) / 360f64 * z) as u32;
	let y = ((1f64 - (lat_rad.tan() + (1f64 / lat_rad.cos())).ln() / PI) / 2f64 * z) as u32;
	(x, y)
}

/// Convert a Web Mercator tile to lon/lat coordinates at a given zoom level.
///
/// # Arguments
///
/// * `x`    - X tile coordinate
/// * `y`    - Y tile coordinate
/// * `zoom` - zoom level
pub fn tile2lonlat(x: u32, y: u32, zoom: u8) -> (f64, f64) {
	let z = 2f64.powf(zoom as f64);
	let lon = x as f64 / z * 360f64 - 180f64;
	let lat = (PI * (1f64 - 2f64 * y as f64 / z)).sinh().atan().to_degrees();
	(lon, lat)
}

/// Zoom in from the given tile.
///
/// The `zoom in` function returns the 4 tiles onto which the given tile is split out
/// when zooming to the next zoom level.
///
/// ```text
/// +--------+--------+
/// | x1, y1 | x2, y1 |
/// +--------+--------+
/// | x1, y2 | x2, y2 |
/// +--------+--------+
/// ```
///
/// # Arguments
///
/// * `x` - X tile coordinate
/// * `y` - Y tile coordinate
pub fn zoom_in(x: u32, y: u32) -> ((u32, u32), (u32, u32), (u32, u32), (u32, u32)) {
	let x2 = 2 * x;
	let y2 = 2 * y;
	((x2, y2), (x2 + 1, y2), (x2, y2 + 1), (x2 + 1, y2 + 1))
}

/// Zoom out from the given tile.
///
/// The `zoom out` function returns the tile onto which the given tile is merged
/// when zooming to the previous zoomlevel.
///
/// # Arguments
///
/// * `x`  - X current tile coordinate
/// * `y`  - Y current tile coordinate
pub fn zoom_out(x: u32, y: u32) -> (u32, u32) {
	(x / 2, y / 2)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_t2l() {
		assert_eq!(tile2lonlat(4376, 2932, 13), (12.3046875, 45.460130637921));
		assert_eq!(tile2lonlat(0, 0, 0), (-180.0, 85.0511287798066));
		assert_eq!(tile2lonlat(4376, 2932, 0), (1575180.0, -90.0));
	}

	#[test]
	fn test_l2t() {
		assert_eq!(lonlat2tile(12.3046875, 45.460130637921, 13), (4376, 2932));
		assert_eq!(lonlat2tile(0.0, 0.0, 0), (0, 0));
		assert_eq!(lonlat2tile(123456.789, 123456.789, 0), (343, 0));
	}

	#[test]
	fn test_zoom() {
		assert_eq!(zoom_in(1, 1), ((2, 2), (3, 2), (2, 3), (3, 3)));
		assert_eq!(zoom_out(5, 7), (2, 3));
		assert_eq!(zoom_out(0, 0), (0, 0));
	}
}

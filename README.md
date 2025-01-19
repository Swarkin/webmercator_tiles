# webmercator_tiles

Rust crate to convert lon/lat coordinates to Web Mercator tiles.<br>
See https://wiki.openstreetmap.org/wiki/Slippy_map_tilenames for details.

## Example

```rust
use webmercator_tiles;

fn main() {
	let t2l = webmercator_tiles::tile2lonlat(4376, 2932, 13); // (12.3046875, 45.460130637921)
	let l2t = webmercator_tiles::lonlat2tile(14.016667, 42.683333, 13); // (4414, 3019)
	println!("Tile (4376, 2932) at zoom 13: {t2l:?}");
	println!("lon 14.016667 lat 42.683333 at zoom 13: {l2t:?}");
}
```

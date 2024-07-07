use super::*;
use coverage_helper::test;

const HAMBURG: (f64, f64) = (53.54383973905111, 9.989119819258486);
const HAMBURG_HEIGHT: f32 = 3.4745164;

const ZUGSPITZE: (f64, f64) = (47.41333947600811, 10.979515398123247);
const ZUGSPITZE_HEIGHT: f32 = 818.86053; // real: 2962.0

const NEBELHORN: (f64, f64) = (47.421890712320305, 10.342221801784103);
const NEBELHORN_HEIGHT: f32 = 713.07715; // real: 2224.0

const BENSHEIM_STATION: (f64, f64) = (49.68134809269307, 8.61687829630227);
const BENSHEIM_STATION_HEIGHT: f32 = 100.41612;

const BENSHEIM_KIRCHBERG: (f64, f64) = (49.687527838796775, 8.626494754374288);
const BENSHEIM_KIRCHBERG_HEIGHT: f32 = 190.58434;

const BAYERN_PUNKT: (f64, f64) = (47.599599438895076, 10.980855540031962);
const BAYERN_PUNKT_HEIGHT: f32 = 2715.91;

#[test]
fn height_extraction() {
    // WGS84 to Mercator
    let converter = Proj::new_known_crs("EPSG:4326", "ESRI:53004", None).unwrap();
    let height_map = HeightMap::load_from_file("assets/dgm200_utm32s.tif");

    macro_rules! test_height {
        ($coords:expr, $height:expr) => {{
            let (lat, lng) = $coords;
            let result = converter.convert((lng, lat));
            let (x, y) = result.unwrap();

            assert_eq!(height_map.height_at_position(x, y), $height);
        }};
    }

    test_height!(BAYERN_PUNKT, BAYERN_PUNKT_HEIGHT);
    test_height!(HAMBURG, HAMBURG_HEIGHT);
    test_height!(BENSHEIM_STATION, BENSHEIM_STATION_HEIGHT);
    test_height!(BENSHEIM_KIRCHBERG, BENSHEIM_KIRCHBERG_HEIGHT);
    test_height!(NEBELHORN, NEBELHORN_HEIGHT);
    test_height!(ZUGSPITZE, ZUGSPITZE_HEIGHT);
}

#[test]
fn test_dummy() {
    let height_map = HeightMap::test_dummy();

    assert_eq!(height_map.height_at_position(0.0, 0.0), 0.0);
    assert_eq!(height_map.height_at_position(-1000.0, -1000.0), 0.0);
    assert_eq!(height_map.height_at_position(10.0, 42.5523), 0.0);
    assert_eq!(height_map.height_at_position(17.22, -412412.0), 0.0);
}

use core::f64;

#[cfg(test)]
use super::*;

/////
/// testing to_screen_coordinates()
/////
#[test]
fn origin_test() {
    let zero_point = Point3D (0.0, 0.0, 0.0);

    let result = zero_point.to_screen_coordinates();

    assert!(result.0 <= SCREEN_WIDTH as u32);
    assert!(result.1 <= SCREEN_HEIGHT as u32);
}

#[test]
fn endpoint_test() {
    let end_point = Point3D (f64::MAX, f64::MAX, f64::MAX);

    let result = end_point.to_screen_coordinates();

    assert!(result.0 < SCREEN_WIDTH as u32);
    assert!(result.1 < SCREEN_HEIGHT as u32);
}

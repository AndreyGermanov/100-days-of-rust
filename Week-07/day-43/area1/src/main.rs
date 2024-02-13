fn tri_area(b: f32, h:f32) -> f32 { 0.5 * b * h }

#[test]
fn test_tri_area() {
    assert_eq!(tri_area(3.0, 2.0), 3.0);
    assert_eq!(tri_area(5.0, 4.0), 10.0);
    assert_eq!(tri_area(7.0, 4.0), 14.0);
    assert_eq!(tri_area(10.0, 10.0), 50.0);
    assert_eq!(tri_area(12.0, 11.0), 66.0);
    assert_eq!(tri_area(0.0, 60.0), 0.0);
}
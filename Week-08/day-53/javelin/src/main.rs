
fn javelin_throw(speed: f32, angle:f32) -> String {
    let angle = angle * (std::f32::consts::PI / 180.0);
    format!("Ymax={}m, Xmax={}m",
        f32::round(f32::powi(speed * f32::sin(angle), 2) / (2.0 * 9.81)),
        f32::round(speed * f32::cos(angle) * 2.0 * speed * f32::sin(angle) / 9.81)
    )
}

#[test]
fn test_javelin_throw() {
    assert_eq!(javelin_throw(36.7, 45.0), "Ymax=34m, Xmax=137m");
    assert_eq!(javelin_throw(51.3, 20.0), "Ymax=16m, Xmax=172m");
    assert_eq!(javelin_throw(100.1, 89.0), "Ymax=511m, Xmax=36m");
}


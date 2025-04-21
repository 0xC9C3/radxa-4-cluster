// Temperature / Fan Speed mapping to define a default fan curve.
pub fn get_default_curve() -> Vec<(f32, f32)> {
    vec![
        (1f32, 10f32),
        (50f32, 20f32),
        (60f32, 50f32),
        (70f32, 80f32),
        (80f32, 100f32),
    ]
}

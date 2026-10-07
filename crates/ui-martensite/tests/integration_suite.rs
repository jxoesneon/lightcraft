//! End-to-end integration test for RAW development workflows.

use lightcraft_ui_martensite::LightcraftApp;

#[test]
fn test_raw_develop_workflow() {
    let mut app = LightcraftApp::new();

    // 1. Initial State
    assert_eq!(app.develop.exposure, 0.0);
    assert_eq!(app.develop.temperature, 5500);

    // 2. Adjust exposure, highlights, shadows
    app.develop.exposure = 0.75;
    app.develop.highlights = -40.0;
    app.develop.shadows = 35.0;
    app.develop.clamp_all();

    assert_eq!(app.develop.exposure, 0.75);
    assert_eq!(app.develop.highlights, -40.0);
    assert_eq!(app.develop.shadows, 35.0);

    // 3. Before/After inspection toggle
    assert!(app.toggle_before_after());
    assert!(app.before_after_split);
}

//! Comprehensive integration test suite for LightCraft's Martensite UI.
//!
//! Validates end-to-end integration across commands, menus, shortcuts,
//! develop sliders, range masks, album hierarchy and loupe coordinates.

use lightcraft_engine::{Engine, Tool};
use lightcraft_ui_martensite::{
    LightcraftApp,
    command_reg::{COMMAND_REGISTRY, find_command},
    menus::generate_main_menu,
    theme::CraftTheme,
    widgets::{
        AlbumItemDef, AlbumTreeWidget, DockPanelGroup, LoupeViewWidget, OptionsBarWidget, RangeMaskWidget, ScrubbyInputWidget, ToolStripWidget,
    },
};

#[test]
fn test_end_to_end_workspace_interaction() {
    let engine = Engine::new();
    let mut app = LightcraftApp::new(engine);

    // 1. Initial State Verification
    assert_eq!(app.active_tool, Tool::Loupe);
    assert_eq!(app.zoom_level, 1.0);
    assert!(app.rulers_visible);
    assert!(!app.before_after_split);

    // 2. Keystroke Workflow: switch to the masking brush, zoom in, hold Space to pan
    let new_tool = app.keyboard.on_key_down("k", app.active_tool);
    assert_eq!(new_tool, Some(Tool::MaskingBrush));
    app.set_tool(Tool::MaskingBrush);

    app.set_zoom(2.0);
    assert_eq!(app.zoom_level, 2.0);

    // Spring-loaded Zoom
    let zoom_tool = app.keyboard.on_key_down("Space", app.active_tool);
    assert_eq!(zoom_tool, Some(Tool::Zoom));
    app.set_tool(Tool::Zoom);

    app.pan_by(50.0, 100.0);
    assert_eq!(app.pan_offset, [50.0, 100.0]);

    // Release Space restores the masking brush
    let restored_tool = app.keyboard.on_key_up("Space");
    assert_eq!(restored_tool, Some(Tool::MaskingBrush));
    app.set_tool(Tool::MaskingBrush);

    // 3. Options Bar Interaction for the Active Masking Brush
    let mut options = OptionsBarWidget::new();
    options.set_tool(Tool::MaskingBrush);
    options.brush_size.on_pointer_down(0.0);
    options.brush_size.on_pointer_move(20.0, false, false);
    options.brush_size.on_pointer_up();
    assert_eq!(options.brush_size.value, 50.0); // 30 + 20

    // 4. Album Tree & Hierarchy Updates
    let mut albums = AlbumTreeWidget::new();
    albums.items.push(AlbumItemDef {
        id: 1,
        name: "Travel 2026".to_string(),
        count: 0,
        is_folder: true,
        expanded: false,
        children: vec![AlbumItemDef { id: 2, name: "Iceland".to_string(), count: 142, is_folder: false, expanded: false, children: vec![] }],
    });
    albums.select_album(2);
    assert_eq!(albums.selected_album_id, Some(2));
    albums.toggle_expanded(1);
    assert!(albums.items[0].expanded);

    // 5. Luminance Range Mask Split Feathering
    let mut mask = RangeMaskWidget::new();
    mask.luminance.set_min_split(10, 40);
    mask.luminance.set_max_split(210, 245);
    assert!(mask.luminance.is_split_min());
    assert!(mask.luminance.is_split_max());

    // 6. Docking System Validation
    let mut dock = DockPanelGroup::new(&["Light", "Color", "Masking"]);
    dock.select_tab(2);
    assert_eq!(dock.active_tab, 2);
    dock.toggle_collapsed();
    assert!(dock.collapsed_to_icons);

    // 7. Menu Generation Consistency
    let menus = generate_main_menu();
    assert!(!menus.is_empty());
    for menu in &menus {
        for item in &menu.items {
            if let Some(cmd_id) = item.command_id {
                assert!(find_command(cmd_id).is_some(), "Unknown command in menu: {}", cmd_id);
            }
        }
    }

    // 8. Theme Color Space Consistency
    let theme = CraftTheme::dark_neutral();
    let studio = CraftTheme::dark_studio();
    assert_ne!(theme.surface_app_bg, studio.surface_app_bg);

    // 9. Develop parameters clamp and reset
    app.develop.exposure = 0.75;
    app.develop.clamp_all();
    assert_eq!(app.develop.exposure, 0.75);
    assert!(app.toggle_before_after());
    app.reset_adjustments();
    assert_eq!(app.develop.exposure, 0.0);
    assert!(!COMMAND_REGISTRY.is_empty());

    let mut loupe = LoupeViewWidget::new(6000, 4000);
    loupe.zoom_at(2.0, [0.0, 0.0]);
    assert_eq!(loupe.screen_to_image([100.0, 100.0]), [50.0, 50.0]);

    let mut strip = ToolStripWidget::new();
    assert!(strip.toggle_mask_overlay());
    assert!(strip.mask_overlay);

    let mut slider = ScrubbyInputWidget::new("Clarity", 0.0, -100.0, 100.0, "%");
    slider.set_direct_value(140.0);
    assert_eq!(slider.value, 100.0);
}

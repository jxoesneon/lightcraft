//! Menu bar catalog.

pub struct MenuCategory {
    pub title: &'static str,
    pub commands: &'static [&'static str],
}

pub const MENUS: &[MenuCategory] = &[
    MenuCategory { title: "File", commands: &["file.import", "file.export"] },
    MenuCategory { title: "Develop", commands: &["photo.auto_tone", "photo.before_after"] },
    MenuCategory { title: "View", commands: &["photo.crop_rotate"] },
];

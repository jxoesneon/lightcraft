//! Decoupled command catalog and taxonomy for LightCraft.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CommandCategory {
    File,
    Edit,
    Library,
    Photo,
    View,
    Window,
    Help,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CommandSpec {
    pub id: &'static str,
    pub label: &'static str,
    pub category: CommandCategory,
    pub default_shortcut: Option<&'static str>,
    pub secondary_shortcut: Option<&'static str>,
}

pub const COMMAND_REGISTRY: &[CommandSpec] = &[
    // File
    CommandSpec {
        id: "file.import",
        label: "Import Photos and Video…",
        category: CommandCategory::File,
        default_shortcut: Some("Shift+Cmd+I"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "file.open_library",
        label: "Open Library…",
        category: CommandCategory::File,
        default_shortcut: Some("Shift+Cmd+O"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "app.export",
        label: "Export…",
        category: CommandCategory::File,
        default_shortcut: Some("Shift+Cmd+E"),
        secondary_shortcut: None,
    },
    // Edit
    CommandSpec { id: "edit.undo", label: "Undo", category: CommandCategory::Edit, default_shortcut: Some("Cmd+Z"), secondary_shortcut: None },
    CommandSpec { id: "edit.redo", label: "Redo", category: CommandCategory::Edit, default_shortcut: Some("Shift+Cmd+Z"), secondary_shortcut: None },
    CommandSpec {
        id: "photo.copy_settings",
        label: "Copy Settings…",
        category: CommandCategory::Edit,
        default_shortcut: Some("Shift+Cmd+C"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "photo.paste_settings",
        label: "Paste Settings",
        category: CommandCategory::Edit,
        default_shortcut: Some("Shift+Cmd+V"),
        secondary_shortcut: None,
    },
    // Library
    CommandSpec {
        id: "photo.flag",
        label: "Set Pick Flag",
        category: CommandCategory::Library,
        default_shortcut: Some("P"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "photo.reject",
        label: "Set Rejected Flag",
        category: CommandCategory::Library,
        default_shortcut: Some("X"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "photo.unflag",
        label: "Remove Flag",
        category: CommandCategory::Library,
        default_shortcut: Some("U"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "library.quick_collection",
        label: "Add to Quick Collection",
        category: CommandCategory::Library,
        default_shortcut: Some("B"),
        secondary_shortcut: None,
    },
    // Photo (develop)
    CommandSpec {
        id: "photo.auto_tone",
        label: "Auto Tone",
        category: CommandCategory::Photo,
        default_shortcut: Some("Cmd+U"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "photo.white_balance",
        label: "White Balance Selector",
        category: CommandCategory::Photo,
        default_shortcut: Some("W"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "photo.crop_rotate",
        label: "Crop & Straighten",
        category: CommandCategory::Photo,
        default_shortcut: Some("R"),
        secondary_shortcut: None,
    },
    CommandSpec { id: "mask.add", label: "New Mask", category: CommandCategory::Photo, default_shortcut: Some("K"), secondary_shortcut: None },
    CommandSpec {
        id: "develop.reset",
        label: "Reset",
        category: CommandCategory::Photo,
        default_shortcut: Some("Shift+Cmd+R"),
        secondary_shortcut: None,
    },
    // View
    CommandSpec { id: "view.grid", label: "Grid View", category: CommandCategory::View, default_shortcut: Some("G"), secondary_shortcut: None },
    CommandSpec { id: "view.loupe", label: "Loupe View", category: CommandCategory::View, default_shortcut: Some("E"), secondary_shortcut: None },
    CommandSpec {
        id: "photo.before_after",
        label: "Before/After",
        category: CommandCategory::View,
        default_shortcut: Some("Y"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "view.zoom_in",
        label: "Zoom In",
        category: CommandCategory::View,
        default_shortcut: Some("Cmd+Plus"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "view.fit",
        label: "Fit on Screen",
        category: CommandCategory::View,
        default_shortcut: Some("Cmd+0"),
        secondary_shortcut: None,
    },
];

pub fn find_command(id: &str) -> Option<&'static CommandSpec> {
    COMMAND_REGISTRY.iter().find(|cmd| cmd.id == id)
}

pub fn commands_by_category(category: CommandCategory) -> Vec<&'static CommandSpec> {
    COMMAND_REGISTRY.iter().filter(|cmd| cmd.category == category).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_command_ids_are_unique() {
        let mut ids = HashSet::new();
        for cmd in COMMAND_REGISTRY {
            assert!(ids.insert(cmd.id), "Duplicate command ID detected: {}", cmd.id);
        }
    }

    #[test]
    fn test_lookup_finds_all_commands() {
        for cmd in COMMAND_REGISTRY {
            let found = find_command(cmd.id);
            assert!(found.is_some());
            assert_eq!(found.map(|c| c.label), Some(cmd.label));
        }
    }

    #[test]
    fn test_categories_populated() {
        assert!(!commands_by_category(CommandCategory::File).is_empty());
        assert!(!commands_by_category(CommandCategory::Edit).is_empty());
        assert!(!commands_by_category(CommandCategory::Library).is_empty());
        assert!(!commands_by_category(CommandCategory::Photo).is_empty());
        assert!(!commands_by_category(CommandCategory::View).is_empty());
    }
}

//! Menu bar items and accessible hierarchy generation.

use crate::command_reg::{CommandCategory, commands_by_category};

#[derive(Clone, Debug, PartialEq)]
pub struct MenuItemDef {
    pub label: String,
    pub command_id: Option<&'static str>,
    pub shortcut: Option<String>,
    pub is_separator: bool,
    pub children: Vec<MenuItemDef>,
}

impl MenuItemDef {
    pub fn action(label: impl Into<String>, command_id: &'static str, shortcut: Option<&str>) -> Self {
        Self {
            label: label.into(),
            command_id: Some(command_id),
            shortcut: shortcut.map(|s| s.to_string()),
            is_separator: false,
            children: Vec::new(),
        }
    }

    pub fn separator() -> Self {
        Self { label: String::new(), command_id: None, shortcut: None, is_separator: true, children: Vec::new() }
    }
}

pub struct MenuCategoryDef {
    pub category: CommandCategory,
    pub title: &'static str,
    pub items: Vec<MenuItemDef>,
}

pub fn generate_main_menu() -> Vec<MenuCategoryDef> {
    vec![
        MenuCategoryDef { category: CommandCategory::File, title: "File", items: build_category_items(CommandCategory::File) },
        MenuCategoryDef { category: CommandCategory::Edit, title: "Edit", items: build_category_items(CommandCategory::Edit) },
        MenuCategoryDef { category: CommandCategory::Library, title: "Library", items: build_category_items(CommandCategory::Library) },
        MenuCategoryDef { category: CommandCategory::Photo, title: "Photo", items: build_category_items(CommandCategory::Photo) },
        MenuCategoryDef { category: CommandCategory::View, title: "View", items: build_category_items(CommandCategory::View) },
        MenuCategoryDef { category: CommandCategory::Window, title: "Window", items: build_category_items(CommandCategory::Window) },
        MenuCategoryDef { category: CommandCategory::Help, title: "Help", items: build_category_items(CommandCategory::Help) },
    ]
}

fn build_category_items(cat: CommandCategory) -> Vec<MenuItemDef> {
    commands_by_category(cat).into_iter().map(|cmd| MenuItemDef::action(cmd.label, cmd.id, cmd.default_shortcut)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command_reg::find_command;

    #[test]
    fn test_menu_generation_coverage() {
        let menus = generate_main_menu();
        assert_eq!(menus.len(), 7);
        for m in &menus {
            for it in &m.items {
                if !it.is_separator {
                    assert!(it.command_id.is_some());
                    assert!(!it.label.is_empty());
                }
            }
        }
        // Every item in the generated menu resolves to a registered command.
        for m in &menus {
            for it in &m.items {
                if let Some(id) = it.command_id {
                    assert!(find_command(id).is_some(), "Unknown command in menu: {id}");
                }
            }
        }
    }
}

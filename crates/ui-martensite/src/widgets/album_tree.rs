//! Folder/album hierarchy tree widget for the library sidebar.

#[derive(Clone, Debug, PartialEq)]
pub struct AlbumItemDef {
    pub id: u64,
    pub name: String,
    /// Photos contained in this album/folder.
    pub count: u32,
    pub is_folder: bool,
    pub expanded: bool,
    pub children: Vec<AlbumItemDef>,
}

pub struct AlbumTreeWidget {
    pub items: Vec<AlbumItemDef>,
    pub selected_album_id: Option<u64>,
}

impl AlbumTreeWidget {
    pub fn new() -> Self {
        Self { items: Vec::new(), selected_album_id: None }
    }

    pub fn select_album(&mut self, id: u64) {
        self.selected_album_id = Some(id);
    }

    pub fn toggle_expanded(&mut self, id: u64) {
        if let Some(item) = find_album_mut(&mut self.items, id) {
            item.expanded = !item.expanded;
        }
    }
}

impl Default for AlbumTreeWidget {
    fn default() -> Self {
        Self::new()
    }
}

fn find_album_mut(items: &mut [AlbumItemDef], id: u64) -> Option<&mut AlbumItemDef> {
    for item in items.iter_mut() {
        if item.id == id {
            return Some(item);
        }
        if let Some(found) = find_album_mut(&mut item.children, id) {
            return Some(found);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_album_tree_mutation() {
        let mut tree = AlbumTreeWidget::new();
        tree.items.push(AlbumItemDef {
            id: 1,
            name: "Travel 2026".to_string(),
            count: 0,
            is_folder: true,
            expanded: false,
            children: vec![AlbumItemDef { id: 2, name: "Iceland".to_string(), count: 142, is_folder: false, expanded: false, children: vec![] }],
        });

        tree.select_album(2);
        assert_eq!(tree.selected_album_id, Some(2));

        // Nested lookup finds the child inside the folder.
        tree.toggle_expanded(1);
        assert!(tree.items[0].expanded);
        tree.toggle_expanded(1);
        assert!(!tree.items[0].expanded);
    }
}

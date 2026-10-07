//! Decoupled RAW development command registry.

pub struct Command {
    pub id: &'static str,
    pub label: &'static str,
    pub shortcut: Option<&'static str>,
}

pub const DEVELOP_COMMANDS: &[Command] = &[
    Command { id: "photo.auto_tone", label: "Auto Tone", shortcut: Some("Cmd+U") },
    Command { id: "photo.white_balance", label: "White Balance Pipette", shortcut: Some("W") },
    Command { id: "photo.crop_rotate", label: "Crop & Straighten", shortcut: Some("R") },
    Command { id: "photo.before_after", label: "Toggle Before/After", shortcut: Some("Y") },
];

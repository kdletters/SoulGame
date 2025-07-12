use godot::prelude::*;

/// GameSignals is a singleton that manages global game events
#[derive(GodotClass)]
#[class(base=Node)]
pub struct GameSignals {
    // Base node fields
    base: Base<Node>,
}

#[godot_api]
impl INode for GameSignals {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
        }
    }

    fn ready(&mut self) {
        godot_print!("GameSignals singleton ready");
    }
}

#[godot_api]
impl GameSignals {
    #[signal]
    pub fn game_started();

    #[signal]
    pub fn game_victory();

    #[signal]
    pub fn game_failure();

    #[signal]
    pub fn game_stopped();

    /// Emit the game_started signal
    #[func]
    pub fn emit_game_started(&mut self) {
        godot_print!("Game started signal emitted");
        self.signals().game_started().emit();
    }

    /// Emit the game_victory signal
    #[func]
    pub fn emit_game_victory(&mut self) {
        godot_print!("Game victory signal emitted");
        self.signals().game_stopped().emit();
        self.signals().game_victory().emit();
    }

    /// Emit the game_failure signal
    #[func]
    pub fn emit_game_failure(&mut self) {
        godot_print!("Game failure signal emitted");
        self.signals().game_stopped().emit();
        self.signals().game_failure().emit();
    }
}

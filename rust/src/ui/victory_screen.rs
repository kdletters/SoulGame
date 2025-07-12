use godot::prelude::*;
use godot::classes::{Control, IControl, Button};
use crate::game_signals::GameSignals;

/// VictoryScreen is shown when the player wins the game
#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct VictoryScreen {
    base: Base<Control>,
    #[init(node = "/root/GlobalGameSignals")]
    game_signals: OnReady<Gd<GameSignals>>,
}

#[godot_api]
impl IControl for VictoryScreen {
    fn ready(&mut self) {
        // Hide the victory screen initially
        self.base_mut().set_visible(false);
        self.game_signals.signals().game_victory().connect_other(self, Self::show_screen);

        // Connect the restart button signal
        let restart_button = self.base().get_node_as::<Button>("RestartButton");

        restart_button.signals().pressed().connect_other(self, Self::on_restart_button_pressed);
    }
}

#[godot_api]
impl VictoryScreen {
    /// Show the victory screen
    #[func]
    pub fn show_screen(&mut self) {
        godot_print!("Showing victory screen");
        self.base_mut().set_visible(true);
    }

    #[func]
    pub fn on_restart_button_pressed(&mut self) {
        godot_print!("Restart button pressed");
        self.base_mut().set_visible(false);

        // We'll emit the game started signal to restart
        self.game_signals.bind_mut().emit_game_started();
    }
}

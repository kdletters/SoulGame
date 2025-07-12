use godot::prelude::*;
use godot::classes::{Control, IControl, Button};
use crate::game_signals::GameSignals;

/// FailureScreen is shown when the player loses the game
#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct FailureScreen {
    base: Base<Control>,
    #[init(node = "/root/GlobalGameSignals")]
    game_signals: OnReady<Gd<GameSignals>>,
}

#[godot_api]
impl IControl for FailureScreen {
    fn ready(&mut self) {

        // Hide the failure screen initially
        self.base_mut().set_visible(false);
        self.game_signals.signals().game_failure().connect_other(self, Self::show_screen);

        // Connect the retry button signal
        let retry_button = self.base().get_node_as::<Button>("RetryButton");
        retry_button.signals().pressed().connect_other(self, Self::on_failure_button_pressed);
    }
}

#[godot_api]
impl FailureScreen {
    /// Show the failure screen
    #[func]
    pub fn show_screen(&mut self) {
        godot_print!("Showing failure screen");
        self.base_mut().set_visible(true);
    }

    #[func]
    pub fn on_failure_button_pressed(&mut self) {
        godot_print!("Restart button pressed");
        self.base_mut().set_visible(false);

        // We'll emit the game started signal to restart
        self.game_signals.bind_mut().emit_game_started();
    }
}

use godot::prelude::*;
use godot::classes::{Control, IControl, Button};
use crate::game_signals::GameSignals;

/// StartMenu is the initial screen shown to the player
#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct StartMenu {
    base: Base<Control>,
}

#[godot_api]
impl IControl for StartMenu {
    fn ready(&mut self) {
        // Connect the start button signal
        let start_button = self.base().get_node_as::<Button>("StartButton");
        start_button.signals().pressed().connect_other(self, Self::on_start_button_pressed);
    }
}

#[godot_api]
impl StartMenu {
    #[func]
    fn on_start_button_pressed(&mut self) {
        godot_print!("Start button pressed");

        // Hide the start menu
        self.base_mut().set_visible(false);

        // Emit the game started signal
        GameSignals::singleton().signals().game_started().emit();
    }
}

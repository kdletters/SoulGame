use crate::game_signals::GameSignals;
use godot::classes::notify::Node3DNotification;
use godot::classes::{
    INode3D, InputEvent, InputEventKey, MeshInstance3D, Node3D, Timer,
};
use godot::global::Key;
use godot::prelude::*;

/// ButtonPart is a game part that requires the player to press a specific key within a time limit
#[derive(GodotClass)]
#[class(init, base=Node3D)]
pub struct ButtonPart {
    base: Base<Node3D>,
    #[init(node = "/root/GlobalGameSignals")]
    game_signals: OnReady<Gd<GameSignals>>,
    #[init(node = "Pillar")]
    pillar: OnReady<Gd<MeshInstance3D>>,
    timer: Option<Gd<Timer>>,
    #[init(val = Key::SPACE)]
    required_key: Key, // Key code that player needs to press
    #[init(val = 1.0)]
    time_limit: f64, // Time limit in seconds
    active: bool, // Whether the part is currently active
}

#[godot_api]
impl INode3D for ButtonPart {
    fn on_notification(&mut self, what: Node3DNotification) {
        match what {
            Node3DNotification::PREDELETE => {
                if let Some(timer) = &self.timer {
                    timer.clone().free();
                    self.timer = None;
                }
            }
            _ => {}
        }
    }

    fn ready(&mut self) {
        // Create a timer for the time limit
        let mut timer = Timer::new_alloc();
        timer.set_wait_time(self.time_limit);
        timer.set_one_shot(false);
        timer
            .signals()
            .timeout()
            .connect_other(self, Self::on_timer_timeout);
        self.base_mut().add_child(&timer);
        self.timer = Some(timer);

        self.game_signals
            .signals()
            .game_started()
            .connect_other(self, Self::start);
        self.game_signals
            .signals()
            .game_stopped()
            .connect_other(self, Self::stop);
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if !self.active {
            return;
        }

        // Check if the event is a key press
        if let Ok(key_event) = event.try_cast::<InputEventKey>() {
            if key_event.is_pressed()
                && !key_event.is_echo()
                && key_event.get_keycode() == self.required_key
            {
                godot_print!("Correct key pressed!");
                self.reset_timer();
            }
        }
    }
}

#[godot_api]
impl ButtonPart {
    /// Start the button part
    #[func]
    pub fn start(&mut self) {
        godot_print!("Button part started");
        self.active = true;
        self.reset_timer();
    }
    /// Stop the button part
    #[func]
    pub fn stop(&mut self) {
        godot_print!("Button part stopped");
        self.active = false;
        self.stop_timer();
    }

    /// Reset the timer
    #[func]
    pub fn reset_timer(&mut self) {
        if let Some(timer) = &mut self.timer {
            timer.start();
        }
    }

    /// Reset the timer
    #[func]
    pub fn stop_timer(&mut self) {
        if let Some(timer) = &mut self.timer {
            timer.stop();
        }
    }

    /// Called when the timer times out
    #[func]
    pub fn on_timer_timeout(&mut self) {
        if !self.active {
            return;
        }

        godot_print!("Time's up! Player failed to press the key in time.");
        self.active = false;

        let mut signal = self.game_signals.clone();
        // Emit the game failure signal
        godot::task::spawn(async move {
            signal.bind_mut().emit_game_failure();
        });
    }

    /// Set the required key
    #[func]
    pub fn set_required_key(&mut self, key: Key) {
        self.required_key = key;
    }

    /// Set the time limit
    #[func]
    pub fn set_time_limit(&mut self, time: f64) {
        self.time_limit = time;
        if let Some(timer) = &mut self.timer {
            timer.set_wait_time(time);
        }
    }
}

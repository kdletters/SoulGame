use crate::game_signals::GameSignals;
use godot::classes::notify::Node3DNotification;
use godot::classes::{Engine, INode3D, InputEvent, InputEventKey, MeshInstance3D, Node3D, StandardMaterial3D, Timer};
use godot::global::Key;
use godot::prelude::*;
use godot_tokio::AsyncRuntime;

/// ButtonPart is a game part that requires the player to press a specific key within a time limit
#[derive(GodotClass)]
#[class(init, base=Node3D)]
pub struct ButtonPart {
    base: Base<Node3D>,
    #[init(node = "%Pillar")]
    pillar: OnReady<Gd<MeshInstance3D>>,
    timer: Option<Gd<Timer>>,
    #[init(val = Key::SPACE)]
    required_key: Key, // Key code that player needs to press
    #[init(val = 1.0)]
    time_limit: f64, // Time limit in seconds
    active: bool, // Whether the part is currently active

    pressed: bool,
}

#[godot_api]
impl INode3D for ButtonPart {
    fn on_notification(&mut self, what: Node3DNotification) {
        match what {
            Node3DNotification::PREDELETE => {
                if let Some(timer) = &self.timer {
                    godot_print!("ButtonPart free timer");
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

        GameSignals::singleton()
            .signals()
            .game_started()
            .connect_other(self, Self::start);
        GameSignals::singleton()
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
                self.pressed = true;
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
        self.pillar.get_material_override().unwrap().cast::<StandardMaterial3D>().set_emission_energy_multiplier(10.0);
    }
    /// Stop the button part
    #[func]
    pub fn stop(&mut self) {
        godot_print!("Button part stopped");
        self.active = false;
        self.stop_timer();
        self.pillar.get_material_override().unwrap().cast::<StandardMaterial3D>().set_emission_energy_multiplier(0.0);
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
    pub fn on_timer_timeout(&mut self) {
        if !self.active {
            return;
        }

        if self.pressed {
            godot_print!("abaaba");
            self.pressed = false;
            return;
        }

        self.stop_timer();
        godot::task::spawn(async move {
            AsyncRuntime::spawn(async {
                tokio::time::sleep(tokio::time::Duration::from_micros(1)).await;
            })
            .await
            .unwrap();
            godot_print!("Time's up! Player failed to press the key in time.");
            GameSignals::singleton().signals().game_failure().emit();
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

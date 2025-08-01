use crate::game_signals::GameSignals;
use godot::classes::Timer;
use godot::prelude::*;
use crate::godot_print_err;

#[derive(GodotClass)]
#[class(init, base = Node3D)]
pub struct Game {
    base: Base<Node3D>,

    #[export]
    time_limit: f64,
    timer: Option<Gd<Timer>>,
}

#[godot_api]
impl INode3D for Game {
    fn ready(&mut self) {
        GameSignals::singleton()
            .signals()
            .game_started()
            .connect_other(self, Self::start);
        GameSignals::singleton()
            .signals()
            .game_stopped()
            .connect_other(self, Self::stop);

        GameSignals::singleton()
            .signals()
            .game_victory()
            .connect_self(|this| {
                this.signals().game_stopped().emit();
            });

        GameSignals::singleton()
            .signals()
            .game_failure()
            .connect_self(|this| {
                this.signals().game_stopped().emit();
            });
    }
}

impl Game {
    fn start(&mut self) {
        let mut timer = Timer::new_alloc();
        timer.set_wait_time(self.time_limit);
        timer.set_one_shot(true);
        timer.set_autostart(true);
        self.base_mut().add_child(&timer);
        self.timer = Some(timer.clone());
        timer.signals().timeout().connect(|| {
            GameSignals::singleton().signals().game_victory().emit();
        });
    }

    fn stop(&mut self) {
        if let Some(timer) = &self.timer {
            timer.clone().free();
            self.timer = None;
        }
    }
}
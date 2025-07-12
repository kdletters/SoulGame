use godot::classes::Timer;
use godot::prelude::*;
use crate::game_signals::GameSignals;

#[derive(GodotClass)]
#[class(init, base = Node3D)]
pub struct Game {
    base: Base<Node3D>,

    #[init(node = "/root/GlobalGameSignals")]
    game_signals: OnReady<Gd<GameSignals>>,
    #[export]
    time_limit: f64,
    timer: Option<Gd<Timer>>,
}

#[godot_api]
impl INode3D for Game {
    fn ready(&mut self) {
        self.game_signals.signals().game_started().connect_other(self, Self::start);
        self.game_signals.signals().game_stopped().connect_other(self, Self::stop);
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
        let mut signal = self.game_signals.clone();
        godot::task::spawn(async move {
            timer.signals().timeout().to_future().await;
            signal.bind_mut().emit_game_victory();
        });
    }

    fn stop(&mut self) {
        if let Some(timer) = &self.timer {
            timer.clone().free();
            self.timer = None;
        }
    }
}
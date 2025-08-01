use crate::game_signals::GameSignals;
use godot::classes::{
    Engine, GDScript, INode3D, Input, InputEvent, MeshInstance3D, Node3D, StandardMaterial3D, Time,
    Timer,
};
use godot::prelude::*;
use std::f64::consts::PI;
use std::ops::Range;

/// ButtonPart is a game part that requires the player to press a specific key within a time limit
#[derive(GodotClass)]
#[class(init, base=Node3D)]
pub struct PinkButton {
    base: Base<Node3D>,
    #[init(node = "%Pillar")]
    pillar: OnReady<Gd<MeshInstance3D>>,
    #[init(val = Timer::new_alloc())]
    timer: Gd<Timer>,
    #[init(val = 0.5..3.0)]
    time_scale_range: Range<f64>,
    #[init(val = 0.1)]
    scale_delta: f64,
    active: bool, // Whether the part is currently active
}

#[godot_api]
impl INode3D for PinkButton {
    fn ready(&mut self) {
        self.timer.set_wait_time(1.0);
        self.timer.set_one_shot(false);
        self.timer
            .signals()
            .timeout()
            .connect_other(self, Self::on_timeout);
        let timer = self.timer.clone();
        self.base_mut().add_child(&timer);

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

        // 检查特定键是否被按下
        if Input::singleton().is_action_just_pressed("pink_button") {
            if Engine::singleton().get_time_scale() <= self.time_scale_range.start {
                return;
            }
            self.decrease_global_time_scale();
            self.rotate_counterclockwise();
        }
    }
}

impl PinkButton {
    fn on_timeout(&mut self) {
        if !self.active {
            return;
        }

        if Engine::singleton().get_time_scale() >= self.time_scale_range.end {
            return;
        }

        godot_print!("pink button timeout");
        self.increase_global_time_scale();
        self.rotate_clockwise();
    }

    fn start(&mut self) {
        godot_print!("pink button started");
        self.active = true;
        self.timer.start();

        Engine::singleton().set_time_scale(1.0);
    }

    fn stop(&mut self) {
        godot_print!("pink button stopped");
        self.active = false;
        self.timer.stop();

        Engine::singleton().set_time_scale(1.0);
    }

    fn increase_global_time_scale(&mut self) {
        let cur_scale = Engine::singleton().get_time_scale() + self.scale_delta;
        Engine::singleton().set_time_scale(cur_scale);
    }

    fn decrease_global_time_scale(&mut self) {
        let cur_scale = Engine::singleton().get_time_scale() - self.scale_delta;
        Engine::singleton().set_time_scale(cur_scale);
    }

    fn rotate_clockwise(&mut self) {
        let angle = (PI * self.scale_delta) as f32;
        self.base_mut().rotate_z(angle);
    }

    fn rotate_counterclockwise(&mut self) {
        let angle = (PI * self.scale_delta) as f32;
        self.base_mut().rotate_z(-angle);
    }
}
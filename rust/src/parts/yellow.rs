use crate::game_signals::GameSignals;
use godot::classes::notify::Node3DNotification;
use godot::classes::{
    INode3D, Input, MeshInstance3D, Node3D, StandardMaterial3D,
    Time, Timer,
};
use godot::global::{randf_range, Key};
use godot::prelude::*;

/// ButtonPart is a game part that requires the player to press a specific key within a time limit
#[derive(GodotClass)]
#[class(init, base=Node3D)]
pub struct YellowButton {
    base: Base<Node3D>,
    #[init(node = "%Pillar")]
    pillar: OnReady<Gd<MeshInstance3D>>,
    #[init(val = Timer::new_alloc())]
    timer1: Gd<Timer>,
    #[init(val = Timer::new_alloc())]
    timer2: Gd<Timer>,
    #[init(val = (5.0, 10.0))]
    delay_time: (f64, f64), // Time limit in seconds
    #[init(val = 5.0)]
    time_limit: f64, // Time limit in seconds
    #[init(val = 2.0)]
    press_time: f64, // Time limit in seconds
    active: bool, // Whether the part is currently active

    pressed: bool,
    press_start_time: f64,
}

#[godot_api]
impl INode3D for YellowButton {
    fn on_notification(&mut self, what: Node3DNotification) {
        match what {
            Node3DNotification::PREDELETE => {
                self.timer1.clone().free();
            }
            _ => {}
        }
    }

    fn process(&mut self, delta: f64) {
        if !self.active {
            return;
        }

        // 检查特定键是否被按下
        if Input::singleton().is_action_pressed("yellow_button") {
            // 如果键刚开始被按下，记录开始时间
            if !self.pressed {
                self.pressed = true;
                self.press_start_time = Time::singleton().get_unix_time_from_system();
                return;
            }

            // 计算按下持续时间
            let current_duration =
                Time::singleton().get_unix_time_from_system() - self.press_start_time;

            // 检查是否达到所需持续时间
            if current_duration >= self.press_time {
                godot_print!("按键已持续按下2秒钟！");
                // 执行你的操作...
                // 重置状态（如果你只想触发一次）
                self.start();
            }
        } else {
            // 键被释放，重置状态
            if self.pressed {
                self.stop();
                GameSignals::singleton().bind_mut().emit_game_failure();
            }
        }
    }

    fn ready(&mut self) {
        // Create a timer for the time limit
        self.timer1.set_one_shot(true);
        self.timer1
            .signals()
            .timeout()
            .connect_other(self, Self::start_check_press);
        let timer = self.timer1.clone();
        self.base_mut().add_child(&timer);

        self.timer2.set_wait_time(self.time_limit);
        self.timer2.set_one_shot(true);
        self.timer2
            .signals()
            .timeout()
            .connect_other(self, Self::on_timeout);
        let timer = self.timer2.clone();
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
}

#[godot_api]
impl YellowButton {
    pub fn start(&mut self) {
        godot_print!("yellow button started");
        // 延时5~10秒
        self.active = false;
        self.pressed = false;
        self.press_start_time = 0.0;
        self.timer1.set_wait_time(randf_range(5.0, 10.0));
        self.reset_timer();
        self.light_off();
    }

    pub fn stop(&mut self) {
        godot_print!("yellow button stopped");
        self.active = false;
        self.pressed = false;
        self.stop_timer();
        self.light_off();
    }
    pub fn reset_timer(&mut self) {
        godot_print!("yellow button reset timer");
        self.timer1.start();
        self.timer2.stop();
    }

    fn stop_timer(&mut self) {
        godot_print!("yellow button stop timer");
        self.timer1.stop();
        self.timer2.stop();
    }

    fn start_check_press(&mut self) {
        godot_print!("yellow button start check press");
        self.active = true;
        // 5秒内单次按下满足2秒则重新启动，否则失败
        self.light_on();
        self.timer2.start();
    }

    fn on_timeout(&mut self) {
        if !self.active {
            return;
        }

        if self.pressed {
            self.pressed = false;
            return;
        }

        godot_print!("yellow button timeout");
        GameSignals::singleton().bind_mut().emit_game_failure();
    }

    fn light_on(&mut self) {
        godot_print!("yellow button light on");
        self.pillar
            .get_material_override()
            .unwrap()
            .cast::<StandardMaterial3D>()
            .set_emission_energy_multiplier(10.0);
    }

    fn light_off(&mut self) {
        godot_print!("yellow button light off");
        self.pillar
            .get_material_override()
            .unwrap()
            .cast::<StandardMaterial3D>()
            .set_emission_energy_multiplier(0.0);
    }
}
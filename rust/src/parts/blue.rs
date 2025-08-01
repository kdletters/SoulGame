/*!
插上时充电，拔掉时放电，需要令其电量不完全满或完全空，否则失败
初始数值50，归零或达到100失败
按下按键每秒+10（匀速上升），松开每秒-10（匀速下降）
*/

use crate::game_signals::GameSignals;
use godot::classes::notify::Node3DNotification;
use godot::classes::{
    Engine, INode3D, Input, InputEvent, MeshInstance3D, Node3D, ProgressBar, StandardMaterial3D,
    Time, Timer,
};
use godot::global::{Key, randf_range};
use godot::prelude::*;
use std::ops::Range;

#[derive(GodotClass)]
#[class(init, base=Node3D)]
pub struct BlueButton {
    base: Base<Node3D>,
    #[init(node = "%Pillar")]
    pillar: OnReady<Gd<MeshInstance3D>>,
    #[init(node = "%ProgressBar")]
    progress_bar: OnReady<Gd<ProgressBar>>,
    /// 自动减少电量
    #[init(val = Timer::new_alloc())]
    auto_timer: Gd<Timer>,
    /// 按下按键时增加电量
    #[init(val = Timer::new_alloc())]
    manual_timer: Gd<Timer>,
    #[init(val = 0.0..100.0)]
    power_range: Range<f64>, // Time limit in seconds
    #[init(val = 50.0)]
    default_power: f64, // Time limit in seconds
    #[init(val = 10.0)]
    change_rate: f64, // Time limit in seconds
    power: f64,
    active: bool, // Whether the part is currently active
}

#[godot_api]
impl INode3D for BlueButton {
    fn ready(&mut self) {
        self.auto_timer.set_wait_time(1.0);
        self.auto_timer.set_one_shot(false);
        self.auto_timer.set_autostart(false);
        self.auto_timer
            .signals()
            .timeout()
            .connect_other(self, Self::decrease_power);
        let timer = self.auto_timer.clone();
        self.base_mut().add_child(&timer);

        self.manual_timer.set_wait_time(1.0);
        self.manual_timer.set_one_shot(false);
        self.manual_timer.set_autostart(false);
        self.manual_timer
            .signals()
            .timeout()
            .connect_other(self, Self::increase_power);
        let timer = self.manual_timer.clone();
        self.base_mut().add_child(&timer);

        self.progress_bar.set_min(self.power_range.start);
        self.progress_bar.set_max(self.power_range.end);

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
        if Input::singleton().is_action_just_pressed("blue_button") {
            godot_print!("blue button pressed");
            self.manual_timer.start();
        }
    }
}

impl BlueButton {
    fn start(&mut self) {
        godot_print!("blue button started");
        self.power = self.default_power;
        self.progress_bar.set_value(self.default_power);

        self.auto_timer.start();
        self.active = true;
    }

    fn stop(&mut self) {
        godot_print!("blue button stopped");
        self.active = false;
        self.auto_timer.stop();
        self.manual_timer.stop();
    }

    fn increase_power(&mut self) {
        godot_print!("blue button increase power");
        self.power += self.change_rate;
        self.update_power();
    }

    fn decrease_power(&mut self) {
        godot_print!("blue button decrease power");
        self.power -= self.change_rate;
        self.update_power();
    }

    fn update_power(&mut self) {
        godot_print!("blue button power: {}", self.power);
        if self.power >= self.power_range.end || self.power <= self.power_range.start {
            GameSignals::singleton().bind_mut().emit_game_failure();
            return;
        }

        self.progress_bar.set_value(self.power);
    }
}
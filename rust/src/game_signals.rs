use godot::classes::Engine;
use godot::prelude::*;
use godot_tokio::AsyncRuntime;
use std::time::Duration;

/// GameSignals is a singleton that manages global game events
#[derive(GodotClass)]
#[class(base=Node)]
pub struct GameSignals {
    // Base node fields
    base: Base<Node>,
}

#[godot_api]
impl INode for GameSignals {
    fn init(base: Base<Node>) -> Self {
        Self { base }
    }

    fn ready(&mut self) {
        godot_print!("GameSignals singleton ready");
    }
}

#[godot_api]
impl GameSignals {
    pub const SINGLETON: &'static str = "Signals";
    pub fn singleton() -> Gd<GameSignals> {
        match Engine::singleton().get_singleton(Self::SINGLETON) {
            Some(singleton) => singleton.cast::<Self>(),
            None => panic!("Can not find GameSignals"),
        }
    }

    #[signal]
    pub fn game_started();

    #[signal]
    pub fn game_victory();

    #[signal]
    pub fn game_failure();

    #[signal]
    pub fn game_stopped();
}

impl GameSignals {
    pub fn emit_game_failure(&mut self) {
        godot::task::spawn(async {
            AsyncRuntime::runtime()
                .spawn(async {
                    tokio::time::sleep(Duration::from_micros(1)).await;
                })
                .await
                .unwrap();
            GameSignals::singleton().signals().game_failure().emit()
        });
    }
}

mod game_controller;
mod game_signals;
mod parts;
mod ui;

use crate::game_signals::GameSignals;
use godot::classes::Engine;
use godot::prelude::*;
use godot_tokio::AsyncRuntime;

#[macro_export]
macro_rules! godot_print_err {
    ($fmt:literal $(, $args:expr)* $(,)?) => {
        godot::global::printerr(&[
            godot::builtin::Variant::from(
                format!($fmt $(, $args)*)
            )
        ])
    };
}
struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {
    fn on_level_init(level: InitLevel) {
        match level {
            InitLevel::Scene => {
                let mut engine = Engine::singleton();
                Self::register_async_runtime(&mut engine);
                Self::register_game_signals(&mut engine);
            }
            _ => (),
        }
    }

    fn on_level_deinit(level: InitLevel) {
        match level {
            InitLevel::Scene => {
                let mut engine = Engine::singleton();
                Self::unregister_async_runtime(&mut engine);
                Self::unregister_game_signals(&mut engine);
            }
            _ => (),
        }
    }
}

impl MyExtension {
    fn register_async_runtime(engine: &mut Gd<Engine>) {
        engine.register_singleton(AsyncRuntime::SINGLETON, &AsyncRuntime::new_alloc());
    }

    fn register_game_signals(engine: &mut Gd<Engine>) {
        engine.register_singleton(GameSignals::SINGLETON, &GameSignals::new_alloc());
    }

    fn unregister_async_runtime(engine: &mut Gd<Engine>) {
        if let Some(async_singleton) = engine.get_singleton(AsyncRuntime::SINGLETON) {
            engine.unregister_singleton(AsyncRuntime::SINGLETON);
            async_singleton.free();
        } else {
            godot_print!(
                "Failed to find & free singleton -> {}",
                AsyncRuntime::SINGLETON
            );
        }
    }

    fn unregister_game_signals(engine: &mut Gd<Engine>) {
        if let Some(game_signals) = engine.get_singleton(GameSignals::SINGLETON) {
            engine.unregister_singleton(GameSignals::SINGLETON);
            game_signals.free();
        } else {
            godot_print!(
                "Failed to find & free singleton -> {}",
                GameSignals::SINGLETON
            );
        }
    }
}

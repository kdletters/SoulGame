mod parts;
mod game_signals;
mod ui;
mod game_controller;

use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

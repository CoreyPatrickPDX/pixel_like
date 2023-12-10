use gdnative::init::{godot_gdnative_init, godot_gdnative_terminate, godot_nativescript_init};
use gdnative::prelude::*;

mod enemy;
mod main_scene;
mod player;

fn init(handle: InitHandle) {
    // Register classes for use in Godot
    handle.add_class::<player::Player>();
    handle.add_class::<main_scene::Main>();
    handle.add_class::<enemy::Enemy>();
}

// Initialize Godot native library
godot_gdnative_init!();
// Initialize the native scripts
godot_nativescript_init!(init);
// Terminate Godot native library
godot_gdnative_terminate!();

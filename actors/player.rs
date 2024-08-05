// Purpose: Defines the Player actor, which represents the user-controlled
// character in the game.
//
// Contents:
// - Player struct: Represents a player in the game.
// - Implementation of the Actor trait for Player.
//
// When to use:
// - Use this as a template for creating the main player character.
// - Extend or modify this struct to add player-specific functionality.
// - This example demonstrates how to implement an actor that uses tick-based updates.

use super::base::{Actor, BaseActor, Timer};
use std::time::Duration;

pub struct Player {
    base: BaseActor,
    inventory: Vec<String>,
    timer: Timer,
}

impl Player {
    pub fn new(position: (f32, f32), health: f32) -> Self {
        Self {
            base: BaseActor::new(position, health),
            inventory: Vec::new(),
            timer: Timer::new(Duration::from_secs(1)),
        }
    }
}

impl Actor for Player {
    fn init(&mut self) {
        println!("Player initialized at position: {:?}", self.base.position);
    }

    fn update(&mut self) {
        // Regular update logic
    }

    fn render(&self) {
        // Render logic
    }

    fn on_tick(&mut self) {
        println!("Player ticked! Position: {:?}", self.base.position);
    }

    fn uses_tick(&self) -> bool {
        true
    }

    fn get_timer(&mut self) -> Option<&mut Timer> {
        Some(&mut self.timer)
    }
}
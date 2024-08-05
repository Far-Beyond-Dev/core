// Purpose: This file defines the core components of the actor system.
//
// Contents:
// - Timer struct: Handles interval-based events for actors.
// - BaseActor struct: Provides common properties for all actors.
// - Actor trait: Defines the interface that all actors must implement.
//
// When to use:
// - Use Timer when you need to implement interval-based actions for an actor.
// - Use BaseActor as a foundation for creating specific actor types.
// - Implement the Actor trait for any entity in your game that needs to be
//   updated, rendered, or perform interval-based actions.
//
// Note: This system is designed to be flexible, allowing for both actors that
// use tick-based updates and those that don't, with minimal performance overhead.

use std::time::{Duration, Instant};

pub struct Timer {
    last_tick: Instant, 
    interval: Duration,
}

impl Timer {
    pub fn new(interval: Duration) -> Self {
        Self {
            last_tick: Instant::now(),
            interval,
        }
    }

    pub fn tick(&mut self) -> bool {
        let now = Instant::now();
        if now.duration_since(self.last_tick) >= self.interval {
            self.last_tick = now;
            true
        } else {
            false
        }
    }
}

pub struct BaseActor {
    pub position: (f32, f32),
    pub health: f32,
}

impl BaseActor {
    pub fn new(position: (f32, f32), health: f32) -> Self {
        Self {
            position,
            health,
        }
    }
}

pub trait Actor {
    fn init(&mut self);
    fn update(&mut self);
    fn render(&self);
    fn on_tick(&mut self) {}
    fn uses_tick(&self) -> bool {
        false
    }
    fn get_timer(&mut self) -> Option<&mut Timer> {
        None
    }
}

use gdnative::api::Node2D;
use gdnative::prelude::*;
use rand::*;

/// Represents an enemy character in the game.
#[derive(NativeClass)]
#[inherit(Node2D)]
#[user_data(user_data::MutexData<Enemy>)]
#[register_with(Self::register_enemy)]
pub struct Enemy {
    #[property(default = 400.0)]
    speed: f32,

    screen_size: Vector2,
    current_direction: Vector2,

    #[property]
    color: Color, // New property for the enemy color
}

#[methods]
impl Enemy {
    /// Register signals and properties for the Enemy class.
    fn register_enemy(builder: &ClassBuilder<Self>) {
        builder.signal("hit").done();
    }

    fn new(_owner: &Node2D) -> Self {
        Enemy {
            speed: 400.0,
            screen_size: Vector2::new(0.0, 0.0),
            color: Color::from_rgb(1.0, 0.0, 0.0), // Change color for enemy
            current_direction: Vector2::new(1.0, 0.0),
        }
    }

    /// Create a new instance of the Enemy class.
    #[method]
    fn _ready(&mut self, #[base] owner: &Node2D) {
        let viewport = owner.get_viewport_rect();
        self.screen_size = viewport.size;

        // Set random initial position for the enemy within screen bounds
        let mut rng = thread_rng();
        let random_position = Vector2::new(
            rng.gen_range(0.0..=(self.screen_size.x - 50.0)), // Adjusted to consider enemy size
            rng.gen_range(0.0..=(self.screen_size.y - 50.0)), // Adjusted to consider enemy size
        );

        owner.set_global_position(random_position);
    }

    /// Processing method called on every frame update.
    #[method]
    unsafe fn _process(&mut self, #[base] owner: &Node2D, delta: f32) {
        // Gradual change in direction for smooth movement
        let mut rng = thread_rng();
        if rng.gen::<f32>() < 0.02 {
            self.current_direction =
                Vector2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)).normalized();
        }

        // Adjust the speed based on the distance from the player
        let player_position =
            owner
                .get_node("/root/Viewport/CanvasLayer/Player")
                .map(|node| unsafe {
                    node.assume_safe()
                        .cast::<Node2D>()
                        .expect("Failed to cast to Node2D")
                });
        if let Some(player_position) = player_position {
            let distance_to_player = owner
                .global_position()
                .distance_to(player_position.global_position());
            let adjusted_speed = (self.speed / 2.0) * (1.0 / (distance_to_player + 1.0)); // Speed decreases as the enemy gets closer
            let velocity = self.current_direction * adjusted_speed * delta;

            let new_position = owner.global_position() + velocity;

            // Keep the enemy within the screen bounds
            let position = Vector2::new(
                new_position.x.clamp(0.0, self.screen_size.x),
                new_position.y.clamp(0.0, self.screen_size.y),
            );

            owner.set_global_position(position);
        }
    }

    #[method]
    fn _draw(&self, #[base] owner: &Node2D) {
        // Draw a colored rectangle for the enemy at its current position
        let rect_size = Vector2::new(50.0, 50.0);
        let position = owner.global_position();

        owner.draw_rect(
            Rect2::new(position, rect_size),
            self.color,
            true,
            1.0,
            false,
        );
    }
}

#[cfg(test)]
mod tests {
    use gdnative::{
        api::Node2D,
        core_types::{Color, Vector2},
    };

    use super::enemy::Enemy;
    #[test]
    fn test_enemy_creation() {
        // Test Enemy creation and initialization
        let owner = Node2D::new();
        let enemy = Enemy::new(&owner);

        assert_eq!(enemy.speed, 400.0);
        assert_eq!(enemy.color, Color::from_rgb(1.0, 0.0, 0.0));
        assert_eq!(enemy.current_direction, Vector2::new(1.0, 0.0));
    }
}

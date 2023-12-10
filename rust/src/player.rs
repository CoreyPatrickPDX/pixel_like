use gdnative::api::Node2D;
use gdnative::prelude::*;

/// Represents the player character in the game.
#[derive(NativeClass)]
#[inherit(Node2D)]
#[user_data(user_data::MutexData<Player>)]
#[register_with(Self::register_player)]
pub struct Player {
    #[property(default = 400.0)]
    speed: f32,

    screen_size: Vector2,

    #[property]
    color: Color, // New property for the player color

    size: Vector2,
}

#[methods]
impl Player {
    /// Register signals and properties for the Player class.
    fn register_player(builder: &ClassBuilder<Self>) {
        builder.signal("hit").done()
    }

    /// Create a new instance of the Player class.
    fn new(_owner: &Node2D) -> Self {
        Player {
            speed: 400.0,
            screen_size: Vector2::new(0.0, 0.0),
            color: Color::from_rgb(0.0, 1.0, 0.0),
            size: Vector2::new(50.0, 50.0),
        }
    }

    /// Initialization method called when the node is added to the scene.
    #[method]
    fn _ready(&mut self, #[base] owner: &Node2D) {
        let viewport = owner.get_viewport_rect();
        godot_print!("Viewport Size: {:?}", viewport);
        self.screen_size = viewport.size;
        godot_print!("Screen Size: {:?}", self.screen_size);
        let position = (self.screen_size - self.size) / 2.0;
        godot_print!("Position: {:?}", position);
        owner.set_global_position(Vector2::new(1.0, 1.0));
    }

    /// Processing method called on every frame update.
    #[method]
    unsafe fn _process(&mut self, #[base] owner: &Node2D, delta: f32) {
        let input = Input::godot_singleton();
        let mut velocity = Vector2::new(0.0, 0.0);

        if Input::is_action_pressed(input, "ui_right", false) {
            velocity.x += 1.0;
        }
        if Input::is_action_pressed(input, "ui_left", false) {
            velocity.x -= 1.0;
        }
        if Input::is_action_pressed(input, "ui_down", false) {
            velocity.y += 1.0;
        }
        if Input::is_action_pressed(input, "ui_up", false) {
            velocity.y -= 1.0;
        }

        if velocity.length() > 0.0 {
            velocity = velocity.normalized() * self.speed;
        }

        let change = velocity * delta;
        let new_position = owner.global_position() + change;
        // Calculate the playable area based on the viewport size
        let half_size = self.size / 2.0;
        let min_position = half_size;
        let max_position = self.screen_size - half_size;

        // Clamp the new position within the playable area
        let clamped_position = Vector2::new(
            new_position.x.clamp(min_position.x, max_position.x),
            new_position.y.clamp(min_position.y, max_position.y),
        );
        owner.set_global_position(clamped_position);
    }

    /// Drawing method to render the player on the screen.
    #[method]
    fn _draw(&self, #[base] owner: &Node2D) {
        // Calculate the position to center the rectangle on the screen
        let position = (self.screen_size - self.size) / 2.0;

        // Draw a colored rectangle
        owner.draw_rect(Rect2::new(position, self.size), self.color, true, 1.0, true);
    }

    /// Start the player at a specific position.
    #[method]
    pub fn start(&self, #[base] owner: &Node2D, pos: Vector2) {
        owner.set_global_position(pos);
        owner.show();
    }
}

#[cfg(test)]
mod tests {
    use super::player::Player;
    use gdnative::{
        api::Node2D,
        core_types::{Color, Vector2},
    };

    #[test]
    fn test_player_creation() {
        // Test Player creation and initialization
        let owner = Node2D::new();
        let player = Player::new(&owner);

        assert_eq!(player.speed, 400.0);
        assert_eq!(player.color, Color::from_rgb(0.0, 1.0, 0.0));
        assert_eq!(player.size, Vector2::new(50.0, 50.0));
    }
}

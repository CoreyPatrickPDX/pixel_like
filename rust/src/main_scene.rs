use crate::player;
use gdnative::{api::Position2D, prelude::*};

/// Represents the main scene of the game.
#[derive(NativeClass)]
#[inherit(Node)]
#[user_data(user_data::LocalCellData<Main>)]
pub struct Main {
    score: i64,
}

#[methods]
impl Main {
    /// Create a new instance of the Main scene.
    fn new(_owner: &Node) -> Self {
        Main { score: 0 }
    }

    /// Start a new game within the main scene.
    #[method]
    fn new_game(&mut self, #[base] owner: &Node) {
        let start_position = unsafe { owner.get_node_as::<Position2D>("start_position").unwrap() };
        let player = unsafe {
            owner
                .get_node_as_instance::<player::Player>("player")
                .unwrap()
        };
        let start_timer = unsafe { owner.get_node_as::<Timer>("start_timer").unwrap() };

        self.score = 0;

        player
            .map(|x, o| x.start(&o, start_position.position()))
            .ok()
            .unwrap_or_else(|| godot_print!("Unable to get player"));

        start_timer.start(0.0);
    }
}

#[cfg(test)]
mod tests {
    use gdnative::api::Node;

    use super::main_scene::Main;
    #[test]
    fn test_main_scene_creation() {
        // Test Main creation and initialization
        let owner = Node::new();
        let main_scene = Main::new(&owner);

        assert_eq!(main_scene.score, 0);
    }
}

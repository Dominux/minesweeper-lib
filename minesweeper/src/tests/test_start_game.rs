use crate::{game::Game, cell::Coordinates};

#[test]
fn test_start_game() {
    let mut game = Game::new(10, 10, 10);
	let coordinates = Coordinates{column: 3, row: 2};
	let first_open_result = game.open_cell(coordinates);
	assert!(!first_open_result)
}

use crate::{
    cell::Coordinates,
    game::Game,
    random_chooser::RandRandomChooser,
};

static RANDOM_CHOOSER: RandRandomChooser = RandRandomChooser {};

#[test]
fn test_start_game() {
	// TODO: Add special struct with specific "random" chooser, that will be known at compilation time

    let mut game = Game::new(10, 10, 10, &RANDOM_CHOOSER);
    let coordinates = Coordinates { column: 3, row: 2 };
    let first_open_result = game.open_cell(coordinates);
    assert!(!first_open_result)
}

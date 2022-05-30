use crate::tests::factories::not_random_chooser::NotRandomChooser;
use crate::view::TerminalViewer;
use crate::{cell::Coordinates, game::Game, random_chooser::RandRandomChooser};

static RANDOM_CHOOSER: RandRandomChooser = RandRandomChooser {};

#[test]
fn test_start_game() {
    let mut game = Game::new(10, 10, 10, &RANDOM_CHOOSER);
    let coordinates = Coordinates { column: 3, row: 2 };
    let first_open_result = game.open_cell(coordinates);
    assert!(!first_open_result)
}

#[test]
fn test_bombs_neighbors() {
    // Consider the field to be like this:
    let expected_field = "\
        1 1 1 0 0 0 1 1 1 0\n\
        1 * 1 0 0 0 1 * 1 0\n\
        2 2 2 0 0 0 1 1 1 0\n\
        1 * 1 0 0 0 0 0 0 0\n\
        2 2 2 0 0 1 1 1 0 0\n\
        2 * 1 0 0 1 * 2 1 0\n\
        * 2 1 1 1 2 2 * 1 0\n\
        1 1 0 2 * 3 2 2 1 0\n\
        0 0 0 2 * 3 * 1 0 0\n\
        0 0 0 1 1 2 1 1 0 0\
    ";

    // So bombs should be the next indexes:
    let choosen_result = vec![11, 17, 31, 51, 56, 60, 67, 74, 84, 86];
    let random_chooser = NotRandomChooser::new(choosen_result);

    let mut game = Game::new(10, 10, 10, &random_chooser);
    let coordinates = Coordinates { column: 3, row: 2 };
    let first_open_result = game.open_cell(coordinates);
    assert!(!first_open_result);

    let view = TerminalViewer::view(game.field);
    println!("{view}");
    assert_eq!(view, expected_field);
}

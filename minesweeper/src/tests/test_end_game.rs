use crate::{
    cell::Coordinates,
    game::{Game, GameResult},
    tests::factories::not_random_chooser::NotRandomChooser,
};

#[test]
fn test_win_game() {
    /*
    Consider the field to be like this:

       1 1 1 0 0 0 0 0 0 0
       1 * 1 0 0 0 0 0 0 0
       2 2 2 0 0 0 0 0 0 0
       1 * 1 0 0 0 0 0 0 0
       2 2 2 0 0 1 1 1 0 0
       2 * 1 0 0 1 * 2 1 0
       * 2 1 1 1 2 2 * 1 0
       1 1 0 2 * 3 2 2 1 0
       0 0 0 2 * 3 * 1 1 1
       0 0 0 1 1 2 1 1 1 *

    */

    // So bombs should be the next indexes:
    let choosen_result = vec![11, 17, 31, 51, 56, 60, 67, 74, 84, 86];
    let random_chooser = NotRandomChooser::new(choosen_result.clone());

    let mut game = Game::new(10, 10, 10, &random_chooser);
    assert!(!game.is_started());

    // Opening all the no-bomb cells
    let mut no_bomb_cells: Vec<_> = (0..97).filter(|i| !choosen_result.contains(i)).collect();

    let last_cell = no_bomb_cells.pop().expect("loooool");

    for i in no_bomb_cells {
        println!("{i}");
        let coordinates = game.field.get_cell_coordinates_from_index(i as u16);
        let open_result = game.open_cell(&coordinates);
        assert!(!open_result);
        assert!(game.is_started());
    }

    // Opening the last cell
    let coordinates = game.field.get_cell_coordinates_from_index(last_cell as u16);
    let open_result = game.open_cell(&coordinates);
    assert!(!open_result);
    assert!(game.is_ended());
    assert!(matches!(game.get_result(), GameResult::Victory))
}

#[test]
fn test_lose_game() {
    /*
    Consider the field to be like this:

       1 1 1 0 0 0 0 0 0 0
       1 * 1 0 0 0 0 0 0 0
       2 2 2 0 0 0 0 0 0 0
       1 * 1 0 0 0 0 0 0 0
       2 2 2 0 0 1 1 1 0 0
       2 * 1 0 0 1 * 2 1 0
       * 2 1 1 1 2 2 * 1 0
       1 1 0 2 * 3 2 2 1 0
       0 0 0 2 * 3 * 1 1 1
       0 0 0 1 1 2 1 1 1 *

    */

    // So bombs should be the next indexes:
    let choosen_result = vec![11, 17, 31, 51, 56, 60, 67, 74, 84, 86];
    let random_chooser = NotRandomChooser::new(choosen_result.clone());

    let mut game = Game::new(10, 10, 10, &random_chooser);
    assert!(!game.is_started());

    // Open first cell and stard the game
    let coordinates = Coordinates { column: 3, row: 2 };
    game.open_cell(&coordinates);

    // Opening the last cell
    let coordinates = Coordinates { column: 2, row: 4 };
    let open_result = game.open_cell(&coordinates);
    assert!(open_result);
    assert!(game.is_ended());
    assert!(matches!(game.get_result(), GameResult::Defeat))
}

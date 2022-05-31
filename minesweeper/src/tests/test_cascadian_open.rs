use crate::{
    cell::Coordinates, game::Game, tests::factories::not_random_chooser::NotRandomChooser,
    view::TerminalViewer,
};

#[test]
fn test_cascadian_open() {
    // Consider the field to be like this:
    let expected_field = "\
        █ █ 1 0 0 0 1 █ 1 0\n\
        █ █ 1 0 0 0 1 █ 1 0\n\
        █ █ 2 0 0 0 1 1 1 0\n\
        █ █ 1 0 0 0 0 0 0 0\n\
        █ █ 2 0 0 1 1 1 0 0\n\
        █ █ 1 0 0 1 █ 2 1 0\n\
        █ █ 1 1 1 2 █ █ 1 0\n\
        █ █ █ █ █ █ █ 2 1 0\n\
        █ █ █ █ █ █ █ 1 0 0\n\
        █ █ █ █ █ █ █ 1 0 0\
    ";

    // So bombs should be the next indexes:
    let choosen_result = vec![11, 17, 31, 51, 56, 60, 67, 74, 84, 86];
    let random_chooser = NotRandomChooser::new(choosen_result);

    let mut game = Game::new(10, 10, 10, &random_chooser);
    assert!(!game.is_started());

    let coordinates = Coordinates { column: 5, row: 2 };
    let first_open_result = game.open_cell(&coordinates);
    assert!(!first_open_result);
    assert!(game.is_started());

    let view = TerminalViewer::view_only_opened(&game.field);
    println!("{view}");
    assert_eq!(view, expected_field);
}

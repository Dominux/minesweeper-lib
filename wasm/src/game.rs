use yew::prelude::*;

use crate::cell_data::CellData;
use crate::field;

#[function_component(Game)]
pub fn game_component() -> Html {
    let game = minesweeper_lib::Minesweeper::new(10, 10, 10);

    let field: Vec<Vec<_>> = game
        .view()
        .iter()
        .map(|r| r.iter().map(|c| CellData::new(c)).collect())
        .collect();

    html! {
        <field::Field cells={field}/>
    }
}

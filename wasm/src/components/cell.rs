use yew::classes;
use yew::prelude::*;

use crate::models;

const DEFAULT_CLASSES: &str = "cell cell-inner";

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub cell: models::CellData,
    pub on_open: Callback<models::Coordinates>,
}

#[function_component(Cell)]
pub fn cell_component(props: &Props) -> Html {
    let Props { cell, on_open } = props.clone();

    let onclick = move |e: MouseEvent| {
        e.stop_propagation();

        if !cell.is_opened {
            on_open.emit(cell.coordinates)
        }
    };

    html! {
        <span class="cell">
            <div
                class={classes!({
                    let class = if cell.is_opened {"opened"} else {"closed"};
                    format!("{} {}", DEFAULT_CLASSES, class)
                })}
                {onclick}
            >
                {get_cell_content(&cell)}
            </div>
        </span>
    }
}

fn get_cell_content(cell: &models::CellData) -> String {
    if cell._type.is_none() {
        return "".to_string();
    }

    let cell_type = cell._type.unwrap();

    match cell_type {
        minesweeper_lib::CellType::Bomb => "💥".to_string(),
        minesweeper_lib::CellType::Empty(i) => match i {
            0 => "".to_string(),
            _ => i.to_string(),
        },
    }
}

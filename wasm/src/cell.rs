use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_opened: bool,
    pub _type: Option<minesweeper_lib::CellType>,
}

#[function_component(Cell)]
pub fn cell_component(props: &Props) -> Html {
    html! {
        <span class="cell">
            { if props.is_opened {
                html! {
                    <div class="cell cell-inner">
                        // { &props.is_opened }
                    </div>
                }
            } else {
                html! {
                    <div class="cell cell-inner closed">
                        // { &props.is_opened }
                    </div>
                }
            }}
        </span>
    }
}

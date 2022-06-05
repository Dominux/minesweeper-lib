use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_opened: bool,
}

#[function_component(Cell)]
pub fn cell_component(props: &Props) -> Html {
    html! {
        <span class="cell">
            { &props.is_opened }
        </span>
    }
}

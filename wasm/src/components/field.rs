use yew::prelude::*;

use super::cell::Cell;
use crate::models;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub cells: Vec<Vec<models::CellData>>,
    pub on_open: Callback<models::Coordinates>,
}

#[function_component(Field)]
pub fn field_component(props: &Props) -> Html {
    html! {
        <>
            <table class="board">
                { for props.cells.iter().map(|row|
                    html! {
                        <tr>
                            { for row.iter().map(|cell|
                                html!(
                                    <td>
                                        <Cell cell={*cell} on_open={&props.on_open}/>
                                    </td>
                                )
                            )}
                        </tr>
                    }
                ) }
            </table>

            <div class="endscreen"></div>
        </>
    }
}

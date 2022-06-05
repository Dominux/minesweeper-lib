use yew::prelude::*;

use crate::cell::Cell;
use crate::cell_data::CellData;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub cells: Vec<Vec<CellData>>,
}

#[function_component(Field)]
pub fn field_component(props: &Props) -> Html {
    html! {
        <div>
            <table class="board">
                { for props.cells.iter().map(|row|

                    html! {
                        <tr>
                            { for row.iter().map(|cell|

                                html!(
                                    <td>
                                        <Cell is_opened={cell.is_opened} _type={cell._type.clone()}/>
                                    </td>
                                )

                            )}
                        </tr>
                    }

                ) }
            </table>

            <div class="endscreen"></div>
        </div>
    }
}

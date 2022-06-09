use std::str::FromStr;

use yew::prelude::*;

use crate::models;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub on_start: Callback<models::SettingsModel>,
}

#[function_component(Settings)]
pub fn settings_component(props: &Props) -> Html {
    let Props { on_start } = props.clone();

    let height = use_state(|| 10_u16);
    let width = use_state(|| 10_u16);
    let bombs = use_state(|| 10);
    let is_valid = use_state(|| true);

    ////////////////////////////////////////
    //  On input
    ////////////////////////////////////////
    let oninput_height = {
        let height = height.clone();
        Callback::from(move |e: Event| {
            if let Some(n) = parse_input_event(e) {
                height.set(n)
            }
        })
    };
    let oninput_width = {
        let width = width.clone();
        Callback::from(move |e: Event| {
            if let Some(n) = parse_input_event(e) {
                width.set(n)
            }
        })
    };
    let oninput_bombs = {
        let bombs = bombs.clone();
        Callback::from(move |e: Event| {
            if let Some(n) = parse_input_event(e) {
                bombs.set(n)
            }
        })
    };

    ////////////////////////////////////////
    //  On key up
    ////////////////////////////////////////
    let onkeyup_height = {
        let height = height.clone();
        Callback::from(move |e: KeyboardEvent| {
            if let Some(n) = parse_keyup_event(e) {
                height.set(n)
            }
        })
    };
    let onkeyup_width = {
        let width = width.clone();
        Callback::from(move |e: KeyboardEvent| {
            if let Some(n) = parse_keyup_event(e) {
                width.set(n)
            }
        })
    };
    let onkeyup_bombs = {
        let bombs = bombs.clone();
        Callback::from(move |e: KeyboardEvent| {
            if let Some(n) = parse_keyup_event(e) {
                bombs.set(n)
            }
        })
    };

    // On start
    let onclick = {
        let (height, width, bombs) = (height.clone(), width.clone(), bombs.clone());
        move |e: MouseEvent| {
            e.stop_propagation();

            let settings = models::SettingsModel::new(*height, *width, *bombs);
            on_start.emit(settings)
        }
    };

    // Validating bombs amount
    {
        let is_valid = is_valid.clone();
        use_effect_with_deps(
            move |(height, width, bombs)| {
                let new_is_valid =
                    ((*height.clone() as usize) * (*width.clone() as usize)) > *bombs.clone();
                is_valid.set(new_is_valid);
                || ()
            },
            (height.clone(), width.clone(), bombs.clone()),
        );
    }

    // TODO: add styles
    html!(
        <form action="javascript:void(0);">
            <div>
                <pre>{"Height"}</pre>
                <input
                    value={height.to_string()}
                    id="height"
                    type="number"
                    min="1"
                    max={u8::MAX.to_string()}
                    onchange={oninput_height}
                    onkeyup={onkeyup_height}
                />
            </div>
            <div>
                <pre>{"Width"}</pre>
                <input
                    value={width.to_string()}
                    id="width"
                    type="number"
                    min="1"
                    max={u8::MAX.to_string()}
                    onchange={oninput_width}
                    onkeyup={onkeyup_width}
                />
            </div>
            <div>
                <pre>{"Bombs"}</pre>
                <input
                    value={bombs.to_string()}
                    id="bombs"
                    type="number"
                    min="1"
                    max={usize::MAX.to_string()}
                    onchange={oninput_bombs}
                    onkeyup={onkeyup_bombs}
                />
            </div>

            <button disabled={!*is_valid.clone()} {onclick}>{"Start"}</button>

            if !*is_valid.clone() {
                <pre class="error">{"Bombs cannot be more than height * width"}</pre>
            }
        </form>
    )
}

fn parse_input_event<T>(e: Event) -> Option<T>
where
    T: FromStr,
{
    e.target_unchecked_into::<web_sys::HtmlInputElement>()
        .value()
        .parse()
        .ok()
}

fn parse_keyup_event<T>(e: KeyboardEvent) -> Option<T>
where
    T: FromStr,
{
    e.target_unchecked_into::<web_sys::HtmlInputElement>()
        .value()
        .parse()
        .ok()
}

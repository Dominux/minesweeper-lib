use yew::prelude::*;

use crate::models;

use super::game::Game;
use super::settings::Settings;

enum AppState {
    InSettings,
    InGame,
}

#[function_component(App)]
pub fn app_component() -> Html {
    let state = use_state(|| AppState::InSettings);
    let height: UseStateHandle<Option<u16>> = use_state(|| None);
    let width: UseStateHandle<Option<u16>> = use_state(|| None);
    let bombs: UseStateHandle<Option<usize>> = use_state(|| None);

    let on_start = {
        let (state, height, width, bombs) =
            (state.clone(), height.clone(), width.clone(), bombs.clone());
        Callback::from(move |settings: models::SettingsModel| {
            height.set(Some(settings.height));
            width.set(Some(settings.width));
            bombs.set(Some(settings.bombs));
            state.set(AppState::InGame)
        })
    };

    match *state.clone() {
        AppState::InSettings => html!(
            <Settings {on_start} />
        ),
        AppState::InGame => html!(
            <Game height={height.clone().unwrap()} width={width.clone().unwrap()} bombs={bombs.clone().unwrap()} />
        ),
    }
}

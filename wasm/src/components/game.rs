use yew::prelude::*;

use super::field;
use crate::models::{CellData, Coordinates};

pub struct Game {
    game: minesweeper_lib::Minesweeper,
    is_ended: bool,
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub height: u16,
    pub width: u16,
    pub bombs: usize,
}

pub enum Msg {
    OpenCell(Coordinates),
}

impl Component for Game {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let Props {
            height,
            width,
            bombs,
        } = &ctx.props();
        let game = minesweeper_lib::Minesweeper::new(*height, *width, *bombs);
        Self {
            game,
            is_ended: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OpenCell(c) => {
                let result = self.game.open_cell(c.column, c.row);

                // Here we guaranteed that all possible errors are avoided
                if result.unwrap().is_some() {
                    self.is_ended = true
                }

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_open = ctx.link().callback(|c: Coordinates| Msg::OpenCell(c));
        html! {
            <field::Field cells={self.get_field()} {on_open}/>
        }
    }
}

impl Game {
    fn get_field(&self) -> Vec<Vec<CellData>> {
        self.game
            .view(true)
            .iter()
            .enumerate()
            .map(|(ir, r)| {
                r.iter()
                    .enumerate()
                    .map(|(ic, c)| CellData::new(c, Coordinates::new(ir + 1, ic + 1)))
                    .collect()
            })
            .collect()
    }
}

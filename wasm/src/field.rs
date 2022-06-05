use yew::prelude::*;

pub struct Field;

pub enum Msg {
    OpenCell(),
}

enum Coordinates {}

impl Component for Field {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!(
            <h1>{ "Sup nibba" }</h1>
        )
    }
}

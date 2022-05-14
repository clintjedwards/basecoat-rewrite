use yew::prelude::*;

pub struct Login {}

impl Component for Login {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <h1 class="text-8xl">{"Yew works!"}</h1>
        }
    }
}

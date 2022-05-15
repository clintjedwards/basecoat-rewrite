use yew::prelude::*;

pub struct Login {}

impl Component for Login {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1 class="text-8xl">{"Basecoat"}</h1>

                <label for="organization">{"Organization:"}</label>
                <input type="text" id="organization" placeholder="Organization Name" autocomplete="off" />

                <label for="username">{"Username:"}</label>
                <input type="text" id="username" placeholder="Username" autocomplete="off" />

                <label for="password">{"Password:"}</label>
                <input type="text" id="password" placeholder="Password" autocomplete="off" />
            </div>
        }
    }
}

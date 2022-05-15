use yew::prelude::*;
use yew_router::prelude::*;

mod components;
use crate::components::Footer;
use crate::components::Login;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/login")]
    Login,
    #[at("/")]
    Formulas,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Login => html! { <Login /> },
        Route::Formulas => html! { <h1>{"Formulas"}</h1>},
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

struct App {}

impl Component for App {
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
                <BrowserRouter>
                    <Switch<Route> render={Switch::render(switch)} />
                </BrowserRouter>
                <Footer></Footer>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}

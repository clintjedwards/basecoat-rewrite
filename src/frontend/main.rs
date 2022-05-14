use yew::prelude::*;
use yew_router::prelude::*;

mod components;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn main() {
    yew::start_app::<components::Login>();
}

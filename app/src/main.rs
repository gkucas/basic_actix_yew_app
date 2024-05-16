use yew::prelude::*;
use yew_bootstrap::util::include_cdn;
use yew_router::prelude::*;

use crate::router::{Route, switch};

mod components;
mod pages;
mod services;
mod router;

#[function_component(Main)]
fn app() -> Html {
    html! {
        <>
            {include_cdn()}
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </>
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}
use yew::{Html, html};
use yew_router::Routable;

use crate::components::logout_button::LogoutButton;
use crate::pages::home_page::HomePage;
use crate::pages::login_page::LoginPage;
use crate::pages::register_page::RegisterPage;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    html! {
        <div class="container">
            {
            match routes {
                    Route::Home => html! {
                        <div>
                            <LogoutButton />
                            <HomePage />
                        </div>
                    },
                    Route::Login => html! {
                        <LoginPage />
                    },
                    Route::Register => html! {
                        <RegisterPage />
                    },
                    Route::NotFound => html! { <h1>{ "404" }</h1> },
                }
            }
        </div>
    }
}
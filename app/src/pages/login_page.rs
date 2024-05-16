use std::ops::Deref;

use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::form_input::FormInput;
use crate::components::submit_button::SubmitButton;
use crate::router;
use crate::services::user_service::login_user;

#[derive(Debug, Default, Clone)]
struct LoginUserState {
    email: String,
    password: String,
    error: String,
}

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let state = use_state(|| LoginUserState::default());
    let navigator = use_navigator().unwrap();

    let cloned_state = state.clone();
    let email_changed = Callback::from(move |value: String| {
        let mut data = cloned_state.deref().clone();
        data.email = value;
        cloned_state.set(data);
    });
    let cloned_state = state.clone();
    let password_changed = Callback::from(move |value| {
        let mut data = cloned_state.deref().clone();
        data.password = value;
        cloned_state.set(data);
    });

    let cloned_state = state.clone();
    let cloned_navigator = navigator.clone();
    let submit = Callback::from(move |event: SubmitEvent| {
        let data = cloned_state.deref().clone();
        let navigator = cloned_navigator.clone();
        event.prevent_default();

        let cloned_state = cloned_state.clone();
        spawn_local(async move {
            let res = login_user(&data.email, &data.password).await;
            match res {
                Ok(_user) => {
                    navigator.push(&router::Route::Home);
                }
                Err(e) => {
                    let mut data = cloned_state.deref().clone();
                    data.error = e;
                    cloned_state.set(data);
                }
            }
        })
    });
    html! {
        <div class="container">
            <form onsubmit={submit}>
                <FormInput name="email" input_type="email" label="Email" handle_onchange={email_changed}/>
                <FormInput name="password" input_type="password" label="Password" handle_onchange={password_changed}/>
                <SubmitButton label="Login"></SubmitButton>
            </form>
            <div>
                <span style="color:red;">{&state.error}</span>
            </div>
            <div>
                <a href="/register">{"Need an account?"}</a>
            </div>
        </div>
    }
}
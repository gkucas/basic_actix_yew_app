use std::ops::Deref;
use wasm_bindgen_futures::spawn_local;

use yew::prelude::*;
use yew_router::hooks::use_navigator;
use crate::components::form_input::FormInput;
use crate::components::submit_button::SubmitButton;
use crate::router;
use crate::services::user_service::register_user;

#[derive(Debug, Default, Clone)]
struct RegisterUserState {
    name: String,
    email: String,
    password: String,
    error: String,
}

#[function_component(RegisterPage)]
pub fn register_page() -> Html {
    let state = use_state(|| RegisterUserState::default());
    let navigator = use_navigator().unwrap();

    let cloned_state = state.clone();
    let name_changed = Callback::from(move |value| {
        let mut data = cloned_state.deref().clone();
        data.name = value;
        cloned_state.set(data);
    });
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
            let res = register_user(&data.name, &data.email, &data.password).await;
            match res{
                Ok(_) => {
                    navigator.push(&router::Route::Login);
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
                <FormInput name="name" input_type="text" label="Name" handle_onchange={name_changed}/>
                <FormInput name="email" input_type="email" label="Email" handle_onchange={email_changed}/>
                <FormInput name="password" input_type="password" label="Password" handle_onchange={password_changed}/>
                <SubmitButton label="Register"></SubmitButton>
            </form>
            <div>
                <a href="/login">{"Already have an account?"}</a>
            </div>
            <div>
                <span style="color:red;">{&state.error}</span>
            </div>
        </div>
    }
}
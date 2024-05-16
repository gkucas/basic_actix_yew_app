use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::hooks::use_navigator;
use crate::router;

use crate::services::user_service::logout;

#[function_component(LogoutButton)]
pub fn logout_button() -> Html {
    let navigator = use_navigator().unwrap();
    let navigator_clone = navigator.clone();
    let onclick = Callback::from(move |_event: MouseEvent| {
        spawn_local({
            let navigator = navigator_clone.clone();
            async move {
                match logout().await {
                    Ok(_) => {
                        navigator.push(&router::Route::Login);
                    }
                    Err(_e) => {}
                }
            }
        });
    });
    html! {
        <button
            type="submit"
            class="btn btn-primary"
            style="float:right"
            onclick={onclick}
        >
            {"Logout"}
        </button>
    }
}
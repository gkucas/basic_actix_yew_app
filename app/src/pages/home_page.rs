use std::ops::Deref;

use wasm_bindgen_futures::spawn_local;
use yew::{function_component, Html, html, use_state};

use crate::services::user_service::load_users;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let users = use_state(|| Vec::default());
    let error = use_state(|| None);
    let loaded = use_state(|| false);
    if !loaded.deref() {
        spawn_local({
            let cloned_users = users.clone();
            let cloned_error = error.clone();
            let cloned_loaded = loaded.clone();
            async move {
                match load_users().await {
                    Ok(users) => {
                        cloned_loaded.set(true);
                        cloned_users.set(users);
                    }
                    Err(e) => {
                        cloned_loaded.set(true);
                        cloned_error.set(Some(e));
                    }
                }
            }
        });
    }

    html! {
        <div>
            <table class="table">
              <thead>
                <tr>
                  <th scope="col">{"Name"}</th>
                  <th scope="col">{"Email"}</th>
                  <th scope="col">{"Password"}</th>
                </tr>
              </thead>
              <tbody>
                { users.iter()
                    .map(|user| {
                        html!{
                        <tr>
                          <td>{&user.name}</td>
                          <td>{&user.email}</td>
                          <td>{&user.password}</td>
                        </tr>
                        }
                    })
                    .collect::<Html>()
                }
              </tbody>
            </table>
            <div>
                {
                    match error.deref() {
                        None => {html!{}}
                        Some(e) => {html!{
                            <span style="color:red;">{&e}</span>
                        }}
                    }
                }
            </div>
        </div>
    }
}
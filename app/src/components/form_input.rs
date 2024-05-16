use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub input_type: String,
    pub label: String,
    pub name: String,
    pub handle_onchange: Callback<String>,
}

#[function_component(FormInput)]
pub fn form_input_component(props: &Props) -> Html {
    let callback = props.handle_onchange.clone();
    let onchange = Callback::from(move |event: Event| {
        let target = event.target().unwrap();
        let input = target.unchecked_into::<HtmlInputElement>();
        callback.emit(input.value());
    });
    html! {
        <div class="mb-3">
            <label for={props.name.clone()} class="form-label">{&props.label.clone()}</label>
            <input type={props.input_type.clone()} class="form-control" id={props.name.clone()} onchange={onchange}/>
        </div>
    }
}
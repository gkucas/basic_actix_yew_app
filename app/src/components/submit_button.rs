use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub label: String,
}

#[function_component(SubmitButton)]
pub fn submit_button(props: &Props) -> Html {
    html! {
    <button
        type="submit"
        class="btn btn-primary"
    >
        {props.label.clone()}
    </button>
    }
}
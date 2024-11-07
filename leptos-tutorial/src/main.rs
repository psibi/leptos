use ev::{MouseEvent, SubmitEvent};
use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! { <App/> }
    })
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <SomeInput />
    }
}

#[component]
fn SomeInput() -> impl IntoView {
    let (name, set_name) = create_signal("Uncontrolled input".to_owned());
    let input_element: NodeRef<html::Input> = create_node_ref();
    let on_submit = move |ev: SubmitEvent| {
        // To stop page from reloading
        ev.prevent_default();
        let value = input_element().expect("<input> should be mounted").value();
        set_name(value);
    };
    view! {
        <form on:submit=on_submit>
        <input type="text" value=name node_ref=input_element />
        <input type="submit" value="Submit" />
        </form>
        <p>"Name is: " {name}</p>
    }
}

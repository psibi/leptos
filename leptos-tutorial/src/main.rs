use ev::{MouseEvent, SubmitEvent};
use leptos::*;
use std::thread;
use std::time::Duration;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! { <App/> }
    })
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <SomeComponent />
    }
}

async fn some_async_function(input: &String) -> String {
    match &input[..] {
        "hi" => "hello".to_owned(),
        "bye" => "Bye bye".to_owned(),
        _ => "Hmm..".to_owned(),
    }
}

#[component]
fn SomeComponent() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let action = create_action(|input: &String| {
        let input = input.to_owned();
        async move { some_async_function(&input).await }
    });

    let click_handler = move |_| {
        set_count.update(|count| *count += 1);
        let count = count();
        if count > 10 {
            action.dispatch("bye".to_owned());
        } else if count > 3 {
            action.dispatch("hi".to_owned());
        }
    };

    let value = action.value();

    view! {
        <button on:click=click_handler>"Click me: " {move || count()} </button>
        <p>"Data: " {move || format!("{:#?}", value()) }</p>
    }
}

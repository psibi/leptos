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

#[component]
fn SomeComponent() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let (count2, set_count2) = create_signal(0);

    let async_data_one = create_resource(count, |value| async move {
        "Some data from API (1)".to_owned()
    });

    let async_data_two = create_resource(count2, |value| async move {
        logging::log!("loading data from API (2)");
        "Some data from API (2)".to_owned()
    });

    let click_handler = move |_| set_count.update(|count| *count += 1);

    view! {
        <button on:click=click_handler>"Click me: " {move || count()} </button>
        <Suspense fallback=move || view! {<p>"Loading..."</p>} >
        <h2> "My data:" </h2>
        <div> "Count 1:" {async_data_one.get().map(|a| view! {<p>{a}</p>})} </div>
        <div> "Count 2:" {async_data_two.get().map(|a| view! {<p>{a}</p>})} </div>
        </Suspense>
    }
}

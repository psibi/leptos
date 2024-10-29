use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! { <App/> }
    })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count() * 2;

    view! {
        <button
        class:red=move || count() % 2 == 1
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >
            "Click me: "
            {move || count()}
        </button>

        <ProgressBar max=50 progress=count/>
        <ProgressBar max=50 progress=double_count/>
    }
}

#[component]
fn ProgressBar(#[prop(default = 100)] max: u16, progress: ReadSignal<i32>) -> impl IntoView {
    view! {
            <progress max=max value=progress/>
    }
}

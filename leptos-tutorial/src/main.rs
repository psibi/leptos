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
        <LoggedIn fallback=move || view!{<p>"Not logged in"</p>}>
        <p>"I'm logged in"</p>
        </LoggedIn>
    }
}

#[component]
pub fn LoggedIn<F, IV>(fallback: F, children: ChildrenFn) -> impl IntoView
where
    F: Fn() -> IV + 'static,
    IV: IntoView,
{
    view! {
        <Suspense
            fallback=|| ()
        >
            <Show
                // check whether user is verified
                // by reading from the resource
                when=move || todo!()
                fallback=fallback
            >
                {children()}
            </Show>
        </Suspense>
    }
}

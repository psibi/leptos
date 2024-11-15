use ev::{MouseEvent, SubmitEvent};
use leptos::*;
use leptos_router::*;
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
        <Router>
        <Routes>
        <Route path="/" view=Home/>
        <Route path="/users" view=Users/>
        <Route path="/me" view=move || view! {<h1>"Sibi"</h1>}/>
        </Routes>
        </Router>
    }
}

#[component]
fn Users() -> impl IntoView {
    view! {
        <div>
        <h1>"Users" </h1>
        <A href="/me">"Navigate to me!"</A>
        </div>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <h1>"Home page"</h1>
    }
}

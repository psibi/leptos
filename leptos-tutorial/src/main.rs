use ev::MouseEvent;
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
        <ProgressBar/>
        <ProgressBar progress=Box::new(|| 5)/>
    }
}

#[component]
fn ProgressBar(
    #[prop(optional)] progress: Option<Box<dyn Fn() -> i32>>,
) -> impl IntoView {
    progress.map(|progress| {
        view! {
            <progress
                max=100
                value=progress
            />
            <br/>
        }
    })
}

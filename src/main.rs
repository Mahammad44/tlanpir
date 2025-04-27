use leptos::*;

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1>"Hello from Tlanpir!"</h1>
    }
}

fn main() {
    mount_to_body(|cx| view! { cx,  <App /> });
}


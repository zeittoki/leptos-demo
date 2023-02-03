use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use super::components::{counter::*, about::*};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
  provide_meta_context(cx);

  view! {
    cx,
    <Title text="Welcome to Leptos"/>

    <Router>
        <main>
            <Routes>
                <Route path="" view=move |cx| view! { cx, <Counter/> }/>
                <Route path="/about" view=move |cx| view! { cx, <About/> }/>
            </Routes>
        </main>
    </Router>
  }
}
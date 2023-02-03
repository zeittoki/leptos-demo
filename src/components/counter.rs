use leptos::*;

#[component]
pub fn Counter(cx: Scope) -> impl IntoView {
  let (count, set_count) = create_signal(cx, 0);
  let on_click = move |_| set_count.update(|count| *count += 1);

  view! { cx,
      <h1>"Counter"</h1>
      <button on:click=on_click>"Click Me: " {count}</button>
  }
}
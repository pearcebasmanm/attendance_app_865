use leptos::*;

#[component]
pub fn Counter(cx: Scope) -> Element {
    let (counter, set_counter) = create_signal(cx, 0);
    view! {
        cx,
        <div>
        <h2>"Counter is at: " {counter}</h2>
        <button on:click=move |_| set_counter.update(|val| *val -= 1)>"-1"</button>
        <button on:click=move |_| set_counter.update(|val| *val += 1)>"+1"</button>
        </div>
    }
}
use leptos::*;

fn main() {
    leptos::mount_to_body(attendance_app)
}


pub fn attendance_app(cx: Scope) -> Element {
    let (counter, set_counter) = create_signal(cx, 0);
    view! {
        cx,
        <div>
        <h1>"Attendance App"</h1>
        <h2>"Counter is at: " {counter}</h2>
        <button on:click=move |_| set_counter.update(|val| *val -= 1)>"-1"</button>
        <button on:click=move |_| set_counter.update(|val| *val += 1)>"+1"</button>
        </div>
    }
}
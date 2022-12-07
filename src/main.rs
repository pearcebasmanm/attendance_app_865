use leptos::*;
mod gsheets;
mod counter;
use counter::{Counter, CounterProps};
mod name_input;
use name_input::{NameInput, NameInputProps};

fn main() {
    leptos::mount_to_body(attendance_app)
}

pub fn attendance_app(cx: Scope) -> Element {
    view! {
        cx,
        <div>
        <h1>"Attendance App"</h1>
        <Counter/>
        <NameInput/>
        </div>
    }
}
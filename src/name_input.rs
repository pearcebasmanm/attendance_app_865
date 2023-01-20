use crate::gsheets::get_names;
use leptos::{web_sys::KeyboardEvent, *};

const TAB: u32 = 9;
const ENTER: u32 = 13;

#[component]
pub fn NameInput(cx: Scope) -> Element {
    let (name, set_name) = create_signal(cx, String::new());
    let (hint, set_hint) = create_signal(cx, String::new());

    let names = get_names();

    let on_keydown = move |ev: KeyboardEvent| {
        let input = event_target_value(&ev);
        let hint = if input.trim().is_empty() {
            String::new()
        } else {
            match names.iter().find(|name| name.contains(&input)) {
                Some(val) => val.clone(),
                None => String::new(),
            }
        };

        match ev.key_code() {
            TAB => {
                set_name(hint);
                set_hint("Accepted Suggestion".to_string());
            }
            ENTER => {
                set_name(input);
                set_hint("Accepted Input".to_string())
            }
            _ => set_hint(hint),
        }
    };

    view! {
        cx,
        <div>
        <p>{name}</p>
        <input
            placeholder="Input Name Here"
            on:keydown=on_keydown/>
        <p>{hint}</p>
        </div>
    }
}

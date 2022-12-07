use leptos::*;
use crate::gsheets;

const TAB: u32 = 9;

#[component]
pub fn NameInput(cx: Scope) -> Element {
    let (name, set_name) = create_signal(cx, String::new());
    let (suggestion, set_suggestion) = create_signal(cx, String::new());

    let names = gsheets::get_names();

    view!{
        cx,
        <div>
        <p>{name}</p>
        <input
            placeholder="Input Name Here"
            on:keydown=move |ev| {
                let written = event_target_value(&ev);
                let suggested = names
                    .iter()
                    .find(|name| name.contains(&written));
                
                if let Some(suggested) = suggested {     
                    if ev.key_code() == TAB {
                        set_name(suggested.to_string());
                        set_suggestion("Accepted Suggestion".to_string());
                    } else {
                        set_suggestion(suggested.to_string());
                    }
                }
            }
        />
        <p>{suggestion}</p>
        </div>
    }
}
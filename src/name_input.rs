use leptos::*;

const TAB: u32 = 9;

#[component]
pub fn NameInput(cx: Scope) -> Element {
    let (name, set_name) = create_signal(cx, String::new());
    let (suggestion, set_suggestion) = create_signal(cx, String::new());

    let names = vec![
        "Max Pearce Basman",
        "Scott William Pearce"
    ];

    view!{
        cx,
        <div>
        <p>{name}</p>
        <input
            placeholder="Input Name Here"
            on:change=move |ev| set_name(event_target_value(&ev))
            on:keydown=move |ev| {
                let suggest = names
                    .iter()
                    .find(|name| name.contains(&event_target_value(&ev)));
                
                if let Some(name) = suggest {
                    set_name(name.to_string());
                    set_suggestion(name.to_string());
                    if ev.key_code() == TAB {
                        set_suggestion("Accepted Suggestion".to_string());
                    }
                }
            }
        />
        <p>{suggestion}</p>
        </div>
    }
}
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();

    let (regex_text, set_regex_text) = signal("".to_string());
    let (regex_pat, set_regex_pat) = signal("".to_string());

    leptos::mount::mount_to_body(move || {
        view! {
            <header>
                <h1>Interactive Rust Regex Tester</h1>
            <div>"by "<a href="https://theokrueger.dev">theokrueger</a></div>
            </header>
            <hr />

            <label for="regex_text">Input</label>
            <br />
            <textarea
                autofocus
                placeholder="Your data"
                 required
                id="regex_text"
                on:input:target=move |ev| {
                    set_regex_text.set(ev.target().value());
                }
                prop:value=regex_text
            />
            <label for="regex_pat">Pattern</label>
            <br />
            <input
                id="regex_pat"
                type="text"
                on:input:target=move |ev| {
                    set_regex_pat.set(ev.target().value());
                }
                prop:value=regex_pat
            />
            <label for="regex_match">Output</label>
            <br />
            <textarea id="regex_match" readonly></textarea>

            <br />
            <label for="regex_ver">Regex</label>
            <select name="regex_ver" id="regex_ver">
                <option value="1.13">1.13</option>
            </select>

            <hr />

            <p>
            <a href="https://gitlab.com/theokrueger/rrx">source</a>,
            " made with "
            <a href="https://www.leptos.dev/">Leptos</a>
            " " and " "
            <a href="https://crates.io/crates/regex">regex</a>
            ". clientsided, no tracking."
            </p>
        }
    })
}

use leptos::prelude::*;
use std::{
    collections::HashMap,
    fmt::{Display, Error, Formatter},
    rc::Rc,
};

enum RegexVer {
    V1_5,
    V1_9,
    V1_11,
    V1_13,
}
use RegexVer as RV;

impl Display for RegexVer {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{}",
            match self {
                RV::V1_5 => "1.5",
                RV::V1_9 => "1.9",
                RV::V1_11 => "1.11",
                RV::V1_13 => "1.13",
            }
        )
    }
}

macro_rules! find_all {
    ($re:ident, $pat:expr, $txt:expr) => {
        match $re::Regex::new($pat) {
            Ok(p) => format!(
                "{}",
                p.find_iter($txt)
                    .enumerate()
                    .filter_map(|(i, m)| if m.as_str().len() > 0 {
                        Some(format!("{i}: {}\n", m.as_str()))
                    } else {
                        None
                    })
                    .collect::<String>()
            ),
            Err(e) => format!("{e:?}"),
        }
    };
}

fn main() {
    // hack
    let s2rv = HashMap::from([
        ("1.5", RV::V1_5),
        ("1.9", RV::V1_9),
        ("1.11", RV::V1_11),
        ("1.13", RV::V1_13),
    ]);
    console_error_panic_hook::set_once();

    let (regex_text, set_regex_text) = signal("".to_string());
    let (regex_pat, set_regex_pat) = signal("".to_string());
    let (regex_out, set_regex_out) = signal("".to_string());
    let (regex_ver, set_regex_ver) = signal(RegexVer::V1_13.to_string());
    leptos::mount::mount_to_body(move || {
        let rv = regex_ver;
        let upd_regex1 = Rc::new(move || {
            let binding = regex_text.get();
            let txt = binding.as_str();
            let pat = regex_pat.get();
            let pat = pat.as_str();
            set_regex_out.set(match s2rv.get(&rv.get().as_str()).unwrap() {
                RV::V1_5 => find_all!(regex_1_5, pat, txt),
                RV::V1_9 => find_all!(regex_1_9, pat, txt),
                RV::V1_11 => find_all!(regex_1_11, pat, txt),
                RV::V1_13 => find_all!(regex_1_13, pat, txt),
            });
        });
        let upd_regex2 = upd_regex1.clone();
        let upd_regex3 = upd_regex1.clone();

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
                    upd_regex1();
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
                    upd_regex2();
                }
                prop:value=regex_pat
            />
            <label for="regex_out">Output</label>
            <br />
            <textarea
            id="regex_out"
            prop:value=regex_out
            readonly></textarea>

            <br />
            <label for="regex_ver">Regex</label>
            <select
            name="regex_ver"
            id="regex_ver"
            on:input:target=move |ev| {
                    set_regex_ver.set(ev.target().value());
                    upd_regex3();
                }
            prop:value=regex_ver
            >
            <option value="1.5">1.5</option>
            <option value="1.9">1.9</option>
            <option value="1.11">1.11</option>
            <option value="1.13">1.13</option>
            </select>" - "<a href=move || format!("https://docs.rs/regex/{}/regex#syntax", regex_ver.get())>syntax reference</a>

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

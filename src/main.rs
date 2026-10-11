use leptos::prelude::*;
use std::rc::Rc;

#[derive(Clone)]
struct Hit {
    text: String,
    start: usize,
    end: usize,
}

#[derive(Clone)]
enum Out {
    Idle,
    Error(String),
    NoMatch,
    Hits(Vec<Hit>),
}

macro_rules! find_all {
    ($re:ident, $pat:expr, $txt:expr) => {
        match $re::Regex::new($pat) {
            Ok(p) => {
                let hits: Vec<Hit> = p
                    .find_iter($txt)
                    .filter(|m| !m.as_str().is_empty())
                    .map(|m| Hit {
                        text: m.as_str().to_string(),
                        start: m.start(),
                        end: m.end(),
                    })
                    .collect();
                if hits.is_empty() {
                    Out::NoMatch
                } else {
                    Out::Hits(hits)
                }
            }
            Err(e) => Out::Error(format!("{e}")),
        }
    };
}

fn out_view(out: Out) -> AnyView {
    match out {
        Out::Idle => view! { <span class="dim">"no pattern/text"</span> }.into_any(),
        Out::Error(msg) => view! { <span class="red">{msg}</span> }.into_any(),
        Out::NoMatch => view! { <span class="dim">"no matches"</span> }.into_any(),
        Out::Hits(hits) => {
            let total = hits.len();
            let iw = total.to_string().len();
            // ranges are padded so the match column lines up
            let ranges: Vec<String> = hits
                .iter()
                .map(|h| format!("[{}..{}]", h.start, h.end))
                .collect();
            let rw = ranges.iter().map(|r| r.len()).max().unwrap_or(0);
            let rows: Vec<AnyView> = hits
                .iter()
                .zip(ranges.iter())
                .enumerate()
                .map(|(i, (h, range))| {
                    let idx = format!("{:>iw$}", i + 1);
                    let range = format!("{range:<rw$}");
                    view! {
                        <span class="dim">{idx}</span>
                        "  "
                        <span class="cyan">{range}</span>
                        "  "
                        <span>{h.text.clone()}</span>
                        "\n"
                    }
                    .into_any()
                })
                .collect();
            let header = format!("{total} match{}", if total == 1 { "" } else { "es" });
            view! {
                <span class="dim">{header}</span>
                "\n\n"
                {rows}
            }
            .into_any()
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();

    let (regex_text, set_regex_text) = signal("".to_string());
    let (regex_pat, set_regex_pat) = signal("".to_string());
    let (regex_out, set_regex_out) = signal(Out::Idle);
    let (regex_ver, set_regex_ver) = signal("1.13".to_string());
    leptos::mount::mount_to_body(move || {
        let rv = regex_ver;
        let upd_regex1 = Rc::new(move || {
            let binding = regex_text.get();
            let txt = binding.as_str();
            let pat = regex_pat.get();
            let pat = pat.as_str();
            if pat.is_empty() {
                set_regex_out.set(if txt.is_empty() {
                    Out::Idle
                } else {
                    Out::NoMatch
                });
                return;
            }
            set_regex_out.set(match rv.get().as_str() {
                "1.5" => find_all!(regex_1_5, pat, txt),
                "1.9" => find_all!(regex_1_9, pat, txt),
                "1.11" => find_all!(regex_1_11, pat, txt),
                _ => find_all!(regex_1_13, pat, txt),
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
            <option value="1.13">1.13</option>
            <option value="1.11">1.11</option>
            <option value="1.9">1.9</option>
            <option value="1.5">1.5</option>
            </select>" - "<a href=move || format!("https://docs.rs/regex/{}/regex#syntax", regex_ver.get())>syntax reference</a>
            <br />

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
            <br />

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
            <br />

            <label for="regex_out">Output</label>
            <br />
            <pre id="regex_out">{move || out_view(regex_out.get())}</pre>

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

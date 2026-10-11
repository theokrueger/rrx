use leptos::prelude::*;
use std::rc::Rc;

#[derive(Clone)]
struct Hit {
    text: String,
    start: usize,
    end: usize,
    groups: Vec<Option<(usize, usize)>>,
}

#[derive(Clone)]
enum Out {
    Idle,
    Error(String),
    NoMatch,
    Hits(Vec<Hit>),
}

const COLORS: usize = 6;

macro_rules! find_all {
    ($re:ident, $pat:expr, $txt:expr) => {
        match $re::Regex::new($pat) {
            Ok(p) => {
                let hits: Vec<Hit> = p
                    .captures_iter($txt)
                    .filter(|c| c.get(0).is_some_and(|m| !m.as_str().is_empty()))
                    .map(|c| {
                        let m = c.get(0).expect("group 0 always present");
                        let groups = (1..c.len())
                            .map(|i| {
                                c.get(i)
                                    .map(|g| (g.start() - m.start(), g.end() - m.start()))
                            })
                            .collect();
                        Hit {
                            text: m.as_str().to_string(),
                            start: m.start(),
                            end: m.end(),
                            groups,
                        }
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

struct GroupNode {
    group: usize,
    start: usize,
    end: usize,
    children: Vec<usize>,
}

fn group_forest(h: &Hit) -> (Vec<GroupNode>, Vec<usize>) {
    let mut sorted: Vec<(usize, usize, usize)> = h
        .groups
        .iter()
        .enumerate()
        .filter_map(|(gi, g)| g.map(|(s, e)| (gi + 1, s, e)))
        .collect();

    sorted.sort_by(|a, b| a.1.cmp(&b.1).then(b.2.cmp(&a.2)).then(a.0.cmp(&b.0)));

    let mut arena: Vec<GroupNode> = Vec::new();
    let mut roots: Vec<usize> = Vec::new();
    let mut stack: Vec<usize> = Vec::new();
    for (group, start, end) in sorted {
        while let Some(&top) = stack.last() {
            let t = &arena[top];
            if t.start <= start && end <= t.end {
                break;
            }
            stack.pop();
        }
        let idx = arena.len();
        arena.push(GroupNode {
            group,
            start,
            end,
            children: Vec::new(),
        });
        match stack.last() {
            Some(&parent) => arena[parent].children.push(idx),
            None => roots.push(idx),
        }
        stack.push(idx);
    }
    (arena, roots)
}

fn render_group(arena: &[GroupNode], idx: usize, text: &str, depth: usize) -> AnyView {
    let node = &arena[idx];
    let mut parts: Vec<AnyView> = Vec::new();
    let mut pos = node.start;
    for &child in &node.children {
        let c = &arena[child];
        if c.start > pos {
            parts.push(text[pos..c.start].to_string().into_any());
        }
        parts.push(render_group(arena, child, text, depth + 1));
        pos = c.end;
    }
    if pos < node.end {
        parts.push(text[pos..node.end].to_string().into_any());
    }

    let text_cls = format!("cg{}", (node.group - 1) % COLORS + 1);

    let class = if depth > 0 || !node.children.is_empty() {
        format!("{text_cls} u{}", depth % COLORS + 1)
    } else {
        text_cls
    };
    view! { <span class=class>{parts}</span> }.into_any()
}

fn hit_text_view(h: &Hit) -> AnyView {
    let (arena, roots) = group_forest(h);
    let text = h.text.as_str();
    let mut parts: Vec<AnyView> = Vec::new();
    let mut pos = 0;
    for &root in &roots {
        let r = &arena[root];
        if r.start > pos {
            parts.push(text[pos..r.start].to_string().into_any());
        }
        parts.push(render_group(&arena, root, text, 0));
        pos = r.end;
    }
    if pos < text.len() {
        parts.push(text[pos..].to_string().into_any());
    }
    view! { {parts} }.into_any()
}

fn out_view(out: Out) -> AnyView {
    match out {
        Out::Idle => view! { <span class="dim">"no pattern/text"</span> }.into_any(),
        Out::Error(msg) => view! { <span class="red">{msg}</span> }.into_any(),
        Out::NoMatch => view! { <span class="dim">"no matches"</span> }.into_any(),
        Out::Hits(hits) => {
            let total = hits.len();
            let iw = total.to_string().len();
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
                        {hit_text_view(h)}
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
            </select>
            " - "
            <a href=move || {
                format!("https://docs.rs/regex/{}/regex#syntax", regex_ver.get())
            }>syntax reference</a>
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
                <a href="https://gitlab.com/theokrueger/rrx">source</a>
                ,
                " made with "
                <a href="https://www.leptos.dev/">Leptos</a>
                " "
                and
                " "
                <a href="https://crates.io/crates/regex">regex</a>
                ". clientsided, no tracking."
            </p>
        }
    })
}

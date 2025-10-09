use std::sync::Arc;

use leptos::{IntoView, component};
use leptos::{ev, prelude::*};

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    use leptos::html::{button, div};

    let (count, set_count) = signal(0);

    let inc = move |_| *set_count.write() += 1;
    let dec = move |_| *set_count.write() -= 1;

    div()
        .child((
            button()
                .child("-")
                .on(ev::click, dec)
                .class("rounded border text-terminal-purple size-32"),
            ProgressBar(ProgressBarProps {
                current: count,
                children: Arc::new(move || {
                    div()
                        .child(move || count.get())
                        .class("w-full text-center text-lg")
                        .into_any()
                }),
            }),
            button()
                .child("+")
                .on(ev::click, inc)
                .class("rounded border fg-terminal-red size-32"),
        ))
        .class("w-full h-full bg-storm flex flex-row")
    /*
    leptos::view! {
        <div>
            <button on:click=inc class="bg-green-600 rounded">+</button>
            <ProgressBar current=count>
                <p>{move || count.get()}</p>
            </ProgressBar>
            <button on:click=dec class="bg-red-600 rounded">-</button>
        </div>
    }
    */
}

#[component]
fn ProgressBar(current: ReadSignal<i32>, children: ChildrenFn) -> impl IntoView {
    use leptos::html::{div, progress};

    div()
        .child(
            progress()
                .max(20)
                .value(move || current.get())
                .class("rounded border w-full h-full"),
        )
        .child(children())
        .class("flex-grow")
}

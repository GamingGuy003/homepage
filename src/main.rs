use std::sync::Arc;

use leptos::{
    IntoView, component, ev,
    html::{button, div, progress},
    prelude::{
        ChildrenFn, ClassAttribute, ElementChild, Get, IntoAny, OnAttribute, ReadSignal, Write,
        signal,
    },
};

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    let inc = move |_| *set_count.write() += 1;
    let dec = move |_| *set_count.write() -= 1;

    /*
    div().child((
        button()
            .child("-")
            .on(ev::click, dec)
            .class("border")
            .class("bg-blue-600")
            .class("hover:bg-blue-600"),
        ProgressBar(ProgressBarProps {
            current: count,
            children: Arc::new(move || div().child(move || count.get()).into_any()),
        }),
        button().child("+").on(ev::click, inc),
    ))
    */
    leptos::view! {
        <div>
            <button on:click=inc class="bg-green-600 rounded">+</button>
            <ProgressBar current=count>
                <p>{move || count.get()}</p>
            </ProgressBar>
            <button on:click=dec class="bg-red-600 rounded">-</button>
        </div>
    }
}

#[component]
fn ProgressBar(current: ReadSignal<i32>, children: ChildrenFn) -> impl IntoView {
    div()
        .child(progress().max(20).value(move || current.get()))
        .child(children())
}

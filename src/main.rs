use leptos::{
    IntoView, component, ev,
    html::{button, div, progress},
    prelude::{ElementChild, Get, OnAttribute, ReadSignal, Write, signal},
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

    div().child((
        button().child("-").on(ev::click, dec),
        ProgressBar(ProgressBarProps { current: count }),
        button().child("+").on(ev::click, inc),
    ))
}

#[component]
fn ProgressBar(current: ReadSignal<i32>) -> impl IntoView {
    progress().max(20).value(move || current.get())
}

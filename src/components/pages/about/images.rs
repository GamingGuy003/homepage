use leptos::prelude::*;

use leptos::html::{div, img};

#[component]
pub fn Images() -> impl IntoView {
    div()
        .class("w-full flex flex-1 justify-center max-h-[50vh]")
        .child(div()
            .class("grid grid-cols-2 grid-rows-3 gap-3 aspect-square md:aspect-auto relative")
            .child((
                div()
                    .class(
                        "relative overflow-hidden rounded-lg group col-span-2 w-full shadow-xl
                        transition-transform duration:250 hover:scale-175 hover:z-100"
                    )
                    .child((
                        img()
                            .class("h-full w-full rounded-lg object-cover object-center")
                            .src("./static/images/proxmox.png")
                            .alt("Proxmox cluster overview"),
                        div()
                            .class(
                                "absolute inset-0 flex items-center text-center justify-center
                                text-foreground dark:text-foreground-dark text-xl font-semibold opacity-0
                                bg-background-ui dark:bg-background-ui-dark text-shadow-xl p-5
                                group-hover:opacity-75 transition duration-250")
                            .child("Current cluster overview")
                    )),
                div()
                    .class(
                        "relative overflow-hidden rounded-lg group shadow-xl
                        transition-transform duration:250 hover:scale-175 hover:z-100"
                    )
                    .child((
                        img()
                            .class("h-full w-full rounded-lg object-cover object-center")
                            .src("./static/images/keyboard.jpg")
                            .alt("Keyboard and mouse on table"),
                        div()
                            .class(
                                "absolute inset-0 flex items-center text-center justify-center
                                text-foreground dark:text-foreground-dark text-xl font-semibold opacity-0
                                bg-background-ui dark:bg-background-ui-dark text-shadow-xl p-5
                                group-hover:opacity-75 transition duration-250")
                            .child("Current keyboard")
                    )),
                div()
                    .class(
                        "shadow-md rounded-lg relative overflow-hidden group flex w-full
                        transition-transform duration:250 hover:scale-175 hover:z-100"
                    )
                    .child((
                        div()
                            .class("grid grid-cols-2 grid-rows-2 gap-2 p-2 max-h-full place-items-center bg-background-ui dark:bg-background-content-dark")
                            .child((
                                img()
                                    .class("h-full aspect-square")
                                    .src("https://upload.wikimedia.org/wikipedia/commons/thumb/3/3f/Fedora_logo.svg/2048px-Fedora_logo.svg.png")
                                    .alt("Fedora my beloved"),
                                img()
                                    .class("h-full aspect-square")
                                    .src("https://upload.wikimedia.org/wikipedia/commons/thumb/1/13/Arch_Linux_%22Crystal%22_icon.svg/2048px-Arch_Linux_%22Crystal%22_icon.svg.png")
                                    .alt("Arch my first"),
                                img()
                                    .class("h-full aspect-square")
                                    .class("h-full")
                                    .src("https://upload.wikimedia.org/wikipedia/commons/1/19/Gentoo_Logo_Vector.svg")
                                    .alt("Gentoo for maximum braindamage"),
                                img()
                                    .class("h-full aspect-square")
                                    .src("https://avatars.githubusercontent.com/u/2678585?s=200&v=4")
                                    .alt("Ol'reliable")
                            )),
                        div()
                            .class(
                                "absolute inset-0 flex items-center text-center justify-center
                                text-foreground dark:text-foreground-dark text-xl font-semibold opacity-0
                                bg-background-ui dark:bg-background-ui-dark text-shadow-xl p-5
                                group-hover:opacity-75 transition duration-250")
                            .child("Favorite distros")

                    )),
                div()
                    .class(
                        "relative overflow-hidden rounded-lg group col-span-2 shadow-xl
                        transition-transform duration:250 hover:scale-175 hover:z-100"
                    )
                    .child((
                        img()
                            .class("h-full w-full col-span-2 rounded-lg object-cover object-center")
                            .src("./static/images/germany.jpg")
                            .alt("Germany"),
                        div()
                            .class(
                                "absolute inset-0 flex items-center text-center justify-center
                                text-foreground dark:text-foreground-dark text-xl font-semibold opacity-0
                                bg-background-ui dark:bg-background-ui-dark text-shadow-xl p-5
                                group-hover:opacity-75 transition duration-250")
                            .child("Picture taken in Germany")
                    ))
            ))
    )
}

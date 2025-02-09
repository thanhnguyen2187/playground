use maud::{html, Markup, DOCTYPE};

pub fn header(page_title: &str) -> Markup {
    html! {
        (DOCTYPE)
        head {
            meta charset="utf-8";
            title { (page_title) };
            script src="https://unpkg.com/htmx.org@2.0.4" {""};
            script defer src="https://unpkg.com/alpinejs@3.14.8" {""};
            link rel="stylesheet" type="text/css" href="./styles.css";
        }
    }
}

pub async fn home() -> Markup {
    html! {
        (header("TODO Home"))
        body {
            div .container .mx-auto .p-4 .flex .flex-col .gap-2 {
                h1 .text-3xl .font-bold { "TODO Home" }
                table .table .w-80 {
                    thead {
                        tr {
                            th { "Title" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        tr {
                            td {
                                "loren ipsum"
                            }
                            td .flex .gap-2 {
                                button .btn .btn-success { "Finish" }
                                button .btn .btn-primary { "Edit" }
                                button .btn { "Delete" }
                            }
                        }
                        tr {
                            td {
                                input .input .input-bordered type="text" value="loren ipsum";
                            }
                            td .flex .gap-2 {
                                button .btn .btn-primary { "Done" }
                            }
                        }
                        tr {
                            td .line-through {
                                "loren ipsum"
                            }
                            td .flex .gap-2 {
                                button .btn .btn-warning { "Reopen" }
                                button .btn .btn-primary { "Edit" }
                                button .btn { "Delete" }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub async fn page_unimplemented() -> Markup {
    html! {
        (header("Unimplemented!!1"))
        body {
            div .container .mx-auto .p-4 {
                h1 .text-3xl .font-bold .mb-2 { "Unimplemented" }
                p { "Please check again later" }
            }
        }
    }
}

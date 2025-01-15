use maud::{html, Markup};
use crate::common::{header, home_back_link};

pub async fn page() -> Markup {
    html! {
        (header("Timer"))
        body {
            h1 { "Timer" }
            form
                x-data="{
                    duration: 10_000,
                    elapsed: 0,
                    interval: undefined,
                    reset() {
                        this.elapsed = 0;
                        if (this.interval !== undefined) {
                            clearInterval(this.interval);
                            this.interval = undefined;
                        }
                        this.start();
                    },
                    start() {
                        if (this.interval !== undefined) {
                            return;
                        }
                        this.interval = setInterval(() => {
                            if (this.elapsed < this.duration) {
                                this.elapsed += 100;
                            } else {
                                clearInterval(this.interval);
                                this.interval = undefined;
                            }
                        }, 100);
                    },

                }"
                x-init="
                    start();
                "
            {
                fieldset {
                    label {
                        "Elapsed Time: "
                        progress
                            ":value"="elapsed"
                            ":max"="duration"
                        ;
                    }
                    label
                        .smaller
                        x-text="elapsed + ' ms / ' + duration + ' ms'"
                    {""};
                    label {
                        "Duration: "
                        input
                            type="range"
                            name="duration"
                            min="5000"
                            x-model="duration"
                            max="30000"
                            "@change"="start()"
                        ;
                    }
                    button
                        type="button"
                        "@click"="reset()"
                        { "Reset" };
                }
            }
            (home_back_link())
        }
    }
}

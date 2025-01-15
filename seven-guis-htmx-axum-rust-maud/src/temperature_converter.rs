use maud::{html, Markup};
use crate::common::{header, home_back_link};

pub async fn page() -> Markup {
    html! {
        (header("Temperature Converter"))
        body {
            h1 { "Temperature Converter" }
            form x-data="{ celsius: 0, fahrenheit: 32 }" {
                fieldset {
                    label {
                        "Celsius: "
                        input
                            x-model="celsius"
                            "@keyup"="fahrenheit = (celsius * (9 / 5)) + 32"
                            type="number"
                            name="celsius";
                    }
                    label {
                        "Fahrenheit: "
                        input
                            x-model="fahrenheit"
                            "@keyup"="celsius = (fahrenheit - 32) * (5 / 9)"
                            type="number"
                            name="fahrenheit";
                    }
                }
            }
            (home_back_link())
        }
    }
}
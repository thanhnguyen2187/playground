use maud::{html, Markup};
use crate::index::header;

pub struct OneWayFlight {
    from: String,
}

pub struct ReturnFlight {
    from: String,
    to: String,
}

pub enum FlightBookerState {
    OneWay(OneWayFlight),
    Return(ReturnFlight),
}

pub async fn page() -> Markup {
    html! {
        (header("Flight Booker"))

        body {
            h1 { "Flight Booker" }
        }
    }
}
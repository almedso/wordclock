use wordclock::WordClock;
use yew::prelude::*;
use chrono::{Utc, Timelike};


#[function_component(App)]
pub fn app() -> Html {

    let word_clock = WordClock::new("ch-bern".to_string());
    let now = Utc::now();

    let display_letters = word_clock
        .show_time_iterator(now.hour() as usize, now.minute() as usize)
        .map(|(letter, highlight, _end_of_row)| {
            html! {
                if highlight {
                    <div style="color:green;">  { format!("{}", letter)  } </div>
                } else {
                    <div style="color:#333333;"> { format!("{}", letter) } </div>
                }
            }
        })
        .collect::<Html>();

    html! {
        <main>
            <div class="grid-container">
                {display_letters}
            </div>
        </main>
    }
}

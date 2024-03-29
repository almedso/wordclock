use chrono::{Timelike, Utc};
use gloo::timers::callback::Interval;
use wordclock::WordClock;
use yew::prelude::*;

pub enum Msg {
    UpdateTime,
}

#[derive(Properties, Clone, PartialEq)]
pub struct LangProps {
    pub language: String
}

pub struct WordClockComponent {
    _standalone: Interval,
    language: String,
    hour: usize,
    minute: usize,
}

impl WordClockComponent {
    fn get_current_time() -> (usize, usize) {
        let now = Utc::now();
        (now.hour() as usize, now.minute() as usize)
    }
}

impl Component for WordClockComponent {
    type Message = Msg;
    type Properties = LangProps;

    fn create(ctx: &Context<Self>) -> Self {
        let _standalone = {
            let link = ctx.link().clone();
            Interval::new(1000, move || link.send_message(Msg::UpdateTime))
        };
        let language = ctx.props().language.clone();

        let (hour, minute) = WordClockComponent::get_current_time();
        Self {
            _standalone,
            hour,
            minute,
            language,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateTime => {
                (self.hour, self.minute) = WordClockComponent::get_current_time();
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {

        let word_clock = WordClock::new(self.language.clone());
        let display_letters = word_clock
            .show_time_iterator(self.hour, self.minute)
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
}

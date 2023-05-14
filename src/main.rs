use iced::Settings;
use iced::Sandbox;
use iced::widget::{Widget, Button, Text, Column, Row, Renderer, Container};

const CH_BERN_GRID: [char; 11 * 11] =  [
    'E', 'S', 'K', 'I', 'S', 'C', 'H', 'A', 'F', 'Ü', 'F',
    'V', 'I', 'E', 'R', 'T', 'U', 'B', 'F', 'Z', 'Ä', 'Ä',
    'Z', 'W', 'Ä', 'N', 'Z', 'G', 'S', 'I', 'V', 'O', 'R',
    'A', 'B', 'O', 'H', 'A', 'U', 'B', 'I', 'E', 'P', 'M',
    'E', 'I', 'S', 'Z', 'W', 'O', 'I', 'S', 'D', 'R', 'Ü',
    'V', 'I', 'E', 'R', 'F', 'Ü', 'N', 'F', 'I', 'Q', 'T',
    'S', 'E', 'C', 'H', 'S', 'I', 'S', 'I', 'B', 'N', 'I',
    'A', 'C', 'H', 'T', 'I', 'N', 'Ü', 'N', 'I', 'E', 'L',
    'Z', 'Ä', 'N', 'I', 'E', 'R', 'B', 'E', 'U', 'F', 'I',
    'Z', 'W', 'Ö', 'U', 'F', 'I', 'A', 'M', 'U', 'H', 'R',
    ' ', ' ', ' ', '*', '*', '*', '*', ' ', ' ', ' ', ' ',
];


struct Counter {
    count: i32
}

#[derive(Debug, Clone, Copy)]
enum CounterMessage {
    Increment,
    Decrement
}

impl Sandbox for Counter {
    type Message = CounterMessage;

    fn new() -> Self {
        Counter{ count: 0 }
    }

    fn title(&self) -> String {
        String::from("Counter app")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            CounterMessage::Increment => self.count += 1,
            CounterMessage::Decrement => self.count -= 1
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {


        let col: Column<Text<Self::Message, Renderer >> = CH_BERN_GRID.into_iter()
            .map( |c|  Text::new(c.to_string()) )
            .collect();


        // let label = Text::new(format!("Count: {}", self.count));
        // let col = Column::new().push(incr).push(label).push(decr);

        Container::new(col).center_x().center_y().width(iced::Length::Fill).height(iced::Length::Fill).into()


    }
}

fn main() -> Result<(), iced::Error> {
    Counter::run(Settings::default())
}


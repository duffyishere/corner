use iced::{Canvas, Color, Length, Point, Rectangle, Sandbox, Settings, Size};
use iced::canvas::{Frame, Path, Program, Stroke, Fill};

pub fn main() -> iced::Result {
    // Counter::run(Settings::default())
    RectangleApp::run(Settings::default())
}

struct RectangleApp;

impl Sandbox for RectangleApp {
    type Message = ();

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        "Rectangle App".into()
    }

    fn update(&mut self, message: Self::Message) {}

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        Canvas::new(RectangleProgram)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

struct RectangleProgram;

impl Program<()> for RectangleProgram {
    fn draw(&self, bounds: Rectangle, _: iced::canvas::Cursor) -> Vec<iced::canvas::Geometry> {
        let mut frame = Frame::new(bounds.size());
        let rectangle = &Path::rectangle(
            Point {
                    x: bounds.width / 10.,
                    y: bounds.height / 10.,
                },
                Size {
                    width: 3. * bounds.width / 5.,
                    height: 4. * bounds.height / 5.,
                },
            );
        frame.stroke(
            rectangle,
            Stroke::default(),
        );

        vec![frame.into_geometry()]
    }
}

// // Counter App Code;
// struct Counter {
//     value: i32,
//     increment_button: button::State,
//     decrement_button: button::State,
// }
//
// #[derive(Debug, Clone, Copy)]
// pub enum Message {
//     IncrementPressed,
//     DecrementPressed,
// }
//
// impl Sandbox for Counter {
//     type Message = Message;
//
//     fn new() -> Self {
//         Self {
//             value: 0,
//             increment_button: Default::default(),
//             decrement_button: Default::default()
//         }
//     }
//
//     fn title(&self) -> String {
//         String::from("counter")
//     }
//
//     fn update(&mut self, message: Message) {
//         match message {
//             Message::IncrementPressed => {
//                 self.value += 1;
//             }
//             Message::DecrementPressed => {
//                 self.value -= 1;
//             }
//         }
//     }
//
//     fn view(&mut self) -> Element<Message> {
//         Column::new()
//             .push(
//             Button::new(&mut self.increment_button, Text::new("+"))
//                 .on_press(Message::IncrementPressed),
//             )
//             .push(
//                 Text::new(self.value.to_string()).size(50)
//             )
//             .push(
//                 Button::new(&mut self.decrement_button, Text::new("-"))
//                     .on_press(Message::DecrementPressed)
//             )
//             .padding(20)
//             .align_items(Alignment::Center)
//             .into()
//     }
// }

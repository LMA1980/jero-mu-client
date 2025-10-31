use iced::{
    alignment,
    widget::{button, column, container, row, text, rule},
    window,
    Application, Element, Length, Settings, Theme,
};

pub fn main() -> iced::Result {
    Jero::run(Settings::default())
}

#[derive(Default)]
struct Jero {
    menu_open: bool,
}

#[derive(Debug, Clone)]
enum Message {
    MenuToggled,
    Exit,
}

impl Application for Jero {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Jero::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Jero")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::MenuToggled => {
                self.menu_open = !self.menu_open;
                Command::none()
            }
            Message::Exit => window::close(),
        }
    }

    fn view(&self) -> Element<Message> {
        let burger_button = button(text("☰").size(30))
            .on_press(Message::MenuToggled)
            .padding(5);

        let menu_bar = row![burger_button]
            .spacing(10)
            .align_items(alignment::Alignment::Center)
            .width(Length::Fill);

        let main_content = container(text("Hello, Jero!"))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y();

        let mut content = column![menu_bar];

        if self.menu_open {
            let menu = column![
                button("Home").width(Length::Fill),
                button("About..").width(Length::Fill),
                rule::Rule::horizontal(10),
                button("Exit").width(Length::Fill).on_press(Message::Exit),
            ]
            .spacing(5)
            .width(200)
            .padding(10);

            content = content.push(menu);
        }

        content = content.push(main_content);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

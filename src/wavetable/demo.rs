use iced::widget::{text};
use iced::{Alignment, Sandbox};

pub struct WtDemo {
    pos: usize,
}


impl Sandbox for WtDemo {
    type Message = ();

    fn new() -> Self {
        WtDemo { pos: 0 }
    }

    fn title(&self) -> String {
        "Wavetable Demo".to_string()
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> iced::Element<'_, Self::Message> {
        iced::widget::column!(text("wt demo").size(50))
            .padding(20)
            .align_items(Alignment::Start)
            .into()
    }
}

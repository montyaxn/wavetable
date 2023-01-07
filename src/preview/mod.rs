pub mod sound;

use iced::widget::canvas::{Cache, Canvas, Path};
use iced::widget::{canvas, column, container, slider};
use iced::{executor, Application, Color, Command, Length, Point, Theme};
use rodio::{OutputStream, Sink, Source};

use crate::waveform::Waveform;

pub fn preview(wt: [Waveform; 256]) -> iced::Result {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let freq = 440.0;
    let index = 0;
    let source =
        sound::WtWave::new(wt.clone()).repeat_infinite();
    sink.append(source);

    WtPreviewer::run(iced::Settings::with_flags(Flags {
        wt: wt,
        freq: freq,
        index: index,
    }))
}
pub struct WtPreviewer {
    image: WtImage,
    freq: f32,
    index_textin: String,
}

pub struct Flags {
    pub wt: [super::Waveform; 256],
    pub freq: f32,
    pub index: u8,
}

#[derive(Debug, Clone)]
pub enum Message {
    IndexChanged(u8),
    FreqChanged(f32),
}

impl Application for WtPreviewer {
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = Flags;
    type Message = Message;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let image = WtImage {
            wt: flags.wt,
            index: flags.index,
            image_cache: Default::default(),
        };
        (
            WtPreviewer {
                image: image,
                freq: flags.freq,
                index_textin: "0".to_string(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Wavetable Previewer".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::IndexChanged(i) => {
                self.image.index = i;
                self.index_textin = i.to_string();

                self.image.image_cache.clear();
            }
            Message::FreqChanged(f) => {
                self.freq = f;
            }
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let index_slider =
            container(slider(0..=255, self.image.index, Message::IndexChanged)).width(Length::Units(300));

        let freq_slider = container(slider(40.0..=40000.0, self.freq, Message::FreqChanged))
            .width(Length::Units(300));

        let wt = Canvas::new(&self.image)
            .width(Length::Units(300))
            .height(Length::Units(200));
        let previewer = column![wt, index_slider, freq_slider].spacing(10);

        container(previewer)
            .height(Length::Fill)
            .width(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

struct WtImage {
    wt: [super::Waveform; 256],
    index: u8,
    image_cache: Cache,
}

impl canvas::Program<Message> for WtImage {
    type State = ();
    fn draw(
        &self,
        _state: &Self::State,
        _theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: canvas::Cursor,
    ) -> Vec<canvas::Geometry> {
        let image_cache = self.image_cache.draw(bounds.size(), |frame| {
            for (i, s) in self.wt[self.index as usize]
                .waveform
                .iter()
                .enumerate()
                .step_by(4)
            {
                let x = frame.width() * (0.05 + (i as f32) / 2048.0 * 0.9);
                let y = frame.height() / 2.0 * (1.0 - s * 0.9);

                let dot = Path::circle(Point::new(x, y), 1.0);
                frame.fill(&dot, Color::BLACK);
            }
        });

        vec![image_cache]
    }
}

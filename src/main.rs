#![feature(trait_upcasting)]

use iced::executor;
use iced::widget::{self, canvas, column, container, text, text_input, row};
use iced_native::widget::{Row, Column};
use iced::{
    Application,
    Alignment, Color, Element, Length, Settings, Theme, Rectangle,
    Subscription, Point, Vector, mouse, Command
};

use iced::widget::canvas::{
    Cache
};



mod constants;
mod dateidea;
mod idea_picker;
mod roulette_wheel;
mod deck_of_cards;

use crate::deck_of_cards::DeckOfCards;
use crate::idea_picker::{
    IdeaPickerAnimated,
    IdeaPicker,
    AnimationState,
    Message,
    DynCanvasProgram

};

use crate::roulette_wheel::RouletteWheel;



struct DateJar{
    data: String,
    date_ideas: dateidea::DateIdeas,
    animation: Box<dyn IdeaPickerAnimated<State = ()>>,
}

impl DateJar{
    pub fn new(ideas: dateidea::DateIdeas, animation: Box<dyn IdeaPickerAnimated<State = ()>>) -> Self {
        DateJar {
            data: "".to_string(),
            date_ideas: ideas,
            animation,
        }
    }
}


pub fn main() -> iced::Result{ 
    DateJar::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

impl Application for DateJar {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            DateJar::new(dateidea::get_data_idea("DateIdea.yaml".to_string()), Box::new(RouletteWheel::new())),
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Tomas and Yilly's Date Jar")
    }

    fn update(&mut self, message: Message) -> Command<Message>{
        match message {
            Message::StartSpin => {
                //TODO: Take the needed Idea
                //self.random_animations.ideas().append(&mut self.date_ideas.ideas);
                self.animation.ideas_mut().clear();
                for i in self.date_ideas.ideas.clone(){
                    self.animation.ideas_mut().push(i);
                }
                self.animation.get_cache().clear();
                self.animation.start();

                //data.truncate(100);
                //self.data = data;
            }
            Message::NewDateIdea(new_date_idea) => {
                self.animation = Box::new(DeckOfCards::new());
                println!("The new date is {}", new_date_idea.name);
            }
            Message::Animate(progress) => {
                //println!("Recived rotation {}", progress);
                //if self.animation.get_animation_length() == self.animation.get_animation_progress(){
                //    self.animation.display_picked_date();
                //    return Command::none();
                //}
                self.animation.progress_animation(progress);
                self.animation.get_cache().clear();

            }

        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        //let t = self.random_animations.as_ref();
        let canvas = canvas(DynCanvasProgram::new(self.animation.as_ref()))
            .width(Length::Fill)
            .height(Length::Fill);

        let button = |txt|
            widget::button(
                txt
            );
            
        let spin_button = button("Spin").padding(10).on_press(Message::StartSpin);
        //let spin_button = button("Spin").padding(10).on_press(Message::StartSpin);

        let content = Column::new().align_items(Alignment::Center).spacing(20).push(spin_button).push(canvas);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20).into()
    }

    fn subscription(&self) -> Subscription<Message> {
        match self.animation.get_animation_state() {
            AnimationState::Start =>{
                //self.randomAnimations.date_ideas = self.date_ideas.ideas;
                Subscription::none().map(|_: fn()|{
                    Message::Animate(1)
                })

            }
            AnimationState::Running =>{
                let rot = 1;
                iced::time::every(std::time::Duration::from_millis(30)).with(Message::Animate(rot)).map(|(m, _)|{
                    m
                })

            }
            //AnimationState::Stopping => {
            //    return iced::time::every(std::time::Duration::from_millis(30)).map(|_| {
            //        Message::Animate(1)
            //    })
            //}
            _ => {
                //self.random_animations = Box::new(deck_of_cards::DeckOfCards::new());
                //println!("Sending rotation");
                //let rot = 10;
                //iced::time::every(std::time::Duration::from_millis(30)).with(Message::Animate(rot)).map(|(m, _)|{
                //    m
                //})
                Subscription::none()
            }
            
        }
    }
}


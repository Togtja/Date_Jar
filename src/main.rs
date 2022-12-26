#![feature(trait_upcasting)]

use iced::executor;
use iced::widget::{self, canvas, column, container, text, text_input, row};
use iced_native::widget::{Row, Column};
use iced::{
    Application,
    Alignment, Color, Element, Length, Settings, Theme, Rectangle,
    Subscription, Point, Vector, mouse, Command
};


mod constants;
mod dateidea;
mod idea_picker;
mod roulette_wheel;
mod deck_of_cards;
mod tag_checkbox;

use crate::deck_of_cards::DeckOfCards;
use crate::idea_picker::{
    IdeaPickerAnimated,
    AnimationState,
    Message,
    DynCanvasProgram

};

use crate::roulette_wheel::RouletteWheel;



struct DateJar{
    date_ideas: dateidea::DateIdeas,
    animation: Box<dyn IdeaPickerAnimated<State = ()>>,
    tags: Vec<tag_checkbox::TagCheckbox>,
}

impl DateJar{
    pub fn new(ideas: dateidea::DateIdeas, animation: Box<dyn IdeaPickerAnimated<State = ()>>) -> Self {
        let mut tags = Vec::new();
        for mut idea in ideas.ideas.clone() {
            tags.append(&mut idea.tags);
        }
        //Sort and remove consecutive duplicates
        tags.sort_unstable();
        tags.dedup();

        let mut checkbox_tags = Vec::new();
        for tag in tags{
            let task = tag_checkbox::TagCheckbox::new(tag);
            checkbox_tags.push(task);
        }

        DateJar {
            date_ideas: ideas,
            animation,
            tags: checkbox_tags,
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
        //TODO: make a function to read dateideas from yaml files
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
            Message::UpdateTags(i, taskm, ) =>{
                if let Some(tag) = self.tags.get_mut(i) {
                    tag.update(taskm);
                }

                self.animation.ideas_mut().clear();

                let mut tags = Vec::new();

                for t_tag in &self.tags {
                    if !t_tag.in_use_ref() {
                        continue;
                    }
                    tags.push(t_tag.get_name_ref());
                }
                
                for i in self.date_ideas.ideas.clone(){
                    let mut has_tag = true;

                    for tag in &tags {
                        if !i.tags.contains(tag) {
                            has_tag = false;
                            break;
                        }
                    }
            
                    if has_tag && !self.animation.ideas_ref().contains(&i){
                        self.animation.ideas_mut().push(i);
                    }

                }
                self.animation.get_cache().clear();
            }

            Message::StartSpin => {
                //TODO: Take the needed Idea
                //self.random_animations.ideas().append(&mut self.date_ideas.ideas);
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
        let tags = column(
            self.tags
            .iter()
            .enumerate()
            .map(|(i, tag)| {
                tag.view().map(move |message|{
                    Message::UpdateTags(i, message)
                })
            })
            .collect()
        )
        .spacing(10);
        //let spin_button = button("Spin").padding(10).on_press(Message::StartSpin);
        //let filtered_tags =
        //    self.tags.iter().filter(|tags| filter.matches(tags));

        let spinny = Column::new().align_items(Alignment::Center).spacing(20).push(spin_button).push(canvas);
        let content = row![tags, spinny];
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


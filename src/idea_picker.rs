use iced::widget::canvas::{
    Cache
};

use iced::widget::{canvas};

use crate::dateidea::{
    DateIdea
};



#[derive(Default, PartialEq)]
pub enum AnimationState{
    #[default]
    Idle,
    Start,
    Running,
    Stopping, // Only used if you are slowing down the animation
    Finished,
}

#[derive(Debug, Clone, Hash)]
pub enum Message {
    StartSpin,
    Animate(u32),
    NewDateIdea(DateIdea),
}

pub trait IdeaPicker: canvas::Program<Message>{
    fn ideas_mut(&mut self) -> &mut Vec<DateIdea>;
    fn ideas_ref(&self) -> &Vec<DateIdea>;
    fn pick_idea(&self) -> (&DateIdea, usize);
    //fn get_self(&self) -> &dyn canvas::Program<Message, State = ()>;
}

pub trait IdeaPickerAnimated: IdeaPicker + canvas::Program<Message> { 
    fn get_cache(&self) -> &Cache;

    fn get_animation_state(&self) -> &AnimationState;
    fn start(&mut self);
    fn display_picked_date(&mut self);
    fn progress_animation(&mut self, progress: u32);
    fn get_animation_progress(&self) -> u32;
    fn get_animation_length(&self) -> u32;
    
}


//#[derive(Clone, Copy)]
pub struct DynCanvasProgram<'a>{
    a : &'a dyn canvas::Program<Message, State = ()>,
    //b : &'a mut dyn IdeaPickerAnimated<State = ()>
}

impl<'a> DynCanvasProgram<'a> {
    pub fn new(t: &'a dyn canvas::Program<Message, State = ()>) -> Self{
        DynCanvasProgram{
            a: t
        }
    }
}

impl<'a> canvas::Program<Message> for DynCanvasProgram<'a> {
    type State = ();

    fn draw(
        &self,
        state: &Self::State,
        theme: &iced_native::Theme,
        bounds: iced::Rectangle,
        cursor: canvas::Cursor,
    ) -> Vec<canvas::Geometry> {
        self.a.draw(state, theme, bounds, cursor)
    }
}

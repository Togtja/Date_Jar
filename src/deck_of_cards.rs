

use rand::seq::SliceRandom;


use iced::widget::{canvas};
use iced::widget::canvas::{
    Cache, Cursor, Geometry, Path
};

use crate::canvas::path::Arc;

use iced::{Color, Theme, Rectangle, Point, Vector};

use crate::idea_picker::{
    AnimationState,
    IdeaPickerAnimated,
    IdeaPicker
};

use crate::dateidea::{
    DateIdea
};

use crate::constants::{
    COLORS
};




#[derive(Default)]
pub struct DeckOfCards{
    state: AnimationState,
    cache: Cache,
    rotation: u32,
    date_ideas: Vec<DateIdea>
}



impl DeckOfCards {
    pub fn new() -> Self {
        DeckOfCards { 
            state: AnimationState::Idle,
            cache: Default::default(),
            rotation: Default::default(),
            date_ideas: Default::default(),
        }
    }

}

impl IdeaPicker for DeckOfCards {
    fn ideas_mut(&mut self) -> &mut Vec<DateIdea> {
        return &mut self.date_ideas
    }

    fn ideas_ref(&self) -> &Vec<DateIdea> {
        return &self.date_ideas
    }

    fn pick_idea(&self) -> (&DateIdea, usize) {
        //TODO: Actually pick a random Idea
        //self.ideas_ref().choose(&mut rand::thread_rng()).get_or_insert(&self.ideas_ref()[0])
        todo!()
    }


}

impl <Message> canvas::Program<Message> for DeckOfCards {
        type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let roulette_wheel = self.cache.draw(bounds.size(), |frame| {
            let center = frame.center();
            let radius = frame.width().min(frame.height()) / 2.0;
            frame.translate(Vector::new(center.x, center.y));

            
            //frame.rotate(rotation(self.rotation, 360));
            
            //frame.stroke(&divide_circle(0,20, radius), thin_stroke(COLORS[0 as usize % COLORS.len()]));
            //frame.stroke(&divide_circle(3,20, radius), thin_stroke(COLORS[3 as usize % COLORS.len()]));
            if self.date_ideas.len() > 0 {
                for i in 0u32..self.date_ideas.len() as u32 {
                    frame.fill(&divide_circle(i,self.date_ideas.len() as u32, radius), COLORS[i as usize % COLORS.len()]);
                    //frame.fill_text(text("Hello").into());
                    //frame.stroke(&divide_circle(i,20, radius), thin_stroke(COLORS[i as usize % COLORS.len()]));
                }
            } 
            else{
                frame.fill(&Path::circle(Point::ORIGIN, radius), Color::BLACK);
            }

            

            //frame.with_save(|frame| {
            //});

            //frame.with_save(|frame| {
            //    frame.rotate(hand_rotation(self.state, 12));
            //    frame.stroke(&short_hand, wide_stroke());
            //});
        });

        vec![roulette_wheel]
    }


    //pub fn progress(&mut self, new_progress)
}


impl IdeaPickerAnimated for DeckOfCards {

    fn get_cache(&self) -> &Cache {
        return &self.cache
    }

    fn get_animation_state(&self) -> &AnimationState {
        return &self.state
    }
    
    fn start(&mut self) {
        match self.state {
            AnimationState::Idle =>{
                self.state = AnimationState::Running
            }
            _ => {}
        }
    }
    
    fn display_picked_date(&mut self) {
        todo!()
    }

    fn progress_animation(&mut self, progress: u32) {
        self.rotation += progress;
    }

    fn get_animation_progress(&self) -> u32 {
        todo!()
    }

    fn get_animation_length(&self) -> u32 {
        todo!()
    }
}


fn rotation(n: u32, total: u32) -> f32 {
    let turns = n as f32 / total as f32;

    2.0 * std::f32::consts::PI * turns
}

fn divide_circle(n: u32, total: u32, radius: f32) -> Path{
    let angle_size = 1.0 / total as f32 * std::f32::consts::PI * 2.0;
    let x = radius * (angle_size * n as f32).cos();
    let y = radius * (angle_size * n as f32).sin();
     Path::new(|p| {
        p.move_to(Point::ORIGIN);
        p.line_to(Point::new(x,  y));
        p.arc(Arc {
            center: Point::ORIGIN,
            radius: radius,
            start_angle: angle_size * n as f32,
            end_angle:   angle_size * (n+1) as f32,
        });
        p.line_to(Point::ORIGIN);
    })
}

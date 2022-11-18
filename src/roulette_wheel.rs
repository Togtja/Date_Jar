

use rand::seq::{SliceRandom, IteratorRandom};


use iced::widget::{canvas};
use iced::widget::canvas::{
    Cache, Cursor, Geometry, Path
};

use crate::canvas::path::Arc;

use iced::{Color,  Theme, Rectangle, Point, Vector};

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
pub struct RouletteWheel{
    state: AnimationState,
    cache: Cache,
    rotation: f32,
    started_rotation: f32,
    progress: u32,
    max_progress: u32,
    date_ideas: Vec<DateIdea>,
}



impl RouletteWheel {
    pub fn new() -> Self {
        RouletteWheel { 
            state: AnimationState::Idle,
            ..Default::default()
        }
    }

    const MAX_ROTATION: u32 = 360;

}

impl IdeaPicker for RouletteWheel {
    fn ideas_mut(&mut self) -> &mut Vec<DateIdea> {
        return &mut self.date_ideas
    }

    fn ideas_ref(&self) -> &Vec<DateIdea> {
        return &self.date_ideas
    }

    fn pick_idea(&self) -> (&DateIdea, usize) {
        //TODO: Actually pick a random Idea
        let i = *(0..self.date_ideas.len()).choose(&mut rand::thread_rng()).get_or_insert(0) as usize;

        return (&self.ideas_ref()[i], i)
    }

    //fn get_self(&self) -> &dyn canvas::Program<Message, State = ()> {
    //    &self
    //}

}

impl <Message> canvas::Program<Message> for RouletteWheel {
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

            frame.rotate(rotation(self.rotation, RouletteWheel::MAX_ROTATION as f32));


            if self.date_ideas.len() > 0 {
                for i in 0u32..self.date_ideas.len() as u32 {
                    frame.fill(&divide_circle(i as f32,self.date_ideas.len() as f32, radius), COLORS[i as usize % COLORS.len()]);
                    //frame.fill_text(text("Hello").into());
                    //frame.stroke(&divide_circle(i,20, radius), thin_stroke(COLORS[i as usize % COLORS.len()]));
                }
            } 
            else{
                frame.fill(&Path::circle(Point::ORIGIN, radius), Color::BLACK);
            }

        });

        vec![roulette_wheel]
    }


    //pub fn progress(&mut self, new_progress)
}


impl IdeaPickerAnimated for RouletteWheel {

    fn get_cache(&self) -> &Cache {
        return &self.cache
    }

    fn get_animation_state(&self) -> &AnimationState {
        return &self.state
    }

    fn start(&mut self) {
        match self.state {
            AnimationState::Idle | AnimationState::Finished =>{
                self.progress = 0;
                self.state = AnimationState::Running;

                let (date ,i)  = self.pick_idea();
                println!("We picked {} at index: {}", date.name, i);
                //TODO: trixy ludo to set correct max progress
                // predicted + 360/self.date_ideas.len() * i
                self.started_rotation = self.rotation;
                self.max_progress = RouletteWheel::MAX_ROTATION * 3 + 180;
            }
            _ => {}
        }
    }

    
    fn display_picked_date(&mut self) {
        self.state = AnimationState::Finished;
        //todo!()
    }

    fn progress_animation(&mut self, _progress: u32) {
        const ROTATION_SPEED: f32 = 10.0;
        const PROGRESS_STEP: u32 = 1;
        self.progress += PROGRESS_STEP;
        //self.rotation += ROTATION_SPEED;
        
        if self.progress < self.max_progress - RouletteWheel::MAX_ROTATION {
            self.rotation += ROTATION_SPEED;
        }else{
            let progress_left = (self.max_progress - self.progress) as f32 / RouletteWheel::MAX_ROTATION as f32; // This moves us 175/progress_step * ROTATION:SPEED?
            self.rotation += ROTATION_SPEED * progress_left;
        }
        
        
        if self.rotation >= RouletteWheel::MAX_ROTATION as f32 {
            self.rotation = self.rotation - RouletteWheel::MAX_ROTATION as f32;
        }
        
        if self.max_progress <= self.progress {
            let rot_prog_ratio = ROTATION_SPEED/PROGRESS_STEP as f32;
            
            let basic_rot = (self.max_progress - RouletteWheel::MAX_ROTATION) as f32;
            let end_rot = (RouletteWheel::MAX_ROTATION as f32 / 2.0) - (0.5 * PROGRESS_STEP as f32);
            let predicted = self.started_rotation + (rot_prog_ratio * (basic_rot + end_rot));
            let predicted_rot = predicted - (predicted / RouletteWheel::MAX_ROTATION as f32).floor() *  RouletteWheel::MAX_ROTATION as f32;

            println!("Stopped at {}, predicted stop at {}={}+{}+{}-{}", self.rotation, predicted_rot, self.started_rotation, basic_rot, end_rot, (predicted / RouletteWheel::MAX_ROTATION as f32).floor() *  RouletteWheel::MAX_ROTATION as f32);
            self.state = AnimationState::Finished;
        }
    }

    fn get_animation_progress(&self) -> u32 {
        return self.progress
    }

    fn get_animation_length(&self) -> u32 {
        //TODO: Random max so that it stops on the idea we want
        return RouletteWheel::MAX_ROTATION * 3;
    }
}


fn rotation(n: f32, total: f32) -> f32 {
    let turns = n / total;

    2.0 * std::f32::consts::PI * turns
}

fn divide_circle(n: f32, total: f32, radius: f32) -> Path{
    let angle_size = 1.0 / total * std::f32::consts::PI * 2.0;
    let x = radius * (angle_size * n as f32).cos();
    let y = radius * (angle_size * n as f32).sin();
     Path::new(|p| {
        p.move_to(Point::ORIGIN);
        p.line_to(Point::new(x,  y));
        p.arc(Arc {
            center: Point::ORIGIN,
            radius,
            start_angle: angle_size * n ,
            end_angle:   angle_size * (n+ 1.0),
        });
        p.line_to(Point::ORIGIN);
    })
}

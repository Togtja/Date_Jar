

use iced::alignment::{Horizontal, Vertical};
use rand::Rng;
use rand::seq::{SliceRandom, IteratorRandom};


use iced::widget::{canvas};
use iced::widget::canvas::{
    Cache, Cursor, Geometry, Path, Text
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
    max_progress: f32,
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
    const ROTATION_SPEED: f32 = 10.0;
    const PROGRESS_STEP: u32 = 1;

}

impl IdeaPicker for RouletteWheel {
    fn ideas_mut(&mut self) -> &mut Vec<DateIdea> {
        return &mut self.date_ideas
    }

    fn ideas_ref(&self) -> &Vec<DateIdea> {
        return &self.date_ideas
    }

    fn pick_idea(&self) -> (&DateIdea, usize) {
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
            let radius = frame.width().min(frame.height()) / 2.5;

            
            frame.translate(Vector::new(center.x, center.y));
            
            frame.with_save(|frame|{
                
                frame.rotate(rotation(self.rotation, RouletteWheel::MAX_ROTATION as f32));
                
                let text = canvas::Text {
                    horizontal_alignment: Horizontal::Center,
                    vertical_alignment: Vertical::Top,
                    size: 30.0,
                    ..canvas::Text::default()
                };
                
                if self.date_ideas.len() > 0 {
                    for i in 0..self.date_ideas.len() {
                        let name = &self.date_ideas[i].name;
                        frame.fill(&divide_circle(i as f32,self.date_ideas.len() as f32, radius), COLORS[i % COLORS.len()]);
                        
                        let angle_size = 1.0 / self.date_ideas.len() as f32 * std::f32::consts::PI * 2.0;
                        let x = radius/1.5 * (angle_size * (i as f32 + 0.5)).cos();
                        let y = radius/1.5 * (angle_size * (i as f32 + 0.5)).sin();
                        frame.with_save(|frame|{
                        frame.fill_text(Text {
                            content: i.to_string() + &name.to_string(),
                            position: Point{
                                x,
                                y,
                            },
                            ..text
                        });
                    });
                    
                    //frame.stroke(&divide_circle(i,20, radius), thin_stroke(COLORS[i as usize % COLORS.len()]));
                }
            } 
            else{
                frame.fill(&Path::circle(Point::ORIGIN, radius), Color::BLACK);
            }
            
            });
            //Top
            frame.with_save(|frame|{
                let triangle_pointer =             Path::new(|p| {
                    p.move_to(Point::new(3.0,  -radius -15.0));
                    p.line_to(Point::new(-3.0,  -radius -15.0));
                    p.line_to(Point::new(0.0,  -radius/1.2 - 15.0));
                    p.line_to(Point::new(3.0,  -radius - 15.0));
                });
                frame.fill(&triangle_pointer, Color::BLACK);
            });
            /*
            //Right
            frame.with_save(|frame|{
                let triangle_pointer =             Path::new(|p| {
                    p.move_to(Point::new(radius + 15.0, 3.0 ));
                    p.line_to(Point::new(radius + 15.0,  -3.0));
                    p.line_to(Point::new(radius/1.2 + 15.0,  0.0));
                    p.line_to(Point::new(radius + 15.0, 3.0 ));
                });
                frame.fill(&triangle_pointer, Color::BLACK);
            });
            //Bottom
            frame.with_save(|frame|{
                let triangle_pointer =             Path::new(|p| {
                    p.move_to(Point::new(3.0,  radius + 15.0));
                    p.line_to(Point::new(-3.0,  radius + 15.0));
                    p.line_to(Point::new(0.0,  radius/1.2 + 15.0));
                    p.line_to(Point::new(3.0,  radius + 15.0));
                });
                frame.fill(&triangle_pointer, Color::BLACK);
            });
            //Left
            frame.with_save(|frame|{
                let triangle_pointer =             Path::new(|p| {
                    p.move_to(Point::new(-radius - 15.0, 3.0 ));
                    p.line_to(Point::new(-radius - 15.0,  -3.0));
                    p.line_to(Point::new(-radius/1.2 - 15.0,  0.0));
                    p.line_to(Point::new(-radius - 15.0, 3.0 ));
                });
                frame.fill(&triangle_pointer, Color::BLACK);
            });
            */
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
                if self.ideas_ref().len() == 0{
                    return;
                }
                self.progress = 0;
                self.state = AnimationState::Running;

                let (date ,i)  = self.pick_idea();
                println!("We picked {} at index: {}", date.name, i);
                //TODO: trixy ludo to set correct max progress
                // predicted + 360/self.date_ideas.len() * i
                self.started_rotation = self.rotation;
                let idea_size_rot = RouletteWheel::MAX_ROTATION as f32 / self.date_ideas.len() as f32;
                let num = rand::thread_rng().gen_range(0.0..idea_size_rot);
                let wanted_pos = RouletteWheel::MAX_ROTATION as f32/ self.date_ideas.len() as f32 * i as f32 + num;
                println!("wanted stop at {}", wanted_pos);
                //Prediction
                let rot_prog_ratio = RouletteWheel::ROTATION_SPEED/RouletteWheel::PROGRESS_STEP as f32;
                let end_rot = (RouletteWheel::MAX_ROTATION as f32 / 2.0) - (0.5 * RouletteWheel::PROGRESS_STEP as f32);

                self.max_progress = (wanted_pos - self.started_rotation) / rot_prog_ratio - end_rot + 2.0*RouletteWheel::MAX_ROTATION as f32;
                println!("max progress {}=({} {})/{} - {} + 360", self.max_progress, wanted_pos, self.started_rotation, rot_prog_ratio, end_rot);

            }
            _ => {}
        }
    }

    
    fn display_picked_date(&mut self) {
        self.state = AnimationState::Finished;
        //todo!()
    }

    fn progress_animation(&mut self, _progress: u32) {
        self.progress += RouletteWheel::PROGRESS_STEP;
        //self.rotation += ROTATION_SPEED;
        
        if (self.progress as f32) < (self.max_progress - RouletteWheel::MAX_ROTATION as f32) {
            self.rotation += RouletteWheel::ROTATION_SPEED;
        }
        else{
            let progress_left = (self.max_progress - self.progress as f32) / RouletteWheel::MAX_ROTATION as f32; // This moves us 175/progress_step * ROTATION:SPEED?
            self.rotation += RouletteWheel::ROTATION_SPEED * progress_left;
        }
        
        
        if self.rotation >= RouletteWheel::MAX_ROTATION as f32 {
            self.rotation = self.rotation - RouletteWheel::MAX_ROTATION as f32;
        }
        
        if self.max_progress <= self.progress as f32 {
            if self.rotation - self.rotation.floor() >= 0.1{
                self.rotation = self.rotation.ceil();
            }
            else{
                self.rotation = self.rotation.floor();
            }
            println!("Stopped at {} {}", self.rotation, self.progress);
            self.state = AnimationState::Finished;
        }
    }

    fn get_animation_progress(&self) -> u32 {
        return self.progress
    }

    fn get_animation_length(&self) -> u32 {
        //TODO: Random max so that it stops on the idea we want
        return self.max_progress.round() as u32;
    }
}


fn rotation(n: f32, total: f32) -> f32 {
    let turns = n / total;

    2.0 * std::f32::consts::PI * turns
}

fn divide_circle(n: f32, total: f32, radius: f32) -> Path{
    let angle_size = 1.0 / total * std::f32::consts::PI * 2.0;
    let y = radius * (angle_size * n as f32).cos();
    let x = radius * (angle_size * n as f32).sin();
     Path::new(|p| {
        p.move_to(Point::ORIGIN);
        p.line_to(Point::new(x,  y));
        p.arc(Arc {
            center: Point::ORIGIN,
            radius,
            start_angle: angle_size * n ,
            end_angle:   angle_size * (n- 1.0),
        });
        p.line_to(Point::ORIGIN);
    })
}

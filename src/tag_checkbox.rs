use iced::widget::{
    checkbox,  row,
};
use iced::{Element};
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagCheckbox {
    description: String,
    use_tag: bool,   
}

#[derive(Debug, Clone, Hash)]
pub enum TaskMessage {
    UseTag(bool),
}

impl TagCheckbox {

    pub fn new(description: String) -> Self {
        TagCheckbox {
            description,
            use_tag: false,
        }
    }

    pub fn get_name_ref(&self) -> String {
        return self.description.clone()
    }
    pub fn get_name(self) -> String {
        return self.description
    }

    pub fn in_use_ref(&self) -> bool {
        return self.use_tag
    }
    pub fn in_use(self) -> bool {
        return self.use_tag
    }

    pub fn update(&mut self, message: TaskMessage) {
        match message {
            TaskMessage::UseTag(use_tag) => {
                self.use_tag = use_tag;
                
                //TODO: Call to update the animation with the tags
                
            }
        }
    }
    pub fn view(&self) -> Element<TaskMessage> {

        let checkbox = checkbox(
            &self.description,
            self.use_tag,
            TaskMessage::UseTag,
        );

        row![
            checkbox
        ]
        .spacing(20)
        .align_items(iced::Alignment::Center)
        .into()

    }

}


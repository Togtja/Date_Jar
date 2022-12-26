use serde::{Deserialize};
use serde_yaml;

#[derive(Default,Deserialize, Debug)]
pub struct DateIdeas {
    pub ideas: Vec<DateIdea>,
}
#[derive(Deserialize, Debug, Clone, Hash, PartialEq)]
pub struct DateIdea{
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub group: String
    
}

pub fn get_data_idea(ref yaml_file: String) -> DateIdeas {
    match  std::fs::read_to_string(yaml_file) {
        Ok(file_data) => {
            println!("Read YAML string: {}", file_data);

            match serde_yaml::from_str::<DateIdeas>(&file_data){
                Ok(date_ideas) => {
                    return date_ideas;
                }
                Err(error) => {
                    println!("There is an error parsing {}: {}", file_data, error);
                }
            }
        }
        Err(error) => {
            println!("There is an error reading {}: {}", yaml_file, error);
        }
    }
    println!("Did we read?");
    DateIdeas::default()

}
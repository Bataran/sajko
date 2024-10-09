use serde::Deserialize;

// create a struct Factor wich has an array of structs called Facet
#[derive(Deserialize, Debug)]
pub struct Factor {
    pub name: String,
    pub facets: Vec<Facet>,
}

// // Factor has constructor with name and facets
// impl Factor {
//     pub fn new(name: &str, facets: Vec<Facet>) -> Factor {
//         Factor {
//             name: name.to_string(),
//             facets,
//         }
//     }

//     fn get_facet_value(&self, facet_name: &str) -> Option<String> {
//         for facet in &self.facets {
//             if facet.name == facet_name {
//                 return Some(facet.value.clone());
//             }
//         }
//         None
//     }
// }

// create a struct Facet with a name and a value
#[derive(Deserialize, Debug)]
pub struct Facet {
    pub name: String,
    pub questions: Vec<Question>,
}

// // Facet has constructor with name and value
// impl Facet {
//     pub fn new(name: &str, value: &str, questions: Vec<Question>) -> Facet {
//         Facet {
//             name: name.to_string(),
//             value: value.to_string(),
//             questions,
//         }
//     }
// }

pub trait GptInput {
    fn construct_gpt_input(&self) -> String;
}
impl GptInput for Factor {
    fn construct_gpt_input(&self) -> String {
        return format!("If {} is a personality dimension", self.name);
    }
}
impl GptInput for Facet {
    fn construct_gpt_input(&self) -> String {
        // let input: String = String::from("If there is a scale from 0 to 5 for questions where 5 means the the statement is correct and 0 that statement is incorrect, explain a person who gave answers on following questions.");
        let questions_input: String = self
            .questions
            .iter()
            .map(|q| format!("{}. {}: {}", q.ordinal_number, q.question, q.answer))
            .reduce(|mut acc, q| {
                acc.push_str(&format!("\n{}", q));
                acc
            })
            .unwrap();
        // TODO: summarize prompt for the current facet here
        return format!("{} {}", self.name, questions_input);
    }
}
// Facet implements the copy trait
// impl Clone for Facet {
//     fn clone(&self) -> Facet {
//         Facet {
//             name: self.name.clone(),
//             value: self.value.clone(),
//             questions: self.questions.clone(),
//         }
//     }
// }

#[derive(Deserialize, Debug)]
pub struct Question {
    pub question: String,
    pub ordinal_number: i32,
    pub answer: i32,
}

// add constructor for Question
impl Question {
    pub fn new(question: &str, ordinal_number: i32, answer: i32) -> Question {
        Question {
            question: question.to_string(),
            ordinal_number,
            answer,
        }
    }
}

// Question implemets clone trait
impl Clone for Question {
    fn clone(&self) -> Question {
        Question {
            question: self.question.clone(),
            ordinal_number: self.ordinal_number.clone(),
            answer: self.answer.clone(),
        }
    }
}

// // create a function to parse a json string into a Factor
// fn parse_factor(json: &str) -> Result<Factor, serde_json::Error> {
//     serde_json::from_str(json)
// }

// // create a function to parse a json string into a Vec<Factor>
// fn parse_factors(json: &str) -> Result<Vec<Factor>, serde_json::Error> {
//     serde_json::from_str(json)
// }

pub struct Questionaire {
    questions: Vec<Question1>,
}

impl Questionaire {
    pub fn new(questions: Vec<Question1>) -> Questionaire {
        Questionaire { questions }
    }
    pub fn get_neuroticism_questions(&self) -> Vec<&Question1> {
        return self
            .questions
            .iter()
            .filter(|q| q.code.starts_with("n"))
            .collect();
    }

    /// The function accepts factors code as following:
    /// Neuroticism: "n"
    /// Extraversion: "e"
    /// Agreeableness: "a"
    /// Conscientiousness: "c"
    /// Openness: "o"
    pub fn get_factor_questions(&self, factor: char) -> Vec<&Question1> {
        return self
            .questions
            .iter()
            .filter(|q| q.code.starts_with(factor))
            .collect();
    }
}

#[derive(Deserialize, Debug)]
pub struct Question1 {
    pub ordinal_number: i32,
    pub score: i32,
    pub answer: i32,
    pub difference: String,
    pub code: String,
    pub question: String,
}

// add constructor for Question
impl Question1 {
    pub fn new(input: (i32, i32, i32, String, String, String)) -> Question1 {
        Question1 {
            ordinal_number: input.0,
            score: input.1,
            answer: input.2,
            difference: input.3,
            code: input.4,
            question: input.5,
        }
    }
}

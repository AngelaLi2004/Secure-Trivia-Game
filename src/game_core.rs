use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use rand::seq::SliceRandom;

#[derive(Debug, Deserialize, Clone)]
pub struct Question {
    pub category: String,
    pub question: String,
    pub options: Vec<String>,
    pub answer: usize,
    pub points: i32,
    #[serde(rename = "hintCost")]
    pub hint_cost: i32,
    pub hint: Option<String>,
}

// Load all questions from the JSON file
pub fn load_questions() -> Vec<Question> {
    let file = File::open("data/questions.json").expect("Could not open questions file");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("Could not parse questions JSON")
}

// Get unique sorted list of categories
pub fn get_categories(questions: &[Question]) -> Vec<String> {
    let mut categories: Vec<String> = questions.iter()
        .map(|q| q.category.clone())
        .collect();
    categories.sort();
    categories.dedup();
    categories
}

// Pick a random question from a category
pub fn get_random_question<'a>(questions: &'a [Question], category: &str) -> Option<&'a Question> {
    let mut rng = rand::thread_rng();
    let filtered: Vec<&Question> = questions.iter()
        .filter(|q| q.category.eq_ignore_ascii_case(category))
        .collect();
    filtered.choose(&mut rng).copied()
}

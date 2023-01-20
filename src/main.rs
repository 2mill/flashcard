use std::fs::File;
use std::{fs, collections::HashMap, os::unix::thread};
use std::io;
use serde_json;
use serde_derive::{Deserialize};
use rand::{thread_rng, seq::SliceRandom};
fn main() {
}


enum FileType {
	YAML(String),
	JSON(String),
	CSV(String),
}

fn detect_filetype(file_name: &str) -> Result<FileType, String> {
	let split_file_name = file_name.split('.').last();
	match split_file_name {
		Some(suffix) => {
			match suffix {
				"yml" => Ok(FileType::YAML("TODO!".to_string())),
				"json" => Ok(FileType::JSON("TODO!".to_string())),
				"csv" => Ok(FileType::CSV("TODO!".to_string())),
				_ => Err("No parser available".to_string()),
			}
		},
		None => Err("Failed to detect filetype.".to_string())
	}
}


struct Deck<T> where T: Card {
	cards: Vec<T>, 
}


trait Card {
	fn validate(&self, answer: &String) -> bool;
	fn has_tag(&self, tag: &String) -> bool;
}

pub struct MultipleTrue {
	pub challenge: String,
	answer: Vec<String>,
	tag: Option<String>
}

impl Card for MultipleTrue {
	fn validate(&self, answer: &String) -> bool {
		for card_answer in self.answer.iter() {
			if card_answer.eq(answer) {
				return true;
			}
		}
		false
	}

	fn has_tag(&self, tag: &String) -> bool {
		match &self.tag {
			Some(defined_tag) => defined_tag.eq(tag),
			None => false
		}
	}
}

impl MultipleTrue {
	pub fn new(challenge: String, answer:Vec<String>, tag: Option<String>) -> MultipleTrue {
		MultipleTrue {
			challenge,
			answer,
			tag
		}
	}
}

// There can be any type of Card
// Vecs of items impl Card can be scheduled.

#[derive(Deserialize, Debug)]
pub struct SimpleCard {
	pub challenge: String,
	answer: String,
	tag: Option<String>
}

impl Card for SimpleCard {
	fn validate(&self, answer: &String) -> bool {
		self.answer.eq(answer)
	}
	fn has_tag(&self, tag: &String) -> bool {
		match &self.tag {
			Some(defined_tag) => defined_tag.eq(tag),
			None => false
		}
	}
}

impl SimpleCard {
	fn new(challenge: String, answer: String, tag: Option<String>) -> SimpleCard {
		SimpleCard {
			challenge,
			answer,
			tag
		}
	}

}


// This is essentially the sorting algorithm that is used
// for what order the cards should be presented in.
trait Schedule<T> where T: Card {
	fn schedule(cards: Vec<&T>) -> Vec<&T>;
}

struct InOrder {}
impl<T> Schedule<T> for InOrder where T: Card {
	fn schedule(cards: Vec<&T>) -> Vec<&T> {
		cards
	}
}

struct RandomSchedule { }
impl<T> Schedule<T> for RandomSchedule where T: Card {
	fn schedule(mut cards: Vec<&T>) -> Vec<&T> {
		let mut rng = thread_rng();
		cards.shuffle(&mut rng);
		cards
	}
}



#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn build_and_validate_simple_card() {
		let simple_card = SimpleCard::new(
			"What is the positive square root of 4".to_string(),
			"2".to_string(),
			None
		);

		assert!(simple_card.validate(&"2".to_string()))
	}
	#[test]
	fn build_and_validate_multiple_true_card() {
		let card = MultipleTrue::new(
			"What numbers comprise the set of the square root of 4".to_string(),
			vec!("-2".to_string(), "2".to_string(), "+2".to_string()),
			None
		);

		assert!(card.validate(&"2".to_string()))
	}

	#[test]
	fn test_detect_filetype() {
		let filenames = [
			"foobar.csv",
			"foobar.yml",
			"foobar.json",
			"foobar.json.json",
			"foobar.json.yml",
		];

		for filename in filenames {
			match detect_filetype(filename) {
				Ok(_) => assert!(true),
				_ => assert!(false)
			}
		}
	}

	#[test]
	fn deserialize_json_simple_card() {
		let json_card = r#"
			{
				"challenge": "What is the positive square root of 4",
				"answer": "2"
			}
		"#;
		let card: Result<SimpleCard, _> = serde_json::from_str(json_card);
		match card {
			Ok(_) => assert!(true),
			_ => assert!(false)
		}
	}

}
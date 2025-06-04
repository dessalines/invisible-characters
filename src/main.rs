use anyhow::Result;
use invisible_characters::convert_code_to_unicode_char;
use serde::Deserialize;
use std::{fs::File, io::Write};

const OUTPUT_FILE: &str = "src/invisible_chars.rs";
const INPUT_CHARS_FILE: &str = "invisible-characters/characters.json";

fn main() -> Result<()> {
  let data = read_chars()?;
  let mut file = File::create(OUTPUT_FILE)?;

  let var_line = format!("pub const INVISIBLE_CHARS: [char; {}] = ", data.len());

  let converted_chars = data
    .iter()
    .map(|c| convert_code_to_unicode_char(&c.code))
    .collect::<Vec<char>>();

  let output_data = format!("{var_line}{converted_chars:#?};");
  write!(file, "{output_data}")?;

  Ok(())
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct InvisibleCharReader {
  code: String,
  name: String,
  description: String,
}

fn read_chars() -> Result<Vec<InvisibleCharReader>> {
  let file = File::open(INPUT_CHARS_FILE)?;
  let data: Vec<InvisibleCharReader> = serde_json::from_reader(file)?;
  Ok(data)
}

use anyhow::Result;
use serde::Deserialize;
use std::{fs::File, io::Write};

const OUTPUT_FILE: &str = "src/invisible_chars.rs";
const INPUT_CHARS_FILE: &str = "invisible-characters/characters.json";

fn main() -> Result<()> {
  let data = read_chars()?;
  let mut file = File::create(OUTPUT_FILE)?;

  let import_line = "use crate::InvisibleChar;";
  let var_line = format!(
    "pub const INVISIBLE_CHARS: [InvisibleCharReader; {}] = ",
    data.len()
  );

  let output_data = format!("{import_line}\n{var_line}{:#?};", data)
    .replace("InvisibleCharReader", "InvisibleChar");
  write!(file, "{}", output_data)?;

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

use serde::{Deserialize, Serialize};
pub mod invisible_chars;
pub use invisible_chars::INVISIBLE_CHARS;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InvisibleChar {
  code: &'static str,
  name: &'static str,
  description: &'static str,
}

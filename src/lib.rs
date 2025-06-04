pub mod invisible_chars;
pub use invisible_chars::INVISIBLE_CHARS;

pub fn convert_code_to_unicode_char(code: &str) -> char {
  let hex = u32::from_str_radix(code, 16).unwrap_or_default();
  char::from_u32(hex).unwrap_or_default()
}

#[cfg(test)]
mod test {
  use crate::INVISIBLE_CHARS;
  use anyhow::Result;

  #[test]
  fn code_convert() -> Result<()> {
    let test_code = "000A";
    let hex = u32::from_str_radix(test_code, 16)?;
    let converted = char::from_u32(hex).unwrap_or('a');
    assert_eq!('\u{000A}', converted);
    Ok(())
  }

  #[test]
  fn first_and_last() -> Result<()> {
    assert_eq!('\u{0009}', *INVISIBLE_CHARS.first().unwrap());
    assert_eq!('\u{e0020}', *INVISIBLE_CHARS.last().unwrap());
    Ok(())
  }

  #[test]
  fn flag_or_emoji_fail() -> Result<()> {
    let has_flag_and_emoji = "hello👋🇦🇴";
    assert!(!has_flag_and_emoji.contains(INVISIBLE_CHARS));
    Ok(())
  }
}

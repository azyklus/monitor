use std::fmt;

/// Returns a formatted string.
pub fn auto_to_string<T: Copy>(value: T) -> String
   where
      T: fmt::Display,
{
   format!("{}", value)
}

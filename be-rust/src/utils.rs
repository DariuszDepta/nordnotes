/*
 * nordnotes
 *
 * MIT license
 *
 * Copyright (c) 2022 Dariusz Depta Engos Software
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

//! Utilities.

use time::macros::format_description;
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

/// Generates a new UUID.
pub fn uuid() -> String {
  Uuid::new_v4().to_string()
}

/// Returns a tuple of current UTC date and time, and optional expiration date and time,
/// both formatted as strings. Expiration date and time is calculated basing on time to live marker.
pub fn create_and_expiration_date_time(ttl: &str) -> (String, Option<String>) {
  let format = format_description!("[year]-[month]-[day]T[hour]:[minute]:00");
  let created_at = OffsetDateTime::now_utc();
  if let Some(minutes) = ttl_to_minutes(ttl) {
    let expires_at = created_at + Duration::minutes(minutes);
    (created_at.format(&format).unwrap(), Some(expires_at.format(&format).unwrap()))
  } else {
    (created_at.format(&format).unwrap(), None)
  }
}

/// Converts time to live marker into minutes.
/// Time to live has the format: `Nw`, `Nd`, `Nh` or `Nm`, where `N` is an integer
/// and letters have the following meaning: `w` - weeks, `d` - days, `h` - hours, `m` - minutes.
/// For example `ttl` == "10d" means 14400 minutes.
fn ttl_to_minutes(ttl: &str) -> Option<i64> {
  // ttl must have minimum two characters
  if ttl.len() >= 2 {
    // the last character must be a time marker
    if let Some(time_marker) = ttl.chars().last() {
      // character before time marker must form an integer number
      let num_str = &ttl[..ttl.len() - 1];
      if let Ok(num) = num_str.parse::<i64>() {
        return match time_marker {
          'w' => Some(num * 7 * 24 * 60),
          'd' => Some(num * 24 * 60),
          'h' => Some(num * 60),
          'm' => Some(num),
          _ => None,
        };
      }
    }
  }
  None
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_uuid() {
    assert_eq!(36, uuid().len());
  }

  #[test]
  fn test_create_and_expiration_date_time() {
    let now = OffsetDateTime::now_utc();
    let created_at = format!(
      "{:04}-{:02}-{:02}T{:02}:{:02}:00",
      now.year(),
      now.month() as u8,
      now.day(),
      now.hour(),
      now.minute()
    );
    let expires_at = format!(
      "{:04}-{:02}-{:02}T{:02}:{:02}:00",
      now.year(),
      now.month() as u8,
      now.day(),
      now.hour() + 2,
      now.minute()
    );
    assert_eq!((created_at, Some(expires_at)), create_and_expiration_date_time("2h"))
  }

  #[test]
  fn test_ttl_to_minutes() {
    assert_eq!(2 * 7 * 24 * 60, ttl_to_minutes("2w").unwrap());
    assert_eq!(18 * 24 * 60, ttl_to_minutes("18d").unwrap());
    assert_eq!(56 * 60, ttl_to_minutes("56h").unwrap());
    assert_eq!(276, ttl_to_minutes("276m").unwrap());
    assert_eq!(1, ttl_to_minutes("1m").unwrap());
    assert_eq!(None, ttl_to_minutes("1y"));
    assert_eq!(None, ttl_to_minutes("1"));
    assert_eq!(None, ttl_to_minutes(""));
  }
}

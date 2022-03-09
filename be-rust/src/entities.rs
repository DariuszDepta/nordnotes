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

//! Implementation of database entities.

use crate::utils::{create_and_expiration_date_time, uuid};
use time::macros::format_description;
use time::{OffsetDateTime, PrimitiveDateTime};

/// Note entity.
#[derive(Clone)]
pub struct NoteEntity {
  /// Unique note identifier.
  pub note_id: String,
  /// Title of the note.
  pub title: String,
  /// Content of the note.
  pub content: String,
  /// Date and time when the note was created,
  /// in format `YYYY-MM-DD hh:mm:ss`.
  pub created_at: String,
  /// Date and time when the note expires,
  /// in format `YYYY-MM-DD hh:mm:ss`.
  pub expires_at: Option<String>,
}

impl NoteEntity {
  /// Creates a new note entity with title, content and expiration time.
  pub fn new(title: &str, content: &str, ttl: &str) -> Self {
    let (created_at, expires_at) = create_and_expiration_date_time(ttl);
    Self {
      note_id: uuid(),
      title: title.to_string(),
      content: content.to_string(),
      created_at,
      expires_at,
    }
  }
  /// Returns `true` when the note has expired.
  pub fn has_expired(&self) -> bool {
    let format = format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]");
    if let Some(date_time_str) = &self.expires_at {
      if let Ok(expires_at) = PrimitiveDateTime::parse(date_time_str, format) {
        return expires_at.assume_utc() < OffsetDateTime::now_utc();
      }
    }
    false
  }
}

/// User entity.
#[derive(Clone)]
pub struct UserEntity {
  /// Unique user identifier.
  pub user_id: String,
  /// User login.
  pub login: String,
  /// User password.
  pub password: String,
  /// Token generated for the user.
  pub token: Option<String>,
}

impl UserEntity {
  /// Creates a new user entity.
  pub fn new(login: &str, password: &str) -> Self {
    Self {
      user_id: uuid(),
      login: login.to_string(),
      password: password.to_string(),
      token: None,
    }
  }
}

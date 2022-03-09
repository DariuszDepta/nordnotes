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

//! Implementation of storage access.

use crate::notes::NoteEntity;
use crate::users::UserEntity;
use uuid::Uuid;

const NOTES_LIMIT: usize = 20;

/// Shared application data.
#[derive(Clone)]
pub struct Storage {
  /// Collection of notes.
  pub notes: Vec<NoteEntity>,
  /// Collection of users.
  pub users: Vec<UserEntity>,
}

impl Storage {
  /// Initializes the storage.
  pub fn new() -> Self {
    Self {
      notes: load_notes(),
      users: load_users(),
    }
  }
  /// Deletes all notes.
  pub fn delete_notes(&mut self) {
    self.notes.clear();
  }
  /// Returns a list of notes that has not expired yet.
  pub fn get_notes(&self) -> Vec<NoteEntity> {
    self.notes.iter().filter(|note| !note.has_expired()).cloned().collect()
  }
  /// Returns a note with specified identifier.
  pub fn get_note(&self, id: &str) -> Option<NoteEntity> {
    for note in &self.notes {
      if note.note_id == id && !note.has_expired() {
        return Some(note.clone());
      }
    }
    None
  }
  /// Creates a new note, returns the identifier of newly created note.
  pub fn create_note(&mut self, title: &str, content: &str, ttl: &str) -> Option<String> {
    // remove first note when the limit was reached
    if self.notes.len() >= NOTES_LIMIT {
      self.notes.remove(0);
    }
    let note = NoteEntity::new(title, content, ttl);
    let note_id = note.note_id.clone();
    self.notes.push(note);
    Some(note_id)
  }
  /// Generates a new token for a user when login and password are correct.
  pub fn get_token(&mut self, login: &str, password: &str) -> Option<String> {
    let token = self.get_uuid();
    for user in &mut self.users {
      if user.login == login && user.password == password {
        user.token = Some(token.clone());
        return Some(token);
      }
    }
    None
  }
  ///
  pub fn check_token(&self, token: &str) -> bool {
    for user in &self.users {
      if user.token == Some(token.to_string()) {
        return true;
      }
    }
    false
  }
  /// Generates new UUID.
  fn get_uuid(&self) -> String {
    let uuid = Uuid::new_v4();
    uuid.to_string()
  }
}

/// Returns few initial notes.
pub fn load_notes() -> Vec<NoteEntity> {
  vec![
    NoteEntity::new("Note 1", "My first note", "1m"),
    NoteEntity::new("Note 2", "My second note", "3m"),
    NoteEntity::new("Note 3", "My third note", "5m"),
    NoteEntity::new("Note 4", "My fourth note", "10m"),
  ]
}

/// Loads user names and passwords from file.
pub fn load_users() -> Vec<UserEntity> {
  let mut users = vec![];
  if let Ok(content) = std::fs::read_to_string("users") {
    for line in content.lines() {
      let mut split = line.split(':');
      if let Some((login, password)) = split.next().zip(split.next()) {
        users.push(UserEntity::new(login, password));
      }
    }
  }
  users
}

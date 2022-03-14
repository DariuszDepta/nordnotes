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

use crate::entities::note::NoteEntity;
use crate::entities::user::UserEntity;
use crate::errors::*;
use crate::repositories::notes::NotesRepository;
use crate::repositories::roles::RolesRepository;
use crate::utils::uuid;
use lazy_static::lazy_static;
use scylla::transport::session::PoolSize;
use scylla::{Session, SessionBuilder};
use std::env;
use std::num::NonZeroUsize;
use std::sync::Arc;

/// Name of the keyspace.
pub const KEYSPACE: &str = "nordnotes";

/// Name of the table with notes.
pub const TABLE_NOTES: &str = "notes";

/// Name of the table with roles.
pub const TABLE_ROLES: &str = "roles";

/// Name of the table with users.
pub const TABLE_USERS: &str = "users";

lazy_static! {
  static ref QUERY_CREATE_KEYSPACE: String = format!(
    "CREATE KEYSPACE IF NOT EXISTS {} WITH REPLICATION = {{'class' : 'SimpleStrategy', 'replication_factor' : 1}}",
    KEYSPACE
  );
  static ref QUERY_CREATE_TABLE_NOTES: String = format!(
    "CREATE TABLE IF NOT EXISTS {}.{} (note_id text, title text, content text, created_at text, expires_at text, primary key (note_id))",
    KEYSPACE, TABLE_NOTES
  );
  static ref QUERY_CREATE_TABLE_ROLES: String = format!(
    "CREATE TABLE IF NOT EXISTS {}.{} (role_id text, name text, primary key (role_id))",
    KEYSPACE, TABLE_ROLES
  );
  static ref QUERY_CREATE_TABLE_USERS: String = format!(
    "CREATE TABLE IF NOT EXISTS {}.{} (user_id text, login text, password text, token text, primary key (user_id))",
    KEYSPACE, TABLE_USERS
  );
}

/// Shared application data.
pub struct Storage {
  /// ScyllaDB session.
  session: Arc<Session>,
  /// Roles repository.
  pub roles_repository: RolesRepository,
  /// Notes repository.
  notes_repository: NotesRepository,
  /// Collection of users.
  pub users: Vec<UserEntity>,
}

impl Storage {
  /// Initializes the storage.
  pub async fn new() -> Result<Self> {
    let uri = env::var("SCYLLA_URI").unwrap_or_else(|_| "127.0.0.1:9042".to_string());
    // connect to database
    let session = Arc::new(
      SessionBuilder::new()
        .known_node(&uri)
        .pool_size(PoolSize::PerHost(NonZeroUsize::new(4).unwrap()))
        .build()
        .await
        .map_err(err_new_session)?,
    );
    // initialize database structure
    session.query(QUERY_CREATE_KEYSPACE.as_str(), &[]).await.map_err(err_query)?;
    session.query(QUERY_CREATE_TABLE_NOTES.as_str(), &[]).await.map_err(err_query)?;
    session.query(QUERY_CREATE_TABLE_ROLES.as_str(), &[]).await.map_err(err_query)?;
    println!("database initialized");
    // initialize repositories
    let roles_repository = RolesRepository::new(Arc::clone(&session));
    Ok(Self {
      session,
      roles_repository,
      notes_repository: NotesRepository,
      users: load_users(),
    })
  }
  /// Creates a new note, returns the identifier of newly created note.
  pub async fn create_note(&mut self, title: &str, content: &str, ttl: &str) -> Result<String> {
    let note = NoteEntity::new(title, content, ttl);
    self.notes_repository.add(&self.session, note).await
  }
  /// Returns a list of notes that has not expired yet.
  pub async fn get_notes(&self) -> Result<Vec<NoteEntity>> {
    self.notes_repository.list(&self.session).await
  }
  /// Returns a note with specified identifier.
  pub async fn get_note(&self, id: &str) -> Result<NoteEntity> {
    self.notes_repository.find(&self.session, id).await
  }
  /// Deletes all notes.
  pub async fn delete_notes(&mut self) -> Result<()> {
    self.notes_repository.delete_all(&self.session).await
  }
  /// Generates a new token for a user when login and password are correct.
  pub fn get_token(&mut self, login: &str, password: &str) -> Option<String> {
    let token = uuid();
    for user in &mut self.users {
      if user.login == login && user.password == password {
        user.token = Some(token.clone());
        return Some(token);
      }
    }
    None
  }
  /// Checks if the token exists for any user.
  pub fn check_token(&self, token: &str) -> bool {
    for user in &self.users {
      if user.token == Some(token.to_string()) {
        return true;
      }
    }
    false
  }
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_queries() {
    assert_eq!(
      "CREATE KEYSPACE IF NOT EXISTS nordnotes WITH REPLICATION = {'class' : 'SimpleStrategy', 'replication_factor' : 1}",
      QUERY_CREATE_KEYSPACE.as_str()
    )
  }
}

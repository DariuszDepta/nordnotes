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

//! Implementation of database repositories.

use crate::entities::NoteEntity;
use crate::errors::*;
use crate::storage::{KEYSPACE, TABLE_NOTES};
use lazy_static::lazy_static;
use scylla::{IntoTypedRows, Session, ValueList};

lazy_static! {
  static ref QUERY_INSERT_NOTE: String = format!(
    "INSERT INTO {}.{} (note_id, title, content, created_at, expires_at) VALUES (?, ?, ?, ?, ?)",
    KEYSPACE, TABLE_NOTES
  );
  static ref QUERY_DELETE_ALL_NOTES: String = format!("TRUNCATE TABLE {}.{}", KEYSPACE, TABLE_NOTES);
  static ref QUERY_LIST_NOTES: String = format!("SELECT note_id, title, content, created_at, expires_at FROM {}.{}", KEYSPACE, TABLE_NOTES);
  static ref QUERY_FIND_NOTE: String = format!(
    "SELECT note_id, title, content, created_at, expires_at FROM {}.{} WHERE note_id = ?",
    KEYSPACE, TABLE_NOTES
  );
}

/// Value list containing the note's identifier.
#[derive(ValueList)]
struct NoteId {
  note_id: String,
}

/// Repository for data manipulation on notes.
pub struct NotesRepository;

impl NotesRepository {
  /// Deletes all notes.
  pub async fn delete_all(&self, session: &Session) -> Result<()> {
    session.query(QUERY_DELETE_ALL_NOTES.as_str(), &[]).await.map_err(err_query)?;
    Ok(())
  }
  /// Adds a new note.
  pub async fn add(&self, session: &Session, note: NoteEntity) -> Result<String> {
    let note_id = note.note_id.clone();
    let values = (note.note_id, note.title, note.content, note.created_at, note.expires_at);
    session.query(QUERY_INSERT_NOTE.as_str(), values).await.map_err(err_query)?;
    Ok(note_id)
  }
  /// Lists all notes.
  pub async fn list(&self, session: &Session) -> Result<Vec<NoteEntity>> {
    let mut notes = vec![];
    if let Some(rows) = session.query(QUERY_LIST_NOTES.as_str(), &[]).await.map_err(err_query)?.rows {
      for row in rows.into_typed::<NoteEntity>() {
        let note = row.map_err(err_from_row)?;
        if !note.has_expired() {
          notes.push(note);
        }
      }
    }
    Ok(notes)
  }
  /// Searches for a note with specified identified.
  pub async fn find(&self, session: &Session, note_id: &str) -> Result<NoteEntity> {
    let id = NoteId { note_id: note_id.to_string() };
    if let Some(rows) = session.query(QUERY_FIND_NOTE.as_str(), id).await.map_err(err_query)?.rows {
      if let Some(row) = rows.into_typed::<NoteEntity>().take(1).next() {
        return row.map_err(err_from_row);
      }
    }
    Err(err_note_not_found(note_id))
  }
}

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

//! Implementation of controllers for notes.

use crate::errors::*;
use crate::handlers::notes::{CreateNoteParams, NoteDto};
use crate::storage::Storage;

/// Controller for deleting all notes.
pub async fn delete_all(storage: &mut Storage) -> Result<String> {
  storage.delete_notes().await?;
  Ok("all notes deleted".to_string())
}

/// Controller for retrieving a list of notes.
pub async fn list(storage: &Storage) -> Result<Vec<NoteDto>> {
  Ok(
    storage
      .get_notes()
      .await?
      .iter()
      .map(|note| NoteDto {
        note_id: note.note_id.clone(),
        title: Some(note.title.clone()),
        content: None,
      })
      .collect(),
  )
}

/// Controller for retrieving a single note.
pub async fn get_by_id(note_id: String, storage: &Storage) -> Result<NoteDto> {
  let note = storage.get_note(&note_id).await?;
  Ok(note.into())
}

/// Controller for creating a new note.
pub async fn create(params: CreateNoteParams, storage: &mut Storage) -> Result<NoteDto> {
  let (title, content, ttl) = params.validate()?;
  if let Ok(note_id) = storage.create_note(&title, &content, &ttl).await {
    Ok(NoteDto { note_id, ..NoteDto::default() })
  } else {
    Err(err_creating_note_failed())
  }
}

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

//! Implementation of controllers called by request handlers.
//!
//! Controller responsibilities:
//! - take request parameters as input values.
//! - validate input parameters (when needed),
//! - convert parameters into input values processed by services,
//! - orchestrate service invocation that business logic is executed,
//! - gather output values,
//! - convert output values into DTOs.
//! - return DTOs (or collections of DTOs) as a result of processing.

use crate::dto::{LoginDto, NoteDto};
use crate::errors::*;
use crate::params::{CreateNoteParams, LoginParams};
use crate::storage::Storage;

/// Controller for deleting all notes.
pub async fn controller_delete_notes(storage: &mut Storage) -> Result<String> {
  storage.delete_notes().await?;
  Ok("all notes deleted".to_string())
}

/// Controller for retrieving a list of notes.
pub async fn controller_list_notes(storage: &Storage) -> Result<Vec<NoteDto>> {
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
pub async fn controller_get_note(note_id: String, storage: &Storage) -> Result<NoteDto> {
  let note = storage.get_note(&note_id).await?;
  Ok(note.into())
}

/// Controller for creating a new note.
pub async fn controller_create_note(params: CreateNoteParams, storage: &mut Storage) -> Result<NoteDto> {
  let (title, content, ttl) = params.validate()?;
  if let Ok(note_id) = storage.create_note(&title, &content, &ttl).await {
    Ok(NoteDto { note_id, ..NoteDto::default() })
  } else {
    Err(err_creating_note_failed())
  }
}

/// Controller for user login.
pub async fn controller_login(params: LoginParams, storage: &mut Storage) -> Result<LoginDto> {
  let (login, password) = params.validate()?;
  if let Some(token) = storage.get_token(&login, &password) {
    Ok(LoginDto { token })
  } else {
    Err(err_invalid_login_or_password())
  }
}

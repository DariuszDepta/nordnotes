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

//! Implementation of controllers called by web server handlers.

use crate::dto::{LoginDto, NoteDto};
use crate::errors::*;
use crate::params::{CreateNoteParams, LoginParams};
use crate::storage::Storage;

/// Controller for deleting all notes.
pub async fn controller_delete_notes(storage: &mut Storage) -> Result<String> {
  storage.delete_notes();
  Ok("all notes deleted".to_string())
}

/// Controller for retrieving a list of notes.
pub async fn controller_list_notes(storage: &Storage) -> Result<Vec<NoteDto>> {
  Ok(
    storage
      .get_notes()
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
pub async fn controller_get_note(note_id: &str, storage: &Storage) -> Result<NoteDto> {
  if let Some(note) = storage.get_note(&note_id) {
    Ok(note.into())
  } else {
    Err(err_note_not_found(note_id))
  }
}

/// Controller for creating a new note.
pub async fn controller_create_note(params: &CreateNoteParams, storage: &mut Storage) -> Result<NoteDto> {
  if let Some(title) = &params.title {
    if let Some(content) = &params.content {
      let ttl = params.ttl.as_ref().unwrap_or(&"".to_string()).to_owned();
      if let Some(id) = storage.create_note(title, content, &ttl) {
        Ok(NoteDto {
          note_id: id,
          ..NoteDto::default()
        })
      } else {
        Err(err_creating_note_failed())
      }
    } else {
      Err(err_required_attribute_not_specified("content"))
    }
  } else {
    Err(err_required_attribute_not_specified("title"))
  }
}

/// Controller for user login.
pub async fn controller_login(params: &LoginParams, storage: &mut Storage) -> Result<LoginDto> {
  if let Some(login) = &params.login {
    if let Some(password) = &params.password {
      if let Some(token) = storage.get_token(login, password) {
        Ok(LoginDto { token })
      } else {
        Err(err_invalid_login_or_password())
      }
    } else {
      Err(err_required_attribute_not_specified("password"))
    }
  } else {
    Err(err_required_attribute_not_specified("login"))
  }
}

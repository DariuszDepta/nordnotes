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

use crate::errors::NordNotesError;
use crate::notes::Note;
use serde::Serialize;

/// Data transfer object for an error.
#[derive(Serialize)]
pub struct ErrorDto {
  /// Error details.
  #[serde(rename = "details")]
  details: String,
}

/// Data transfer object for the result (valid data or error).
#[derive(Serialize)]
pub struct ResultDto<T> {
  /// Result containing data.
  #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
  data: Option<T>,
  /// Result containing errors.
  #[serde(rename = "errors", skip_serializing_if = "Vec::is_empty")]
  errors: Vec<ErrorDto>,
}

impl<T> Default for ResultDto<T> {
  /// Creates default result structure.
  fn default() -> Self {
    Self { data: None, errors: vec![] }
  }
}

impl<T> ResultDto<T> {
  /// Creates [ResultDto] with some data inside.
  pub fn data(d: T) -> ResultDto<T> {
    ResultDto {
      data: Some(d),
      ..Default::default()
    }
  }
  /// Creates [ResultDto] with single error inside.
  pub fn error(err: NordNotesError) -> ResultDto<T> {
    ResultDto {
      errors: vec![ErrorDto { details: format!("{}", err) }],
      ..Default::default()
    }
  }
}

/// Data transfer object for a note.
#[derive(Default, Serialize)]
pub struct NoteDto {
  /// Unique note identifier.
  #[serde(rename = "noteId")]
  pub note_id: String,
  /// Title of the note.
  #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
  pub title: Option<String>,
  /// Content of the note (optional).
  #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
  pub content: Option<String>,
}

impl From<Note> for NoteDto {
  /// Converts a [Note] into [NoteDto].
  fn from(note: Note) -> Self {
    Self::from(&note)
  }
}

impl From<&Note> for NoteDto {
  /// Converts a reference to [Note] into [NoteDto].
  fn from(note: &Note) -> Self {
    Self {
      note_id: note.note_id.clone(),
      title: Some(note.title.clone()),
      content: Some(note.content.clone()),
    }
  }
}

/// Data transfer object for login result.
#[derive(Serialize)]
pub struct LoginDto {
  #[serde(rename = "token")]
  pub token: String,
}

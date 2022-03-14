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

use crate::controllers::notes;
use crate::entities::note::NoteEntity;
use crate::errors::*;
use crate::handlers::is_authorized;
use crate::server::{ApplicationData, ResultDto};
use actix_web::web::{Json, Path};
use actix_web::{delete, get, post, web, HttpRequest};
use serde_derive::{Deserialize, Serialize};

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

impl From<NoteEntity> for NoteDto {
  /// Converts a [NoteEntity] into [NoteDto].
  fn from(note: NoteEntity) -> Self {
    Self::from(&note)
  }
}

impl From<&NoteEntity> for NoteDto {
  /// Converts a reference to [NoteEntity] into [NoteDto].
  fn from(note: &NoteEntity) -> Self {
    Self {
      note_id: note.note_id.clone(),
      title: Some(note.title.clone()),
      content: Some(note.content.clone()),
    }
  }
}

/// Parameters needed when a new note is created.
#[derive(Deserialize)]
pub struct CreateNoteParams {
  /// The title of a note.
  #[serde(rename = "title")]
  pub title: Option<String>,
  /// The content of a note.
  #[serde(rename = "content")]
  pub content: Option<String>,
  /// Time to live, defines how long the content of a note will be available.
  /// `ttl` has the format: `Nw`, `Nd`, `Nh` or `Nm`, where `N` is an integer
  /// and letters have following meaning: `w` - weeks, `d` - days, `h` - hours, `m` - minutes.
  /// For example `ttl` == "10d" means that the note will expire after 10 days from creation.
  #[serde(rename = "ttl")]
  pub ttl: Option<String>,
}

impl CreateNoteParams {
  /// Validates required parameters for creating a new note.
  pub fn validate(self) -> Result<(String, String, String)> {
    if let Some(title) = self.title {
      if let Some(content) = self.content {
        Ok((title, content, self.ttl.unwrap_or_else(|| "".to_string())))
      } else {
        Err(err_required_attribute_not_specified("content"))
      }
    } else {
      Err(err_required_attribute_not_specified("title"))
    }
  }
}

/// Handler for creating a new note.
#[post("/api/v1/notes")]
pub async fn create(req: HttpRequest, params: Json<CreateNoteParams>, data: web::Data<ApplicationData>) -> std::io::Result<Json<ResultDto<NoteDto>>> {
  let mut storage = data.storage.write().await;
  if is_authorized(&req, &storage) {
    match notes::create(params.into_inner(), &mut storage).await {
      Ok(result) => Ok(Json(ResultDto::data(result))),
      Err(reason) => Ok(Json(ResultDto::error(reason))),
    }
  } else {
    Ok(Json(ResultDto::error(err_not_authorized())))
  }
}

/// Handler for retrieving a list of notes.
#[get("/api/v1/notes")]
pub async fn list(data: web::Data<ApplicationData>) -> std::io::Result<Json<ResultDto<Vec<NoteDto>>>> {
  let storage = data.storage.read().await;
  match notes::list(&storage).await {
    Ok(result) => Ok(Json(ResultDto::data(result))),
    Err(reason) => Ok(Json(ResultDto::error(reason))),
  }
}

/// Handler for retrieving the details of a single note identified by unique identifier.
#[get("/api/v1/notes/{id}")]
pub async fn get_by_id(req: HttpRequest, id: Path<String>, data: web::Data<ApplicationData>) -> std::io::Result<Json<ResultDto<NoteDto>>> {
  let storage = data.storage.read().await;
  if is_authorized(&req, &storage) {
    match notes::get_by_id(id.into_inner(), &storage).await {
      Ok(result) => Ok(Json(ResultDto::data(result))),
      Err(reason) => Ok(Json(ResultDto::error(reason))),
    }
  } else {
    Ok(Json(ResultDto::error(err_not_authorized())))
  }
}

/// Handler for deleting all notes.
#[delete("/api/v1/notes")]
pub async fn delete_all(data: web::Data<ApplicationData>) -> std::io::Result<Json<ResultDto<String>>> {
  let mut storage = data.storage.write().await;
  match notes::delete_all(&mut storage).await {
    Ok(result) => Ok(Json(ResultDto::data(result))),
    Err(reason) => Ok(Json(ResultDto::error(reason))),
  }
}

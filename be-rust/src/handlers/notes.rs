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

use crate::controllers::{controller_create_note, controller_delete_notes, controller_get_note, controller_list_notes};
use crate::dto::{NoteDto, ResultDto};
use crate::errors::*;
use crate::handlers::is_authorized;
use crate::params::CreateNoteParams;
use crate::server::ApplicationData;
use actix_web::web::{Json, Path};
use actix_web::{delete, get, post, web, HttpRequest};

/// Handler for creating a new note.
#[post("/api/v1/notes")]
pub async fn create(req: HttpRequest, params: Json<CreateNoteParams>, data: web::Data<ApplicationData>) -> std::io::Result<Json<ResultDto<NoteDto>>> {
  let mut storage = data.storage.write().await;
  if is_authorized(&req, &storage) {
    match controller_create_note(params.into_inner(), &mut storage).await {
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
  match controller_list_notes(&storage).await {
    Ok(result) => Ok(Json(ResultDto::data(result))),
    Err(reason) => Ok(Json(ResultDto::error(reason))),
  }
}

/// Handler for retrieving the details of a single note identified by unique identifier.
#[get("/api/v1/notes/{id}")]
pub async fn get_by_id(req: HttpRequest, id: Path<String>, data: web::Data<ApplicationData>) -> std::io::Result<Json<ResultDto<NoteDto>>> {
  let storage = data.storage.read().await;
  if is_authorized(&req, &storage) {
    match controller_get_note(id.into_inner(), &storage).await {
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
  match controller_delete_notes(&mut storage).await {
    Ok(result) => Ok(Json(ResultDto::data(result))),
    Err(reason) => Ok(Json(ResultDto::error(reason))),
  }
}

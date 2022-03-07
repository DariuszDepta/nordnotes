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

use crate::dto::{LoginDto, NoteDto, ResultDto};
use crate::errors::*;
use crate::params::{CreateNoteParams, LoginParams};
use crate::storage::Storage;
use actix_cors::Cors;
use actix_web::web::{Json, Path};
use actix_web::{delete, get, post, web, App, HttpRequest, HttpServer};
use std::sync::Mutex;

struct ApplicationData {
  storage: Mutex<Storage>,
}

/// Handler for retrieving the list of notes.
#[delete("/api/v1/notes")]
async fn handler_delete_notes(data: web::Data<ApplicationData>) -> std::io::Result<Json<ResultDto<Vec<NoteDto>>>> {
  if let Ok(mut storage) = data.storage.lock() {
    storage.delete_notes();
    Ok(Json(ResultDto::data(get_notes(&storage))))
  } else {
    Ok(Json(ResultDto::error(err_no_access_to_storage())))
  }
}

/// Handler for retrieving the list of notes.
#[get("/api/v1/notes")]
async fn handler_get_notes(data: web::Data<ApplicationData>) -> std::io::Result<Json<ResultDto<Vec<NoteDto>>>> {
  if let Ok(storage) = data.storage.lock() {
    Ok(Json(ResultDto::data(get_notes(&storage))))
  } else {
    Ok(Json(ResultDto::error(err_no_access_to_storage())))
  }
}

/// Handler for retrieving details of the single note.
#[get("/api/v1/notes/{id}")]
async fn handler_get_note(req: HttpRequest, id: Path<String>, data: web::Data<ApplicationData>) -> std::io::Result<Json<ResultDto<NoteDto>>> {
  if let Ok(storage) = data.storage.lock() {
    if is_authorized(&req, &storage) {
      if let Some(note) = storage.get_note(&id).map(|note| note.into()) {
        Ok(Json(ResultDto::data(note)))
      } else {
        Ok(Json(ResultDto::error(err_note_not_found(&id))))
      }
    } else {
      Ok(Json(ResultDto::error(err_not_authorized())))
    }
  } else {
    Ok(Json(ResultDto::error(err_no_access_to_storage())))
  }
}

/// Handler for creating a new note.
#[post("/api/v1/notes")]
async fn handler_create_note(req: HttpRequest, params: Json<CreateNoteParams>, data: web::Data<ApplicationData>) -> std::io::Result<Json<ResultDto<NoteDto>>> {
  if let Ok(mut storage) = data.storage.lock() {
    if is_authorized(&req, &storage) {
      if let Some(title) = &params.title {
        if let Some(content) = &params.content {
          let ttl = params.ttl.as_ref().unwrap_or(&"".to_string()).to_owned();
          if let Some(id) = storage.create_note(title, content, &ttl) {
            Ok(Json(ResultDto::data(NoteDto {
              note_id: id,
              ..NoteDto::default()
            })))
          } else {
            Ok(Json(ResultDto::error(err_creating_note_failed())))
          }
        } else {
          Ok(Json(ResultDto::error(err_required_attribute_not_specified("content"))))
        }
      } else {
        Ok(Json(ResultDto::error(err_required_attribute_not_specified("title"))))
      }
    } else {
      Ok(Json(ResultDto::error(err_not_authorized())))
    }
  } else {
    Ok(Json(ResultDto::error(err_no_access_to_storage())))
  }
}

/// Handler for user login.
#[post("/api/v1/login")]
async fn handler_login(params: Json<LoginParams>, data: web::Data<ApplicationData>) -> std::io::Result<Json<ResultDto<LoginDto>>> {
  if let Ok(mut storage) = data.storage.lock() {
    if let Some(login) = &params.login {
      if let Some(password) = &params.password {
        if let Some(token) = storage.get_token(login, password) {
          Ok(Json(ResultDto::data(LoginDto { token })))
        } else {
          Ok(Json(ResultDto::error(err_invalid_login_or_password())))
        }
      } else {
        Ok(Json(ResultDto::error(err_required_attribute_not_specified("password"))))
      }
    } else {
      Ok(Json(ResultDto::error(err_required_attribute_not_specified("login"))))
    }
  } else {
    Ok(Json(ResultDto::error(err_no_access_to_storage())))
  }
}

/// Handler for 404 errors.
async fn not_found_handler(req: HttpRequest) -> std::io::Result<Json<ResultDto<()>>> {
  Ok(Json(ResultDto::error(err_endpoint_not_found(req.path()))))
}

/// Starts the server.
pub async fn start_server() -> std::io::Result<()> {
  let storage = Storage::new();
  let application_data = web::Data::new(ApplicationData { storage: Mutex::new(storage) });
  let address = "0.0.0.0:8871";
  println!("started nordnotes {}", address);
  HttpServer::new(move || {
    let cors = Cors::permissive();
    App::new()
      .wrap(cors)
      .app_data(application_data.clone())
      .service(handler_delete_notes)
      .service(handler_get_notes)
      .service(handler_get_note)
      .service(handler_create_note)
      .service(handler_login)
      .default_service(web::route().to(not_found_handler))
  })
  .bind(address)?
  .run()
  .await
}

/// Returns a list of DTOs for notes.
fn get_notes(storage: &Storage) -> Vec<NoteDto> {
  storage
    .get_notes()
    .iter()
    .map(|note| NoteDto {
      note_id: note.note_id.clone(),
      title: Some(note.title.clone()),
      content: None,
    })
    .collect()
}

/// Checks if the request contains valid authorization header.
fn is_authorized(req: &HttpRequest, storage: &Storage) -> bool {
  if let Some(value) = req.headers().get("Authorization") {
    if !value.is_empty() {
      if let Ok(str_value) = value.to_str() {
        if let Some(token) = str_value.trim().strip_prefix("Bearer ") {
          return storage.check_token(token.trim());
        }
      }
    }
  }
  false
}

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

//! Implementation of the web server.

use crate::controllers::*;
use crate::dto::{LoginDto, NoteDto, ResultDto, ServiceInfoDto};
use crate::errors::*;
use crate::params::{CreateNoteParams, LoginParams};
use crate::storage::Storage;
use actix_cors::Cors;
use actix_web::web::{Json, Path};
use actix_web::{delete, get, post, web, App, HttpRequest, HttpServer};

/// Shared application data.
struct ApplicationData {
  /// Shared access to storage.
  storage: tokio::sync::RwLock<Storage>,
}

/// Handler for service information.
#[get("/api/v1/info")]
async fn handler_service_info() -> std::io::Result<Json<ResultDto<ServiceInfoDto>>> {
  let service_info = ServiceInfoDto {
    name: "nordnotes".to_string(),
    version: "1.0.0".to_string(),
    legal_note: "Copyright © 2022 Dariusz Depta Engos Software".to_string(),
  };
  Ok(Json(ResultDto::data(service_info)))
}

/// Handler for deleting all notes.
#[delete("/api/v1/notes")]
async fn handler_delete_notes(data: web::Data<ApplicationData>) -> std::io::Result<Json<ResultDto<String>>> {
  let mut storage = data.storage.write().await;
  match controller_delete_notes(&mut storage).await {
    Ok(result) => Ok(Json(ResultDto::data(result))),
    Err(reason) => Ok(Json(ResultDto::error(reason))),
  }
}

/// Handler for retrieving a list of notes.
#[get("/api/v1/notes")]
async fn handler_list_notes(data: web::Data<ApplicationData>) -> std::io::Result<Json<ResultDto<Vec<NoteDto>>>> {
  let storage = data.storage.read().await;
  match controller_list_notes(&storage).await {
    Ok(result) => Ok(Json(ResultDto::data(result))),
    Err(reason) => Ok(Json(ResultDto::error(reason))),
  }
}

/// Handler for retrieving details of the single note.
#[get("/api/v1/notes/{id}")]
async fn handler_get_note(req: HttpRequest, id: Path<String>, data: web::Data<ApplicationData>) -> std::io::Result<Json<ResultDto<NoteDto>>> {
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

/// Handler for creating a new note.
#[post("/api/v1/notes")]
async fn handler_create_note(req: HttpRequest, params: Json<CreateNoteParams>, data: web::Data<ApplicationData>) -> std::io::Result<Json<ResultDto<NoteDto>>> {
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

/// Handler for user login.
#[post("/api/v1/login")]
async fn handler_login(params: Json<LoginParams>, data: web::Data<ApplicationData>) -> std::io::Result<Json<ResultDto<LoginDto>>> {
  let mut storage = data.storage.write().await;
  match controller_login(params.into_inner(), &mut storage).await {
    Ok(result) => Ok(Json(ResultDto::data(result))),
    Err(reason) => Ok(Json(ResultDto::error(reason))),
  }
}

/// Handler for 404 errors.
async fn not_found_handler(req: HttpRequest) -> std::io::Result<Json<ResultDto<()>>> {
  Ok(Json(ResultDto::error(err_endpoint_not_found(req.path()))))
}

/// Starts the server.
pub async fn start_server() -> Result<()> {
  let storage = Storage::new().await?;
  let application_data = web::Data::new(ApplicationData {
    storage: tokio::sync::RwLock::new(storage),
  });
  let address = "0.0.0.0:8871";
  println!("started nordnotes {}", address);
  HttpServer::new(move || {
    let cors = Cors::permissive();
    App::new()
      .wrap(cors)
      .app_data(application_data.clone())
      .service(handler_service_info)
      .service(handler_delete_notes)
      .service(handler_list_notes)
      .service(handler_get_note)
      .service(handler_create_note)
      .service(handler_login)
      .default_service(web::route().to(not_found_handler))
  })
  .bind(address)?
  .run()
  .await
  .map_err(err_server_internal)
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

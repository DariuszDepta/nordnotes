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

//! # JSON RPC Server
//!
//! The implementation of the JSON RPC server based on Actix Web.
//! This server is responsible for:
//! - registering request handlers that take care of processing client requests,
//! - defining CORS permissions,
//! - initializing an access to shared storage.

use crate::errors::{NordNotesError, *};
use crate::handlers;
use crate::storage::Storage;
use actix_cors::Cors;
use actix_web::web::Json;
use actix_web::{web, App, HttpRequest, HttpServer};
use serde_derive::Serialize;

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

/// Shared application data.
pub struct ApplicationData {
  /// Shared access to storage.
  pub storage: tokio::sync::RwLock<Storage>,
}

/// Default handler (404 error).
async fn handler_404(req: HttpRequest) -> std::io::Result<Json<ResultDto<()>>> {
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
      // handlers for authorization
      .service(handlers::auth::login)
      // handlers for system operations
      .service(handlers::system::info)
      // handlers for roles
      .service(handlers::roles::create)
      .service(handlers::roles::list)
      .service(handlers::roles::find)
      .service(handlers::roles::delete_all)
      // handlers for notes
      .service(handlers::notes::list)
      .service(handlers::notes::get_by_id)
      .service(handlers::notes::delete_all)
      .service(handlers::notes::create)
      // default handler
      .default_service(web::route().to(handler_404))
  })
  .bind(address)?
  .run()
  .await
  .map_err(err_server_internal)
}

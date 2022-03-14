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

use crate::controllers::roles;
use crate::dto::{ResultDto, RoleDto};
use crate::errors::*;
use crate::handlers::is_authorized;
use crate::server::ApplicationData;
use actix_web::web::{Json, Path};
use actix_web::{delete, get, post, web, HttpRequest};
use serde_derive::Deserialize;

/// Parameters needed when a new note is created.
#[derive(Deserialize)]
pub struct CreateRoleParams {
  /// Name of a role.
  #[serde(rename = "name")]
  pub name: Option<String>,
}

impl CreateRoleParams {
  /// Validates attributes required when creating a new role.
  pub fn validate(self) -> Result<String> {
    if let Some(name) = self.name {
      Ok(name)
    } else {
      Err(err_required_attribute_not_specified("name"))
    }
  }
}

/// Handler for creating a new role.
#[post("/api/v1/roles")]
pub async fn create(req: HttpRequest, params: Json<CreateRoleParams>, data: web::Data<ApplicationData>) -> std::io::Result<Json<ResultDto<RoleDto>>> {
  let mut storage = data.storage.write().await;
  if is_authorized(&req, &storage) {
    match roles::create(params.into_inner(), &mut storage).await {
      Ok(result) => Ok(Json(ResultDto::data(result))),
      Err(reason) => Ok(Json(ResultDto::error(reason))),
    }
  } else {
    Ok(Json(ResultDto::error(err_not_authorized())))
  }
}

/// Handler for retrieving a list of roles.
#[get("/api/v1/roles")]
pub async fn list(data: web::Data<ApplicationData>) -> std::io::Result<Json<ResultDto<Vec<RoleDto>>>> {
  let storage = data.storage.read().await;
  match roles::list(&storage).await {
    Ok(result) => Ok(Json(ResultDto::data(result))),
    Err(reason) => Ok(Json(ResultDto::error(reason))),
  }
}

/// Handler for retrieving a single role searched by identifier.
#[get("/api/v1/roles/{id}")]
pub async fn find(req: HttpRequest, id: Path<String>, data: web::Data<ApplicationData>) -> std::io::Result<Json<ResultDto<RoleDto>>> {
  let storage = data.storage.read().await;
  if is_authorized(&req, &storage) {
    match roles::find(id.into_inner(), &storage).await {
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
  match roles::delete_all(&mut storage).await {
    Ok(result) => Ok(Json(ResultDto::data(result))),
    Err(reason) => Ok(Json(ResultDto::error(reason))),
  }
}

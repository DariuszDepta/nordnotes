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

use crate::controllers::auth;
use crate::errors::*;
use crate::server::{ApplicationData, ResultDto};
use actix_web::web::Json;
use actix_web::{post, web};
use serde_derive::{Deserialize, Serialize};

/// Data transfer object for login result.
#[derive(Serialize)]
pub struct LoginDto {
  #[serde(rename = "token")]
  pub token: String,
}

/// Parameters needed when processing user's login.
#[derive(Deserialize)]
pub struct LoginParams {
  #[serde(rename = "login")]
  pub login: Option<String>,
  #[serde(rename = "password")]
  pub password: Option<String>,
}

impl LoginParams {
  /// Validates required parameters for logging a user.
  pub fn validate(self) -> Result<(String, String)> {
    if let Some(login_value) = self.login {
      if let Some(password_value) = self.password {
        Ok((login_value, password_value))
      } else {
        Err(err_required_attribute_not_specified("password"))
      }
    } else {
      Err(err_required_attribute_not_specified("login"))
    }
  }
}

/// Handler for logging a user.
#[post("/api/v1/login")]
pub async fn login(params: Json<LoginParams>, data: web::Data<ApplicationData>) -> std::io::Result<Json<ResultDto<LoginDto>>> {
  let mut storage = data.storage.write().await;
  match auth::login(params.into_inner(), &mut storage).await {
    Ok(result) => Ok(Json(ResultDto::data(result))),
    Err(reason) => Ok(Json(ResultDto::error(reason))),
  }
}

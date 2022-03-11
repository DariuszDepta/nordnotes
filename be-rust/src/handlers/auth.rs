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

use crate::controllers::controller_login;
use crate::dto::{LoginDto, ResultDto};
use crate::params::LoginParams;
use crate::server::ApplicationData;
use actix_web::web::Json;
use actix_web::{post, web};

/// Handler for logging a user.
#[post("/api/v1/login")]
pub async fn login(params: Json<LoginParams>, data: web::Data<ApplicationData>) -> std::io::Result<Json<ResultDto<LoginDto>>> {
  let mut storage = data.storage.write().await;
  match controller_login(params.into_inner(), &mut storage).await {
    Ok(result) => Ok(Json(ResultDto::data(result))),
    Err(reason) => Ok(Json(ResultDto::error(reason))),
  }
}

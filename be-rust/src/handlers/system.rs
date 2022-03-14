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

use crate::server::ResultDto;
use actix_web::get;
use actix_web::web::Json;
use serde_derive::Serialize;

const SYSTEM_NAME: &str = "nordnotes";
const SYSTEM_VERSION: &str = "1.0.0";
const SYSTEM_LEGAL_NOTE: &str = "Copyright © 2022 Dariusz Depta Engos Software";

/// Data transfer object for service information.
#[derive(Serialize)]
pub struct SystemInfoDto {
  /// Service name.
  #[serde(rename = "name")]
  pub name: String,
  /// Service version.
  #[serde(rename = "version")]
  pub version: String,
  /// Legal note.
  #[serde(rename = "legalNote")]
  pub legal_note: String,
}

/// Handler for retrieving system information.
#[get("/api/v1/info")]
pub async fn info() -> std::io::Result<Json<ResultDto<SystemInfoDto>>> {
  Ok(Json(ResultDto::data(SystemInfoDto {
    name: SYSTEM_NAME.to_string(),
    version: SYSTEM_VERSION.to_string(),
    legal_note: SYSTEM_LEGAL_NOTE.to_string(),
  })))
}

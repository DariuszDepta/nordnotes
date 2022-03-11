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

use crate::dto::{ResultDto, ServiceInfoDto};
use actix_web::get;
use actix_web::web::Json;

const SYSTEM_NAME: &str = "nordnotes";
const SYSTEM_VERSION: &str = "1.0.0";
const SYSTEM_LEGAL_NOTE: &str = "Copyright © 2022 Dariusz Depta Engos Software";

/// Handler for retrieving system information.
#[get("/api/v1/info")]
pub async fn info() -> std::io::Result<Json<ResultDto<ServiceInfoDto>>> {
  let service_info = ServiceInfoDto {
    name: SYSTEM_NAME.to_string(),
    version: SYSTEM_VERSION.to_string(),
    legal_note: SYSTEM_LEGAL_NOTE.to_string(),
  };
  Ok(Json(ResultDto::data(service_info)))
}

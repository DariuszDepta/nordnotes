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

//! Implementation of request handlers called by web server.
//!
//! Request handlers are split between several modules, usually by *entity domain*.
//! It means that for example handlers operating on notes are grouped together.
//!
//! Handler responsibilities:
//! - take requests as input parameter,
//! - unwrap input parameter (when needed),
//! - unwrap access to storage,
//! - authorize access to business logic based on JWT token,
//! - orchestrate calls of controllers needed to execute required business logic,
//! - gather result DTOs and return response result to caller,
//! - when errors occur, return result describing the details.

use crate::storage::Storage;
use actix_web::HttpRequest;

pub mod auth;
pub mod notes;
pub mod system;

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

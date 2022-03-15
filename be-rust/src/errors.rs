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

//! Definition of common error type used across `nordnotes` application.

use scylla::cql_to_rust::FromRowError;
use scylla::transport::errors::{NewSessionError, QueryError};
use std::fmt::Display;
use std::io::Error;

/// Common result type used across `nordnotes` application.
pub type Result<T, E = NordNotesError> = std::result::Result<T, E>;

/// Common error used across `nordnotes` application.
#[derive(Debug)]
pub struct NordNotesError(String);

impl Display for NordNotesError {
  /// Implementation of [Display] trait for [NordNotesError].
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl From<Error> for NordNotesError {
  /// Converts [Error] into [NordNotesError].
  fn from(e: Error) -> Self {
    Self(e.to_string())
  }
}

/// Creates an internal web server error description.
pub fn err_server_internal(e: Error) -> NordNotesError {
  e.into()
}

/// Creates a non-existing endpoint error.
pub fn err_endpoint_not_found(message: &str) -> NordNotesError {
  NordNotesError(format!("endpoint not found: {}", message))
}

/// Creates a non-existing note error.
pub fn err_note_not_found(note_id: &str) -> NordNotesError {
  NordNotesError(format!("note not found, id = {}", note_id))
}

/// Creates a non-existing entity error.
pub fn err_entity_not_found(entity: &str, description: &str) -> NordNotesError {
  NordNotesError(format!("{} not found: {}", entity, description))
}

/// Creates a missing required attribute error.
pub fn err_required_attribute_not_specified(attribute_name: &str) -> NordNotesError {
  NordNotesError(format!("required attribute not specified, name = {}", attribute_name))
}

/// Creates a failed login error.
pub fn err_invalid_login_or_password() -> NordNotesError {
  NordNotesError("invalid login or password".to_string())
}

/// Creates a failed note creation error.
pub fn err_creating_note_failed() -> NordNotesError {
  NordNotesError("creating a new note failed".to_string())
}

/// Creates a failed role creation error.
pub fn err_creating_role_failed() -> NordNotesError {
  NordNotesError("creating a new role failed".to_string())
}

/// Creates a not authorized user error.
pub fn err_not_authorized() -> NordNotesError {
  NordNotesError("not authorized".to_string())
}

/// Creates a new session initialization error.
pub fn err_new_session(e: NewSessionError) -> NordNotesError {
  NordNotesError(format!("{:?}", e))
}

/// Creates an error for failed query.
pub fn err_query(e: QueryError) -> NordNotesError {
  NordNotesError(format!("{:?}", e))
}

/// Creates an error for failure during row conversion.
pub fn err_from_row(e: FromRowError) -> NordNotesError {
  NordNotesError(format!("{:?}", e))
}

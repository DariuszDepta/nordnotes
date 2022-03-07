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

/// Common error definition used by `nordnotes` application.
#[derive(Debug)]
pub struct NordNotesError(String);

impl std::fmt::Display for NordNotesError {
  /// Implementation of [Display](std::fmt::Display) trait for [NordNotesError].
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl NordNotesError {
  /// Creates a new [NordNotesError] with specified `message`.
  pub fn new(message: &str) -> Self {
    Self(message.to_string())
  }
}

/// Creates a new error describing non-existing endpoint.
pub fn err_endpoint_not_found(message: &str) -> NordNotesError {
  NordNotesError::new(&format!("endpoint not found: {}", message))
}

/// Creates a new error describing non-existing note.
pub fn err_note_not_found(note_id: &str) -> NordNotesError {
  NordNotesError::new(&format!("note not found, id = {}", note_id))
}

/// Creates a new error describing missing required attribute
pub fn err_required_attribute_not_specified(attribute_name: &str) -> NordNotesError {
  NordNotesError::new(&format!("required attribute not specified, name = {}", attribute_name))
}

/// Creates a new error describing why login has failed.
pub fn err_invalid_login_or_password() -> NordNotesError {
  NordNotesError::new("invalid login or password")
}

/// Creates a new error describing that creating a new note has failed.
pub fn err_creating_note_failed() -> NordNotesError {
  NordNotesError::new("creating a new note has failed")
}

/// Creates a new error describing no access to storage.
pub fn err_no_access_to_storage() -> NordNotesError {
  NordNotesError::new("no access to storage")
}

/// Creates a new error describing that a user is not authorized.
pub fn err_not_authorized() -> NordNotesError {
  NordNotesError::new("not authorized")
}

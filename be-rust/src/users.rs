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

//! Implementation of the data object for users.

use crate::utils::uuid;

/// Data object for user.
#[derive(Clone)]
pub struct User {
  /// Unique user identifier.
  pub user_id: String,
  /// User login.
  pub login: String,
  /// User password.
  pub password: String,
  /// Token generated for the user.
  pub token: Option<String>,
}

impl User {
  /// Creates a new user.
  pub fn new(login: &str, password: &str) -> Self {
    Self {
      user_id: uuid(),
      login: login.to_string(),
      password: password.to_string(),
      token: None,
    }
  }
}

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

//! Implementation of input parameters required by controllers.

use crate::errors::*;
use serde_derive::Deserialize;

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
    if let Some(login) = self.login {
      if let Some(password) = self.password {
        Ok((login, password))
      } else {
        Err(err_required_attribute_not_specified("password"))
      }
    } else {
      Err(err_required_attribute_not_specified("login"))
    }
  }
}

/// Parameters needed when a new note is created.
#[derive(Deserialize)]
pub struct CreateNoteParams {
  /// The title of a note.
  #[serde(rename = "title")]
  pub title: Option<String>,
  /// The content of a note.
  #[serde(rename = "content")]
  pub content: Option<String>,
  /// Time to live, defines how long the content of a note will be available.
  /// `ttl` has the format: `Nw`, `Nd`, `Nh` or `Nm`, where `N` is an integer
  /// and letters have following meaning: `w` - weeks, `d` - days, `h` - hours, `m` - minutes.
  /// For example `ttl` == "10d" means that the note will expire after 10 days from creation.
  #[serde(rename = "ttl")]
  pub ttl: Option<String>,
}

impl CreateNoteParams {
  /// Validates required parameters for creating a new note.
  pub fn validate(self) -> Result<(String, String, String)> {
    if let Some(title) = self.title {
      if let Some(content) = self.content {
        Ok((title, content, self.ttl.unwrap_or("".to_string()).to_owned()))
      } else {
        Err(err_required_attribute_not_specified("content"))
      }
    } else {
      Err(err_required_attribute_not_specified("title"))
    }
  }
}

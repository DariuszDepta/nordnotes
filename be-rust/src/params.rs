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

use serde::Deserialize;

/// Parameters needed when processing user's login.
#[derive(Deserialize)]
pub struct LoginParams {
  #[serde(rename = "login")]
  pub login: Option<String>,
  #[serde(rename = "password")]
  pub password: Option<String>,
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

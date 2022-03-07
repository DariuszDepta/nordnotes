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

//! Application description (TBD)

extern crate actix_cors;
extern crate actix_web;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate time;
extern crate uuid;

use crate::server::start_server;

mod dto;
mod errors;
mod notes;
mod params;
mod server;
mod storage;
mod users;
mod utils;

/// Main entrypoint of the application.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
  start_server().await
}

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

//! The `nordnotes` application.

extern crate actix_cors;
extern crate actix_web;
extern crate lazy_static;
extern crate scylla;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate time;
extern crate tokio;
extern crate uuid;

use crate::errors::Result;
use crate::server::start_server;

mod controllers;
mod dto;
mod entities;
mod errors;
mod params;
mod repositories;
mod server;
mod storage;
mod utils;

/// Main entrypoint of the `nordnotes` application.
#[tokio::main]
async fn main() -> Result<()> {
  start_server().await
}

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

//! Implementation of controllers for authorization.

use crate::errors::*;
use crate::handlers::auth::{LoginDto, LoginParams};
use crate::storage::Storage;

/// Controller for logging a user.
pub async fn login(params: LoginParams, storage: &mut Storage) -> Result<LoginDto> {
  let (login, password) = params.validate()?;
  if let Some(token) = storage.get_token(&login, &password) {
    Ok(LoginDto { token })
  } else {
    Err(err_invalid_login_or_password())
  }
}

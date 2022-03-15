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

//! Implementation od system services.

use crate::errors::Result;
use crate::storage::Storage;

/// Initializes roles and users.
///
/// This function checks the values of the following environment variables:
/// - `NORDNOTES_SUPERUSER`
/// - `NORDNOTES_PASSWORD`.
/// The following logic is applied (presented below as decision table):
/// ```text
/// ┌─────────────────────┬
/// │ NORDNOTES_SUPERUSER
///
/// ┌───┬──────────┬───────╥──────╥─────────────┐
/// │ U │ Customer │ Order ║      ║ Description │
/// ╞═══╪══════════╪═══════╬══════╬═════════════╡
/// │ 1 │"Business"│  <10  ║ 0.10 ║ Small order │
/// ├───┼──────────┼───────╫──────╫─────────────┤
/// │ 2 │"Business"│ >=10  ║ 0.15 ║ Large order │
/// ├───┼──────────┼───────╫──────╫─────────────┤
/// │ 3 │"Private" │   -   ║ 0.05 ║ All orders  │
/// └───┴──────────┴───────╨──────╨─────────────┘
/// ```
pub async fn initialize_roles_and_users(_storage: &Storage) -> Result<()> {
  if let Ok(_superuser) = std::env::var("NORDNOTES_SUPERUSER") {
    if let Ok(_password) = std::env::var("NORDNOTES_PASSWORD") {
      let _user_exists = false; //TODO check in database
      let _role_exists = false; //TODO check in database
    }
  }
  Ok(())
}

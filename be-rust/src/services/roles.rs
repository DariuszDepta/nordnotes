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

use crate::entities::role::RoleEntity;
use crate::errors::Result;
use crate::storage::Storage;

/// Service for retrieving a single role by its identifier.
pub async fn find_by_id(role_id: String, storage: &Storage) -> Result<RoleEntity> {
  Ok(storage.roles_repository.find_by_id(&role_id).await?)
}

/// Service for retrieving a single role by its name.
pub async fn find_by_name(role_name: String, storage: &Storage) -> Result<RoleEntity> {
  Ok(storage.roles_repository.find_by_name(&role_name).await?)
}

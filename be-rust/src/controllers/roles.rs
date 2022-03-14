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

//! Implementation of controllers for roles.

use crate::entities::role::RoleEntity;
use crate::errors::*;
use crate::handlers::roles::{CreateRoleParams, RoleDto};
use crate::storage::Storage;

/// Controller for creating a new role.
pub async fn create(params: CreateRoleParams, storage: &mut Storage) -> Result<RoleDto> {
  let name = params.validate()?;
  let role = RoleEntity::new(&name);
  if let Ok(role_id) = storage.roles_repository.create(role).await {
    Ok(RoleDto { role_id, ..RoleDto::default() })
  } else {
    Err(err_creating_role_failed())
  }
}

/// Controller for retrieving a list of roles.
pub async fn list(storage: &Storage) -> Result<Vec<RoleDto>> {
  Ok(storage.roles_repository.list().await?.iter().map(|role| role.into()).collect())
}

/// Controller for retrieving a single role by its identifier.
pub async fn find(id: String, storage: &Storage) -> Result<RoleDto> {
  Ok(storage.roles_repository.find(&id).await?.into())
}

/// Controller for deleting all roles.
pub async fn delete_all(storage: &mut Storage) -> Result<String> {
  storage.roles_repository.delete_all().await?;
  Ok("all roles deleted".to_string())
}

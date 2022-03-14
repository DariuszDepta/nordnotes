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

//! Implementation of database repository for notes.

use crate::entities::role::RoleEntity;
use crate::entities::Entity;
use crate::errors::*;
use crate::storage::{KEYSPACE, TABLE_ROLES};
use lazy_static::lazy_static;
use scylla::{IntoTypedRows, Session, ValueList};
use std::sync::Arc;

lazy_static! {
  static ref QUERY_CREATE: String = format!("INSERT INTO {}.{} (role_id, name) VALUES (?, ?)", KEYSPACE, TABLE_ROLES);
  static ref QUERY_LIST: String = format!("SELECT role_id, name FROM {}.{}", KEYSPACE, TABLE_ROLES);
  static ref QUERY_FIND: String = format!("SELECT role_id, name FROM {}.{} WHERE role_id = ?", KEYSPACE, TABLE_ROLES);
  static ref QUERY_DELETE_ALL: String = format!("TRUNCATE TABLE {}.{}", KEYSPACE, TABLE_ROLES);
}

/// Repository for roles.
pub struct RolesRepository {
  session: Arc<Session>,
}

/// Value list containing the role's identifier.
#[derive(ValueList)]
struct RoleId {
  role_id: String,
}

impl RolesRepository {
  /// Creates a new roles repository.
  pub fn new(session: Arc<Session>) -> Self {
    Self { session }
  }
  /// Creates a new note.
  pub async fn create(&self, role: RoleEntity) -> Result<String> {
    let values = (role.id(), role.name());
    self.session.query(QUERY_CREATE.as_str(), values).await.map_err(err_query)?;
    Ok(role.id())
  }
  /// Lists all notes.
  pub async fn list(&self) -> Result<Vec<RoleEntity>> {
    let mut roles = vec![];
    if let Some(rows) = self.session.query(QUERY_LIST.as_str(), &[]).await.map_err(err_query)?.rows {
      for row in rows.into_typed::<RoleEntity>() {
        let role = row.map_err(err_from_row)?;
        roles.push(role);
      }
    }
    Ok(roles)
  }
  /// Searches for a role with specified identifier.
  pub async fn find(&self, role_id: &str) -> Result<RoleEntity> {
    let id = RoleId { role_id: role_id.to_string() };
    if let Some(rows) = self.session.query(QUERY_FIND.as_str(), id).await.map_err(err_query)?.rows {
      if let Some(row) = rows.into_typed::<RoleEntity>().take(1).next() {
        return row.map_err(err_from_row);
      }
    }
    Err(err_entity_not_found("role", role_id))
  }
  /// Deletes all roles.
  pub async fn delete_all(&self) -> Result<()> {
    self.session.query(QUERY_DELETE_ALL.as_str(), &[]).await.map_err(err_query)?;
    Ok(())
  }
}

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

//! Implementation of role entity.

use super::Entity;
use crate::utils::uuid;
use scylla::macros::FromRow;

/// Role entity.
#[derive(Debug, Clone, FromRow)]
pub struct RoleEntity {
  /// Unique role identifier.
  role_id: String,
  /// Role name (always uppercase).
  name: String,
}

impl Entity for RoleEntity {
  /// Returns role's unique identifier.
  fn id(&self) -> String {
    self.role_id.clone()
  }
}

impl RoleEntity {
  /// Creates a new role.
  pub fn new(name: &str) -> Self {
    Self {
      role_id: uuid(),
      name: name.to_uppercase(),
    }
  }
  /// Returns role's name.
  pub fn name(&self) -> String {
    self.name.clone()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_create() {
    let role = RoleEntity::new("admin");
    assert_eq!(36, role.id().len());
    assert_eq!("ADMIN", role.name());
  }
}

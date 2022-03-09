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

//! Implementation of controllers called by web server handlers.

use crate::dto::NoteDto;
use crate::errors::Result;
use crate::storage::Storage;

/// Controller for deleting all notes.
pub async fn controller_delete_notes(storage: &mut Storage) -> Result<String> {
  storage.delete_notes();
  Ok("all notes deleted".to_string())
}

/// Controller for retrieving a list of notes.
pub async fn controller_list_notes(storage: &Storage) -> Result<Vec<NoteDto>> {
  Ok(
    storage
      .get_notes()
      .iter()
      .map(|note| NoteDto {
        note_id: note.note_id.clone(),
        title: Some(note.title.clone()),
        content: None,
      })
      .collect(),
  )
}

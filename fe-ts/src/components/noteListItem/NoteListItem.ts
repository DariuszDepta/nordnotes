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

import './NoteListItem.css';
import template from './NoteListItem.html';
import {Note} from '../../model/Note';
import {Component} from 'simpa/lib/component';
import {$innerHTML, $touch,} from 'simpa/lib/utils';
import {onError, onUnrecoverableError} from '../../model/Error';
import app from '../../app/App';
import noteService from '../../services/NoteService';
import authorizationService from '../../services/AuthorizationService';

export class NoteListItem extends Component {

  private readonly titleId = '';
  private readonly note: Note;

  /** Creates note list item component. */
  constructor(note: Note) {
    super('nn-note-list-item');
    this.note = note;
  }

  /** Returns a template imported from HTML file. */
  public doTemplate(): string {
    return template;
  }

  /** Initializes all components. */
  public doInit(): void {
    super.doInit();
    const me = this;
    $innerHTML(this.titleId, this.note.title);
    $touch(this.titleId, () => {
      if (authorizationService.isAuthorized()) {
        noteService.getNote(me.note.noteId, (note) => {
          app.showNoteDetailsView(note);
        }, onError, onUnrecoverableError);
      } else {
        app.showLoginView(
          () => {
            app.showNoteListView();
          },
          () => {
            noteService.getNote(me.note.noteId, (note) => {
              app.showNoteDetailsView(note);
            }, onError, onUnrecoverableError);
          });
      }
    });
  }
}

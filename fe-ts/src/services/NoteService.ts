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

import {Note} from '../model/Note';
import {configuration} from '../common/Configuration';
import {$GET, $POST} from '../common/Utils';
import {CreateNoteParams} from '../forms/noteForm/CreateNoteParams';
import {IErrorCallback, IUnrecoverableCallback} from '../model/Error';

/** Definition od callback with the list of notes. */
export interface IGetNotesCallback {
  (notes: Note[]): void;
}

/** Definition od callback with single note. */
export interface IGetNoteCallback {
  (note: Note): void;
}

/** Service for handling operations on notes. */
class NoteService {
  /** Retrieves a collection of notes from server. */
  public getNotes(dataCallback: IGetNotesCallback, errorCallback: IErrorCallback, unrecoverableCallback: IUnrecoverableCallback) {
    fetch(configuration().API_URL + '/notes', $GET())
      .then(result => result.json())
      .then(response => {
        if (response.data) {
          dataCallback(response.data);
        }
        if (response.errors) {
          errorCallback(response.errors);
        }
      })
      .catch(reason => {
        unrecoverableCallback(reason);
      });
  }

  /** Retrieves a note with specified identifier. */
  public getNote(noteId: string, dataCallback: IGetNoteCallback, errorCallback: IErrorCallback, unrecoverableCallback: IUnrecoverableCallback) {
    fetch(configuration().API_URL + '/notes/' + noteId, $GET())
      .then(result => result.json())
      .then(response => {
        if (response.data) {
          dataCallback(response.data);
        }
        if (response.errors) {
          errorCallback(response.errors);
        }
      })
      .catch(reason => {
        unrecoverableCallback(reason);
      });
  }

  /** Creates a new note. */
  public createNote(params: CreateNoteParams, dataCallback: IGetNoteCallback, errorCallback: IErrorCallback, unrecoverableCallback: IUnrecoverableCallback) {
    fetch(configuration().API_URL + '/notes', $POST(params))
      .then(result => result.json())
      .then(response => {
        if (response.data) {
          dataCallback(response.data);
        }
        if (response.errors) {
          errorCallback(response.errors);
        }
      })
      .catch(reason => {
        unrecoverableCallback(reason);
      });
  }
}

/** Global service object. */
const noteService = new NoteService();

/** Default export. */
export default noteService;

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

import {View} from 'simpa/lib/view';
import {Note} from '../../model/Note';
import {NoteDetails} from '../../components/noteDetails/NoteDetails';

/** View for presenting the details of the note. */
export class NoteDetailsView extends View {

  private noteDetails: NoteDetails;

  /** Creates a view. */
  constructor() {
    super('nn-note-details-view');
  }

  /** Displays the view with note details. */
  public showView(note: Note) {
    this.noteDetails = new NoteDetails(note);
    this.noteDetails.doCreate();
    this.replaceBody(this.noteDetails.componentRoot);
    this.noteDetails.doBuild();
    this.noteDetails.doInit();
    super.show();
  }
}

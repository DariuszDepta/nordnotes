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
import {NoteList} from '../../components/noteList/NoteList';

/** View for presenting the list of notes. */
export class NoteListView extends View {

  private readonly noteList: NoteList;

  /** Creates a view. */
  constructor() {
    super('nn-note-list-view');
    this.noteList = new NoteList();
  }

  /** Creates components. */
  public doCreate() {
    super.doCreate();
    this.noteList.doCreate();
  }

  /** Builds the DOM structure. */
  public doBuild() {
    super.doBuild();
    this.appendChild(this.noteList.componentRoot);
    this.noteList.doBuild();
  }

  /** Initializes the view. */
  public doInit() {
    super.doInit();
    this.noteList.doInit();
  }

  /** Reloads the list of notes and displays the view. */
  public refreshAndShow(): void {
    this.noteList.refresh();
    super.show();
  }
}

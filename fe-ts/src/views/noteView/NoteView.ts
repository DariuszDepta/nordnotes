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
import {NoteForm} from '../../forms/noteForm/NoteForm';

/** View for presenting the form for creating a new note. */
export class NoteView extends View {

  private readonly noteForm: NoteForm;

  /** Creates a view. */
  constructor() {
    super('nn-note-view');
    this.noteForm = new NoteForm();
  }

  /** Creates components. */
  public doCreate() {
    super.doCreate();
    this.noteForm.doCreate();
  }

  /** Builds the DOM structure. */
  public doBuild() {
    super.doBuild();
    this.appendChild(this.noteForm.componentRoot);
    this.noteForm.doBuild();
  }

  /** Initializes the view. */
  public doInit() {
    super.doInit();
    this.noteForm.doInit();
  }
}

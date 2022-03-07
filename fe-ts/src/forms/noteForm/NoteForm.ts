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

import './NoteForm.css';
import template from './NoteForm.html';
import {Form} from 'simpa/lib/form';
import {CreateNoteParams} from './CreateNoteParams';
import {$hidden, $id, $SetFocus, $touch, $visible} from 'simpa/lib/utils';
import {onError, onUnrecoverableError} from '../../model/Error';
import app from '../../app/App';
import noteService from '../../services/NoteService';

/** Form for creating a new note. */
export class NoteForm extends Form {

  private readonly titleId = '';
  private readonly titleValidationMessageId = '';
  private readonly contentId = '';
  private readonly contentValidationMessageId = '';
  private readonly ttlId = '';
  private readonly ttlValidationMessageId = '';
  private readonly buttonCreateId = '';
  private readonly buttonCancelId = '';
  private titleEl: HTMLInputElement;
  private contentEl: HTMLInputElement;
  private ttlEl: HTMLInputElement;

  /** Creates a form for creating a new note. */
  constructor() {
    super('nn-note-form');
  }

  /** Returns a template imported from HTML file. */
  public doTemplate(): string {
    return template;
  }

  /** Initializes all components. */
  public doInit(): void {
    super.doInit();
    this.titleEl = $id(this.titleId) as HTMLInputElement;
    this.contentEl = $id(this.contentId) as HTMLInputElement;
    this.ttlEl = $id(this.ttlId) as HTMLInputElement;
    const me = this;
    $touch(this.buttonCreateId, () => {
      me.processCreate();
    });
    $touch(this.buttonCancelId, () => {
      me.processCancel();
    });
  }

  /** Processes an action that creates a new note. */
  private processCreate(): void {
    this.hideErrorMessages();
    const params = this.validateForm();
    if (params) {
      noteService.createNote(params, (_) => {
        app.showNoteListView(true);
      }, onError, onUnrecoverableError);
    }
  }

  /** Processes an action that cancels creating a new note. */
  private processCancel(): void {
    this.clearForm();
    app.showNoteListView();
  }

  /** Validates input fields. */
  private validateForm(): CreateNoteParams | null {
    // validate note's title
    const title = this.titleEl.value;
    if (title) {
      $hidden(this.titleValidationMessageId);
    } else {
      $visible(this.titleValidationMessageId);
      $SetFocus(this.titleEl);
      return null;
    }
    // validate note's content
    const content = this.contentEl.value;
    if (content) {
      $hidden(this.contentValidationMessageId);
    } else {
      $visible(this.contentValidationMessageId);
      $SetFocus(this.contentEl);
      return null;
    }
    // validate note's expiration time (time to live)
    const ttl = this.ttlEl.value.trim();
    if (ttl) {
      const rx = new RegExp('^[1-9][0-9]*[wdhm]$');
      if (rx.test(ttl)) {
        $hidden(this.ttlValidationMessageId);
      } else {
        $visible(this.ttlValidationMessageId);
        $SetFocus(this.ttlEl);
        return null;
      }
    }
    // prepare a note
    let params = new CreateNoteParams();
    params.title = title;
    params.content = content;
    params.ttl = ttl;
    this.clearForm();
    return params;
  }

  /** Clears the content of all form fields. */
  private clearForm() {
    this.titleEl.value = '';
    this.contentEl.value = '';
    this.ttlEl.value = '';
    this.hideErrorMessages();
  }

  /** Hides all error messages. */
  private hideErrorMessages() {
    $hidden(this.titleValidationMessageId);
    $hidden(this.contentValidationMessageId);
    $hidden(this.ttlValidationMessageId);
  }
}

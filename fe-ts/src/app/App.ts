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

import './App.css';
import template from './App.html';
import {Application} from 'simpa/lib/application';
import {Footer} from '../components/footer/Footer';
import {Header} from '../components/header/Header';
import {IViewCallback} from '../common/Utils';
import {LoginView} from '../views/loginView/LoginView';
import {Note} from '../model/Note';
import {NoteDetailsView} from '../views/noteDetailsView/NoteDetailsView';
import {NoteListView} from '../views/noteListView/NoteListView';
import {NoteView} from '../views/noteView/NoteView';
import {UnrecoverableErrorView} from '../views/unrecoverableErrorView/UnrecoverableErrorView';
import {$id} from 'simpa/lib/utils';
import {onError, onUnrecoverableError} from '../model/Error';
import authorizationService from '../services/AuthorizationService';
import noteService from '../services/NoteService';

/**
 * The application.
 */
class App extends Application {

  private readonly historyTitle = 'nordnotes';
  private readonly headerContainerId = '';
  private readonly viewContainerId = '';
  private readonly footerContainerId = '';
  private readonly header: Header;
  private readonly footer: Footer;
  private readonly loginView: LoginView;
  private readonly noteDetailsView: NoteDetailsView;
  private readonly noteListView: NoteListView;
  private readonly noteView: NoteView;
  private readonly unrecoverableErrorView: UnrecoverableErrorView;

  /** Creates the application. */
  constructor(appContainerId: string, appClass: string) {
    super(appContainerId, appClass);
    // clear authorization tokens
    authorizationService.clearAuthorized();
    // create a view with login form
    this.loginView = new LoginView();
    this.views.push(this.loginView);
    // create a view with note details (title, content, ...)
    this.noteDetailsView = new NoteDetailsView();
    this.views.push(this.noteDetailsView);
    // create a view with shared notes
    this.noteListView = new NoteListView();
    this.views.push(this.noteListView);
    // create a view for creating a new note
    this.noteView = new NoteView();
    this.views.push(this.noteView);
    // create a view for unrecoverable error
    this.unrecoverableErrorView = new UnrecoverableErrorView();
    this.views.push(this.unrecoverableErrorView);
    // create header and footer
    this.header = new Header();
    this.footer = new Footer();
  }

  /** Returns a template imported from HTML file. */
  public doTemplate(): string {
    return template;
  }

  /** Creates all components. */
  public doCreate(): void {
    super.doCreate();
    this.header.doCreate();
    this.doCreateViews();
    this.footer.doCreate();
  }

  /** Builds the DOM structure. */
  public doBuild(): void {
    super.doBuild();
    $id(this.headerContainerId).appendChild(this.header.componentRoot);
    this.header.doBuild();
    this.views.forEach((view) => {
      $id(this.viewContainerId).appendChild(view.componentRoot);
    });
    this.doBuildViews();
    $id(this.footerContainerId).appendChild(this.footer.componentRoot);
    this.footer.doBuild();
  }

  /** Initializes all components. */
  public doInit(): void {
    const me = this;
    this.header.doInit();
    this.doInitViews();
    this.footer.doInit();
    this.hideViews();
    this.showContentPointedByUrl();
    window.onhashchange = () => {
      me.showContentPointedByUrl();
    };
  };

  /** Shows the view with login form. */
  public showLoginView(prev: IViewCallback, next: IViewCallback, replace: boolean = false): void {
    this.hideViews();
    this.loginView.showView(prev, next);
    this.updateHistory('login-view', '#/login', replace);
  }

  /** Shows the view with note details. */
  public showNoteDetailsView(note: Note, replace: boolean = false) {
    this.hideViews();
    this.noteDetailsView.showView(note);
    this.updateHistory('note-details-view', '#/notes/' + note.noteId, replace);
  }

  /** Shows the view with the list of notes. */
  public showNoteListView(refresh: boolean = false, replace: boolean = false): void {
    this.hideViews();
    if (refresh) {
      this.noteListView.refreshAndShow();
    } else {
      this.noteListView.show();
    }
    this.updateHistory('note-list-view', '#/notes', replace);
  }

  /** Shows the view with a form for creating a new note. */
  public showNoteView(replace: boolean = false): void {
    this.hideViews();
    this.noteView.show();
    this.updateHistory('note-view', '#/note', replace);
  }

  /** Shows the view with unrecoverable error message. */
  public showUnrecoverableErrorView(message: string, replace: boolean = false): void {
    this.hideViews();
    this.unrecoverableErrorView.showView(message);
    this.updateHistory('unrecoverable-error-view', '#/error', replace);
  }

  /** Updates the browser's history. */
  private updateHistory(id: string, tag: string, replace: boolean): void {
    if (replace) {
      history.replaceState({id: id}, this.historyTitle, tag);
    } else {
      history.pushState({id: id}, this.historyTitle, tag);
    }
  }

  /**
   * Displays the content pointed by the URL tag after the hash.
   */
  private showContentPointedByUrl(): void {
    const me = this;
    let pointer = window.location.hash;
    if (pointer && pointer.length > 0) {
      if (pointer === '#/') {
        me.showNoteListView();
        return;
      }
      let rxNotes = new RegExp('^#/notes(/)?$');
      if (rxNotes.test(pointer)) {
        me.showNoteListView();
        return;
      }
      let rxNoteDetails = new RegExp('^#/note/([0-9a-f]{8}(-[0-9a-f]{4}){3}-[0-9a-f]{12})(/)?$');
      if (rxNoteDetails.test(pointer)) {
        const noteId = pointer.match(rxNoteDetails)[1];
        noteService.getNote(noteId, (note) => {
          me.showNoteDetailsView(note);
        }, onError, onUnrecoverableError);
        return;
      }
    }
    this.showNoteListView(false, true);
  }
}

// Create an application, provide the identifier of the element that will contain the whole application and its style.
const app = new App('appContainer', 'nn-app');

// The default export is the application itself.
export default app;

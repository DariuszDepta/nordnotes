import './NoteList.css';
import template from './NoteList.html';
import {Component} from 'simpa/lib/component';
import {NoteListItem} from '../noteListItem/NoteListItem';
import {Note} from '../../model/Note';
import {$DeleteChildren, $id, $touch} from 'simpa/lib/utils';
import {onError, onUnrecoverableError} from '../../model/Error';
import noteService from '../../services/NoteService';
import app from '../../app/App';
import authorizationService from '../../services/AuthorizationService';

export class NoteList extends Component {

  private readonly buttonCreateNoteId = '';
  private readonly containerId = '';
  private containerEl: HTMLElement;

  /** Creates note list component. */
  constructor() {
    super('nn-note-list');
  }

  /** Returns a template imported from HTML file. */
  public doTemplate(): string {
    return template;
  }

  /** Initializes all components. */
  public doInit(): void {
    super.doInit();
    const me = this;
    this.containerEl = $id(this.containerId);
    $touch(this.buttonCreateNoteId, () => {
      me.doCreateNote();
    });
    noteService.getNotes(
      (notes) => {
        me.addNotes(notes);
      }, onError, onUnrecoverableError);
  }

  /** Reloads the list of notes. */
  public refresh(): void {
    const me = this;
    noteService.getNotes(
      (notes) => {
        $DeleteChildren(me.containerEl);
        me.addNotes(notes);
      }, onError, onUnrecoverableError);
  }

  /** Adds a note to the list. */
  private addNotes(notes: Note[]): void {
    for (let note of notes) {
      let noteListItem = new NoteListItem(note);
      noteListItem.doCreate();
      this.containerEl.appendChild(noteListItem.componentRoot);
      noteListItem.doBuild();
      noteListItem.doInit();
    }
  }

  /** Process user's action - switch to form for creating a note, authorization is required. */
  private doCreateNote() {
    if (authorizationService.isAuthorized()) {
      app.showNoteView();
    } else {
      app.showLoginView(
        () => {
          app.showNoteListView();
        },
        () => {
          app.showNoteView();
        });
    }
  }
}

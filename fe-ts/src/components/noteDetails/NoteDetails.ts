import './NoteDetails.css';
import template from './NoteDetails.html';
import {Note} from '../../model/Note';
import {Component} from 'simpa/lib/component';
import {$innerHTML, $md, $touch} from 'simpa/lib/utils';
import app from '../../app/App';

/** Component for presenting the details of a note. */
export class NoteDetails extends Component {

  private readonly buttonBackId = '';
  private readonly titleId = '';
  private readonly contentId = '';
  private readonly note: Note;

  /** Creates note's details component. */
  constructor(note: Note) {
    super('nn-note-details');
    this.note = note;
  }

  /** Returns a template imported from HTML file. */
  public doTemplate(): string {
    return template;
  }

  /** Initializes all components. */
  public doInit(): void {
    super.doInit();
    $innerHTML(this.titleId, this.note.title);
    $innerHTML(this.contentId, $md(this.note.content));
    $touch(this.buttonBackId, () => {
      app.showNoteListView();
    });
  }
}

import './UnrecoverableError.css';
import template from './UnrecoverableError.html';
import {Component} from 'simpa/lib/component';
import {$id, $InnerHTML} from 'simpa/lib/utils';

export class UnrecoverableError extends Component {

  private readonly unrecoverableErrorId = '';
  private unrecoverableErrorEl: HTMLDivElement;

  /** Creates an unrecoverable error component. */
  constructor() {
    super('nn-unrecoverable-error');
  }

  /** Returns a template imported from HTML file. */
  public doTemplate(): string {
    return template;
  }

  /** Initializes all components. */
  public doInit(): void {
    super.doInit();
    this.unrecoverableErrorEl = $id(this.unrecoverableErrorId) as HTMLDivElement;
  }

  /** Sets the value for a message displayed as unrecoverable error. */
  public setMessage(message: string) {
    $InnerHTML(this.unrecoverableErrorEl, message);
  }
}

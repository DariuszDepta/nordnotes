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
import {UnrecoverableError} from '../../components/unrecoverableError/UnrecoverableError';

/** View for presenting unrecoverable error message. */
export class UnrecoverableErrorView extends View {

  private readonly unrecoverableError: UnrecoverableError;

  /** Creates a view. */
  constructor() {
    super('nn-unrecoverable-error-view');
    this.unrecoverableError = new UnrecoverableError();
  }

  /** Creates components. */
  public doCreate(): void {
    super.doCreate();
    this.unrecoverableError.doCreate();
  }

  /** Builds the DOM structure. */
  public doBuild(): void {
    super.doBuild();
    this.appendChild(this.unrecoverableError.componentRoot);
    this.unrecoverableError.doBuild();
  }

  /** Initializes the view. */
  public doInit(): void {
    super.doInit();
    this.unrecoverableError.doInit();
  }

  /** Shows the view with unrecoverable error message. */
  public showView(message: string): void {
    this.unrecoverableError.setMessage(message);
    super.show();
  }
}

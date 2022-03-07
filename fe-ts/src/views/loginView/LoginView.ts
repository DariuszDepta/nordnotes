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
import {LoginForm} from '../../forms/loginForm/LoginForm';
import {IViewCallback} from '../../common/Utils';

/** View for presenting the login form. */
export class LoginView extends View {

  private readonly loginForm: LoginForm;

  /** Creates a view. */
  constructor() {
    super('nn-login-view');
    this.loginForm = new LoginForm();
  }

  /** Creates components. */
  public doCreate() {
    super.doCreate();
    this.loginForm.doCreate();
  }

  /** Builds the DOM structure. */
  public doBuild() {
    super.doBuild();
    this.appendChild(this.loginForm.componentRoot);
    this.loginForm.doBuild();
  }

  /** Initializes the view. */
  public doInit() {
    super.doInit();
    this.loginForm.doInit();
  }

  /** Sets view switching callbacks and shows the login view. */
  public showView(prev: IViewCallback, next: IViewCallback): void {
    this.loginForm.switchViews(prev, next);
    super.show();
  }
}

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

import './LoginForm.css';
import template from './LoginForm.html';
import {Form} from 'simpa/lib/form';
import {LoginParams} from './LoginParams';
import {IViewCallback} from '../../common/Utils';
import {$hidden, $id, $InnerHTML, $SetFocus, $touch, $visible} from 'simpa/lib/utils';
import authorizationService from '../../services/AuthorizationService';
import {flattenErrors, onUnrecoverableError} from '../../model/Error';

/** Login form. */
export class LoginForm extends Form {

  private readonly loginId = '';
  private readonly loginValidationMessageId = '';
  private readonly passwordId = '';
  private readonly passwordValidationMessageId = '';
  private readonly buttonLoginId = '';
  private readonly buttonCancelId = '';
  private readonly errorsId = '';
  private loginEl: HTMLInputElement;
  private passwordEl: HTMLInputElement;
  private errorsEl: HTMLDivElement;
  private previousView: IViewCallback;
  private nextView: IViewCallback;

  /** Creates a login form. */
  constructor() {
    super('nn-login-form');
  }

  /** Returns a template imported from HTML file. */
  public doTemplate(): string {
    return template;
  }

  /** Initializes all components. */
  public doInit() {
    super.doInit();
    const me = this;
    this.loginEl = $id(this.loginId) as HTMLInputElement;
    this.passwordEl = $id(this.passwordId) as HTMLInputElement;
    this.errorsEl = $id(this.errorsId) as HTMLDivElement;
    $touch(this.buttonLoginId, () => {
      me.processLogin();
    });
    $touch(this.buttonCancelId, () => {
      me.cancelLogin();
    });
  }

  /** Sets view switching callbacks for views that will be displayed after cancelling or accepting login. */
  public switchViews(prev: IViewCallback, next: IViewCallback): void {
    this.previousView = prev;
    this.nextView = next;
    $SetFocus(this.loginEl);
  }

  /** Calls authorization service when user has confirmed login. */
  private processLogin(): void {
    const me = this;
    this.hideErrorMessages();
    const credentials = this.validateForm();
    if (credentials) {
      authorizationService.login(credentials,
        (token) => {
          authorizationService.setAuthorized(token.token);
          if (me.nextView) {
            me.clearForm();
            me.nextView();
          }
        },
        (errors) => {
          me.clearForm();
          $InnerHTML(me.errorsEl, flattenErrors(errors));
          $visible(this.errorsId);
          $SetFocus(this.loginEl);
        }, onUnrecoverableError);
    }
  }

  /** Switches to previous view when user has canceled login. */
  private cancelLogin(): void {
    if (this.previousView) {
      this.clearForm();
      this.previousView();
    }
  }

  /** Validates input fields. */
  private validateForm(): LoginParams | null {
    // validate login
    const login = this.loginEl.value;
    if (login) {
      $hidden(this.loginValidationMessageId);
    } else {
      $visible(this.loginValidationMessageId);
      $SetFocus(this.loginEl);
      return null;
    }
    // validate password
    const password = this.passwordEl.value;
    if (password) {
      $hidden(this.passwordValidationMessageId);
    } else {
      $visible(this.passwordValidationMessageId);
      $SetFocus(this.passwordEl);
      return null;
    }
    // return validated credentials
    let credentials = new LoginParams();
    credentials.login = login;
    credentials.password = password;
    return credentials;
  }

  /** Clears the content of all form fields. */
  private clearForm() {
    this.loginEl.value = '';
    this.passwordEl.value = '';
    $InnerHTML(this.errorsEl, '&nbsp;');
    this.hideErrorMessages();
  }

  /** Hides all error messages. */
  private hideErrorMessages() {
    $hidden(this.loginValidationMessageId);
    $hidden(this.passwordValidationMessageId);
    $hidden(this.errorsId);
  }
}

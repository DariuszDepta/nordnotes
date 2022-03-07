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

import {Token} from '../model/Token';
import {LoginParams} from '../forms/loginForm/LoginParams';
import {configuration} from '../common/Configuration';
import {$POST} from '../common/Utils';
import {IErrorCallback, IUnrecoverableCallback} from '../model/Error';

/** Definition of login callback. */
export interface ILoginCallback {
  (token: Token): void;
}

/** Service for handling authorization. */
class AuthorizationService {
  /** Retrieves a collection of notes from server. */
  public login(credentials: LoginParams, dataCallback: ILoginCallback, errorCallback: IErrorCallback, unrecoverableCallback: IUnrecoverableCallback): void {
    fetch(configuration().API_URL + '/login', $POST(credentials))
      .then(result => result.json())
      .then(response => {
        if (response.data) {
          dataCallback(response.data);
        }
        if (response.errors) {
          errorCallback(response.errors);
        }
      })
      .catch(reason => {
        unrecoverableCallback(reason);
      });
  }

  /** Saves authorization token in local storage. */
  public setAuthorized(token: string): void {
    localStorage.setItem(configuration().AUTHORIZED_STORAGE_KEY, 'authorized');
    localStorage.setItem(configuration().TOKEN_STORAGE_KEY, token);
  }

  /** Clears authorization token from local storage. */
  public clearAuthorized(): void {
    localStorage.setItem(configuration().AUTHORIZED_STORAGE_KEY, null);
    localStorage.setItem(configuration().TOKEN_STORAGE_KEY, null);
  }

  /** Checks if current user is authorized. */
  public isAuthorized(): boolean {
    const authorized = localStorage.getItem(configuration().AUTHORIZED_STORAGE_KEY);
    return authorized && authorized === 'authorized';
  }

  /** Returns the value of the authorization header. */
  public getAuthorizationHeaderValue(): string | null {
    if (this.isAuthorized()) {
      const token = localStorage.getItem(configuration().TOKEN_STORAGE_KEY);
      if (token) {
        return 'Bearer ' + token;
      }
    }
    return null;
  }

}

/** Global authorization object. */
const authorizationService = new AuthorizationService();

/** Default exported service object. */
export default authorizationService;

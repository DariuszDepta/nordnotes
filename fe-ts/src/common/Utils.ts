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

import authorizationService from '../services/AuthorizationService';

/** Definition of callback for switching views. */
export interface IViewCallback {
  (): void;
}

/** Returns initialization object for GET requests. */
export function $GET(): RequestInit {
  let init: RequestInit = {
    method: 'GET'
  };
  if (authorizationService.isAuthorized()) {
    init.headers = {'Authorization': authorizationService.getAuthorizationHeaderValue()};
  }
  return init;
}

/** Returns initialization object for POST requests. */
export function $POST(data: Object): RequestInit {
  let init: RequestInit = {
    method: 'POST',
    cache: 'no-cache',
    credentials: 'omit',
    headers: {'Content-Type': 'application/json'},
    redirect: 'follow',
    referrer: 'no-referrer',
    body: JSON.stringify(data)
  };
  if (authorizationService.isAuthorized()) {
    init.headers = {
      'Content-Type': 'application/json',
      'Authorization': authorizationService.getAuthorizationHeaderValue()
    };
  } else {
    init.headers = {'Content-Type': 'application/json'};
  }
  return init;
}

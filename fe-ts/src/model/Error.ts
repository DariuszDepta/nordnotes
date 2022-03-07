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

import app from '../app/App';

/** Callback definition for handling errors returned by backend. */
export interface IErrorCallback {
  (errors: Error[]): void;
}

/** Callback definition for handling unrecoverable errors. */
export interface IUnrecoverableCallback {
  (reason: Object): void;
}

/** An error returned from backend. */
export class Error {
  /** Detailed error description. */
  public details: string;
}

/** Converts an array of errors into single string. */
export function flattenErrors(errors: Error[]): string {
  return errors.reduce((previousValue, currentValue) => {
    return previousValue + ' ' + currentValue.details;
  }, '');
}

/** Opens a view with unrecoverable error message build from errors. */
export function onError(errors: Error[]): void {
  app.showUnrecoverableErrorView(flattenErrors(errors));
}

/** Opens a view with unrecoverable error message build from reason object. */
export function onUnrecoverableError(reason: Object): void {
  console.log(reason);
  app.showUnrecoverableErrorView('UNRECOVERABLE ERROR: ' + reason);
}

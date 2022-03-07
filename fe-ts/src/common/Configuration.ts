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

/** Definition of the configuration attributes. */
export interface IConfiguration {
  /** URL of the backend. */
  API_URL: string;
  /** Name of the key where authorization flag is stored. */
  AUTHORIZED_STORAGE_KEY: string;
  /** Name of the key where authorization token is stored. */
  TOKEN_STORAGE_KEY: string;
}

/** Address and port for local development. */
const localHost = '0.0.0.0:8871';

/** Backend API version. */
const apiVersion = '/api/v1';

/** Configuration for production mode. */
const Production: IConfiguration = {
  API_URL: window.location.protocol + '//' + window.location.host + apiVersion,
  AUTHORIZED_STORAGE_KEY: 'authorized',
  TOKEN_STORAGE_KEY: 'token',
};

/** Configuration for development mode. */
const Development: IConfiguration = {
  API_URL: window.location.protocol + '//' + localHost + apiVersion,
  AUTHORIZED_STORAGE_KEY: 'authorized',
  TOKEN_STORAGE_KEY: 'token',
};

/** Returns configuration attributes based on current mode. */
export function configuration(): IConfiguration | null {
  if (process.env.NODE_ENV === 'production') {
    return Production;
  }
  if (process.env.NODE_ENV === 'development') {
    return Development;
  }
  return null;
}

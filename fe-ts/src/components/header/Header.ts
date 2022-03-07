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

import './Header.css';
import template from './Header.html';
import {TopTitle} from '../topTitle/TopTitle';
import {Component} from 'simpa/lib/component';
import {$id} from 'simpa/lib/utils';

/** Header component. */
export class Header extends Component {

  private readonly containerId = '';
  private readonly topTitle: TopTitle;

  /** Creates a header component. */
  constructor() {
    super('nn-header');
    this.topTitle = new TopTitle();
  }

  /** Returns a template imported from HTML file. */
  public doTemplate(): string {
    return template;
  }

  /** Creates all components. */
  public doCreate(): void {
    super.doCreate();
    this.topTitle.doCreate();
  }

  /** Builds the DOM structure. */
  public doBuild(): void {
    super.doBuild();
    const container = $id(this.containerId);
    container.appendChild(this.topTitle.componentRoot);
    this.topTitle.doBuild();
  }

  /** Initializes all components. */
  public doInit(): void {
    super.doInit();
    this.topTitle.doInit();
  }
}

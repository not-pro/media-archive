import { elements } from './elements.js';

export function showStatus(message, kind = 'info') {
  elements.status.textContent = message;
  elements.status.className = `status${kind === 'info' ? '' : ` ${kind}`}`;
  elements.status.hidden = false;
}

export function clearStatus() {
  elements.status.hidden = true;
  elements.status.textContent = '';
}

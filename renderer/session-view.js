import { elements } from './elements.js';

export function updateSession(connected) {
  elements['session-badge'].textContent = connected
    ? 'Есть сохранённая сессия'
    : 'Автовход из браузера';
  elements['session-badge'].classList.toggle('connected', connected);
}

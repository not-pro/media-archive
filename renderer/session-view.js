import { elements } from './elements.js';

export function updateSession(connected) {
  elements['session-badge'].textContent = connected ? 'Сессия подключена' : 'Без сессии';
  elements['session-badge'].classList.toggle('connected', connected);
  elements['clear-cookies'].hidden = !connected;
  elements['import-cookies'].textContent = connected
    ? 'Обновить cookies.txt'
    : 'Импорт cookies.txt';
}

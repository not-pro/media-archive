import { api, unwrap } from './api.js';
import { updateSession } from './session-view.js';
import { showStatus } from './status.js';

export async function importCookies() {
  try {
    const result = unwrap(await api.importCookies());
    updateSession(result.connected);
    if (result.connected) {
      showStatus('Сессия аккаунта сохранена в зашифрованном виде.', 'success');
    }
  } catch (error) {
    showStatus(error.message, 'error');
  }
}

export async function clearCookies() {
  if (!window.confirm('Удалить сохранённую сессию видеосервиса из приложения?')) return;
  try {
    updateSession(unwrap(await api.clearCookies()).connected);
    showStatus('Сессия удалена.', 'success');
  } catch (error) {
    showStatus(error.message, 'error');
  }
}

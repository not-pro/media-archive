import { api, unwrap } from './api.js';
import { elements } from './elements.js';
import { updateSession } from './session-view.js';
import { showStatus } from './status.js';

const importBrowserButton = elements['import-from-browser'];
const importManualButton = elements['import-manually'];
const clearSessionButton = elements['clear-session'];

function setLoading(button, loading) {
  if (loading) {
    button.disabled = true;
    button.textContent = '⋯';
    button.dataset.originalText = button.textContent;
  } else {
    button.disabled = false;
    button.textContent = button.dataset.originalText || button.textContent;
  }
}

export async function importFromBrowser() {
  setLoading(importBrowserButton, true);
  try {
    const result = unwrap(await api.importFromBrowser());
    updateSession(result.connected);
    if (result.connected) {
      showStatus(
        'Сессия подключена из браузера. Cookies сохранены в защищённом хранилище системы.',
        'success',
      );
    }
  } catch (error) {
    const code = error.code ?? '';
    if (code === 'BROWSER_NOT_FOUND' || code === 'BROWSER_ACCESS_DENIED') {
      showStatus(error.message, 'error');
    } else {
      showStatus(error.message, 'error');
    }
  } finally {
    setLoading(importBrowserButton, false);
  }
}

export async function importManually() {
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

export async function clearSession() {
  if (!window.confirm('Удалить сохранённую сессию видеосервиса из приложения?')) return;
  try {
    updateSession(unwrap(await api.clearCookies()).connected);
    showStatus('Сессия удалена.', 'success');
  } catch (error) {
    showStatus(error.message, 'error');
  }
}

export function bindSessionActions() {
  importBrowserButton.addEventListener('click', importFromBrowser);
  importManualButton.addEventListener('click', importManually);
  clearSessionButton.addEventListener('click', clearSession);
}

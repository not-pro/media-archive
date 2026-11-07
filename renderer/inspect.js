import { api, unwrap } from './api.js';
import { updateControls } from './controls.js';
import { elements } from './elements.js';
import { state } from './state.js';
import { clearStatus, showStatus } from './status.js';
import { invalidateMetadata, renderMetadata } from './video-view.js';

export async function inspectVideo(event) {
  event.preventDefault();
  if (state.downloading) return;
  invalidateMetadata();
  clearStatus();
  elements.inspect.disabled = true;
  elements.inspect.textContent = 'Проверка…';
  try {
    state.metadata = unwrap(await api.inspect(elements['video-url'].value));
    renderMetadata(state.metadata);
    showStatus('Видео найдено. Выберите качество и папку.', 'success');
  } catch (error) {
    state.metadata = null;
    showStatus(error.message, 'error');
  } finally {
    elements.inspect.textContent = 'Проверить';
    updateControls();
  }
}

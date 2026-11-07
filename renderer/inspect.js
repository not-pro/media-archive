import { api, unwrap } from './api.js';
import { updateControls } from './controls.js';
import { elements } from './elements.js';
import { state } from './state.js';
import { clearStatus, showStatus } from './status.js';
import { invalidateMetadata, renderMetadata } from './video-view.js';

let latestRequestId = 0;

export async function inspectVideo(event) {
  event.preventDefault();
  if (state.downloading || state.inspecting) return;

  const requestId = ++latestRequestId;
  const url = elements['video-url'].value;

  state.inspecting = true;
  invalidateMetadata();
  clearStatus();
  elements.inspect.textContent = 'Проверка…';
  updateControls();

  try {
    const metadata = unwrap(await api.inspect(url));
    if (requestId === latestRequestId && elements['video-url'].value === url) {
      state.metadata = metadata;
      renderMetadata(state.metadata);
      showStatus('Видео найдено. Выберите качество и папку.', 'success');
    }
  } catch (error) {
    if (requestId === latestRequestId) {
      state.metadata = null;
      showStatus(error.message, 'error');
    }
  } finally {
    if (requestId === latestRequestId) {
      state.inspecting = false;
      elements.inspect.textContent = 'Проверить';
      updateControls();
    }
  }
}

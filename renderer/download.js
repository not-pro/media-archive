import { api, unwrap } from './api.js';
import { updateControls } from './controls.js';
import { elements } from './elements.js';
import { resetProgress } from './progress-view.js';
import { state } from './state.js';
import { clearStatus, showStatus } from './status.js';

export async function startDownload() {
  if (!state.metadata || !state.directory || state.downloading) return;
  state.downloading = true;
  clearStatus();
  resetProgress();
  updateControls();
  try {
    const result = unwrap(
      await api.startDownload({
        url: elements['video-url'].value,
        height: elements.quality.value ? Number(elements.quality.value) : null,
        outputDirectory: state.directory,
        overwrite: elements.overwrite.checked,
      }),
    );
    elements.progress.value = 100;
    elements['progress-text'].textContent = 'Готово';
    elements['speed-text'].textContent = '';
    const message = result.filePath
      ? `Видео сохранено: ${result.filePath}`
      : `Видео сохранено в ${state.directory}`;
    showStatus(message, 'success');
  } catch (error) {
    showStatus(error.message, error.code === 'CANCELLED' ? 'info' : 'error');
    elements['progress-text'].textContent = error.code === 'CANCELLED' ? 'Отменено' : 'Ошибка';
  } finally {
    state.downloading = false;
    updateControls();
  }
}

export async function cancelDownload() {
  elements.cancel.disabled = true;
  elements['progress-text'].textContent = 'Отмена…';
  try {
    unwrap(await api.cancelDownload());
  } catch (error) {
    showStatus(error.message, 'error');
  } finally {
    elements.cancel.disabled = false;
  }
}

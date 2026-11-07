import { elements } from './elements.js';
import { formatDuration } from './format.js';
import { state } from './state.js';

export function resetProgress() {
  elements['progress-area'].hidden = false;
  elements.progress.value = 0;
  elements['progress-text'].textContent = 'Подготовка…';
  elements['speed-text'].textContent = '';
}

export function renderProgress(progress) {
  if (!state.downloading) return;
  if (progress.percent === null) {
    elements.progress.removeAttribute('value');
    elements['progress-text'].textContent = 'Скачивание…';
  } else {
    elements.progress.value = progress.percent;
    elements['progress-text'].textContent = `${progress.percent.toFixed(1)}%`;
  }
  const speed = progress.speed === null ? '' : `${(progress.speed / 1024 / 1024).toFixed(1)} МБ/с`;
  const eta = progress.eta === null ? '' : ` · осталось ${formatDuration(progress.eta)}`;
  elements['speed-text'].textContent = `${speed}${eta}`;
}

import { elements } from './elements.js';
import { formatDuration } from './format.js';
import { state } from './state.js';

function renderQualities(qualities) {
  elements.quality.replaceChildren();
  const best = document.createElement('option');
  best.value = '';
  best.textContent = 'Лучшее доступное';
  elements.quality.append(best);
  for (const quality of qualities) {
    const option = document.createElement('option');
    option.value = String(quality.height);
    option.textContent = `До ${quality.label}`;
    elements.quality.append(option);
  }
}

export function invalidateMetadata() {
  state.metadata = null;
  elements['video-card'].hidden = true;
  elements['download-settings'].hidden = true;
  elements.download.disabled = true;
}

export function renderMetadata(metadata) {
  elements['video-title'].textContent = metadata.title;
  const details = [metadata.uploader, formatDuration(metadata.duration)]
    .filter(Boolean)
    .join(' · ');
  elements['video-details'].textContent = details;
  if (metadata.thumbnail) {
    elements.thumbnail.src = metadata.thumbnail;
    elements.thumbnail.hidden = false;
  } else {
    elements.thumbnail.hidden = true;
    elements.thumbnail.removeAttribute('src');
  }
  renderQualities(metadata.qualities);
  elements['video-card'].hidden = false;
  elements['download-settings'].hidden = false;
}

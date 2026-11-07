import { elements } from './elements.js';
import { state } from './state.js';

export function updateControls() {
  elements.download.disabled = !state.metadata || !state.directory || state.downloading;
  elements.inspect.disabled = state.downloading;
  elements['video-url'].disabled = state.downloading;
  elements['choose-directory'].disabled = state.downloading;
  elements.quality.disabled = state.downloading;
  elements.overwrite.disabled = state.downloading;
  elements['import-cookies'].disabled = state.downloading;
  elements['clear-cookies'].disabled = state.downloading;
  elements.cancel.hidden = !state.downloading;
  elements.download.hidden = state.downloading;
  elements['open-directory'].disabled = !state.directory || state.downloading;
}

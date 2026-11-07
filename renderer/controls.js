import { elements } from './elements.js';
import { state } from './state.js';

export function updateControls() {
  const busy = state.downloading || state.inspecting;
  elements.download.disabled = !state.metadata || !state.directory || state.downloading;
  elements.inspect.disabled = busy;
  elements['video-url'].disabled = busy;
  elements['choose-directory'].disabled = state.downloading;
  elements.quality.disabled = state.downloading;
  elements.overwrite.disabled = state.downloading;
  elements['import-cookies'].disabled = busy;
  elements['clear-cookies'].disabled = busy;
  elements.cancel.hidden = !state.downloading;
  elements.download.hidden = state.downloading;
  elements['open-directory'].disabled = !state.directory || state.downloading;
}

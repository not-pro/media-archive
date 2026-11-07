import { api, unwrap } from './api.js';
import { updateControls } from './controls.js';
import { elements } from './elements.js';
import { state } from './state.js';
import { showStatus } from './status.js';

export async function chooseDirectory() {
  try {
    const result = unwrap(await api.chooseDirectory());
    if (result.directory) {
      state.directory = result.directory;
      elements.directory.value = state.directory;
    }
    updateControls();
  } catch (error) {
    showStatus(error.message, 'error');
  }
}

export async function openDirectory() {
  if (!state.directory) return;
  try {
    unwrap(await api.openDirectory(state.directory));
  } catch (error) {
    showStatus(error.message, 'error');
  }
}

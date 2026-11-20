import { api, unwrap } from './api.js';
import { updateControls } from './controls.js';
import { chooseDirectory, openDirectory } from './directory-actions.js';
import { cancelDownload, startDownload } from './download.js';
import { elements } from './elements.js';
import { inspectVideo } from './inspect.js';
import { renderProgress } from './progress-view.js';
import { clearCookies, importCookies } from './session-actions.js';
import { updateSession } from './session-view.js';
import { showStatus } from './status.js';
import { invalidateMetadata } from './video-view.js';

const form = elements['url-form'];
form.addEventListener('submit', inspectVideo);
elements['video-url'].addEventListener('paste', () => {
  setTimeout(() => form.requestSubmit(), 0);
});
elements['video-url'].addEventListener('input', invalidateMetadata);
elements['choose-directory'].addEventListener('click', chooseDirectory);
elements.download.addEventListener('click', startDownload);
elements.cancel.addEventListener('click', cancelDownload);
elements['open-directory'].addEventListener('click', openDirectory);
api.onProgress(renderProgress);

api
  .getSessionStatus()
  .then((result) => updateSession(unwrap(result).connected))
  .catch((error) => showStatus(error.message, 'error'));

elements['import-manually'].addEventListener('click', importCookies);
elements['clear-session'].addEventListener('click', clearCookies);
updateControls();

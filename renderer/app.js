import { api, unwrap } from './api.js';
import { updateControls } from './controls.js';
import { chooseDirectory, openDirectory } from './directory-actions.js';
import { cancelDownload, startDownload } from './download.js';
import { elements } from './elements.js';
import { inspectVideo } from './inspect.js';
import { renderProgress } from './progress-view.js';
import { bindSessionActions } from './session-select.js';
import { updateSession } from './session-view.js';
import { showStatus } from './status.js';
import { invalidateMetadata } from './video-view.js';

elements['url-form'].addEventListener('submit', inspectVideo);
elements['video-url'].addEventListener('input', invalidateMetadata);
elements['choose-directory'].addEventListener('click', chooseDirectory);
elements.download.addEventListener('click', startDownload);
elements.cancel.addEventListener('click', cancelDownload);
elements['open-directory'].addEventListener('click', openDirectory);
api.onProgress(renderProgress);

api
  .listen('session:ready', (event) => {
    updateSession(event.payload);
  })
  .then(() => {
    api
      .getSessionStatus()
      .then((result) => updateSession(unwrap(result)))
      .catch((error) => showStatus(error.message, 'error'));
  });

bindSessionActions();
updateControls();

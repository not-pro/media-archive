const invoke = window.__TAURI__.core.invoke;
const listen = window.__TAURI__.event.listen;

async function wrap(request) {
  try {
    return { ok: true, value: await request };
  } catch (error) {
    return {
      ok: false,
      error: {
        code: error?.code ?? 'UNKNOWN',
        message: error?.message ?? String(error),
      },
    };
  }
}

export const api = {
  getSessionStatus: () => wrap(invoke('session_status')),
  importCookies: () => wrap(invoke('session_import')),
  clearCookies: () => wrap(invoke('session_clear')),
  inspect: (url) => wrap(invoke('video_inspect', { url })),
  chooseDirectory: () => wrap(invoke('directory_choose')),
  startDownload: (options) => wrap(invoke('download_start', { options })),
  cancelDownload: () => wrap(invoke('download_cancel')),
  openDirectory: (directory) => wrap(invoke('directory_open', { directory })),
  onProgress: (callback) => {
    const unlisten = listen('download:progress', (event) => callback(event.payload));
    return () => unlisten.then((remove) => remove());
  },
};

export function unwrap(result) {
  if (!result?.ok) {
    const error = new Error(result?.error?.message ?? 'Неизвестная ошибка.');
    error.code = result?.error?.code;
    throw error;
  }
  return result.value;
}

import { uploadImage } from './api.js';
import { createLocalId } from './id.js';
import { showToast } from './toast.js';

const STATUS_LABEL = {
  uploading: 'アップロード中…',
  ready: '編集可能',
  editing: '編集中',
  saving: '保存中…',
  saved: '保存済み',
  error: 'エラー',
};

export function initGallery({ store, fileInputEl, dropzoneEl, listEl, emptyHintEl, onOpenSession }) {
  fileInputEl.addEventListener('change', (evt) => {
    handleFiles(evt.target.files);
    fileInputEl.value = ''; // 同じファイルを連続選択できるようにリセット
  });

  dropzoneEl.addEventListener('click', () => fileInputEl.click());

  dropzoneEl.addEventListener('dragover', (evt) => {
    evt.preventDefault();
    dropzoneEl.classList.add('is-dragover');
  });
  dropzoneEl.addEventListener('dragleave', () => {
    dropzoneEl.classList.remove('is-dragover');
  });
  dropzoneEl.addEventListener('drop', (evt) => {
    evt.preventDefault();
    dropzoneEl.classList.remove('is-dragover');
    handleFiles(evt.dataTransfer.files);
  });

  listEl.addEventListener('click', (evt) => {
    const card = evt.target.closest('[data-local-id]');
    if (!card || card.disabled) return;
    onOpenSession(card.dataset.localId);
  });

  store.subscribe((state) => render(state));
  render(store.getState());

  function handleFiles(fileList) {
    const files = Array.from(fileList || []).filter((f) => f.type.startsWith('image/'));
    for (const file of files) {
      const localId = createLocalId();
      const thumbUrl = URL.createObjectURL(file);

      store.createSession(localId, { thumbUrl, file });

      uploadImage(file)
        .then(({ sessionId, imageId }) => {
          store.updateSession(localId, { sessionId, imageId, status: 'ready' });
        })
        .catch((err) => {
          store.updateSession(localId, { status: 'error', error: err.message });
          showToast({
            type: 'danger',
            message: `${file.name}: アップロードに失敗しました (${err.message})`,
            code: err.code,
          });
        });
    }
  }

  function render(state) {
    listEl.innerHTML = '';
    emptyHintEl.hidden = state.sessions.size > 0;

    for (const session of state.sessions.values()) {
      const card = document.createElement('button');
      card.type = 'button';
      card.className = 'thumb-card';
      card.dataset.localId = session.localId;
      card.disabled = session.status === 'uploading' || session.status === 'error';

      const img = document.createElement('img');
      img.src = session.thumbUrl;
      img.alt = session.file ? session.file.name : '画像';
      card.appendChild(img);

      const badge = document.createElement('span');
      badge.className = `status-badge status-${session.status}`;
      badge.textContent = session.status === 'error'
        ? (session.error || STATUS_LABEL.error)
        : (STATUS_LABEL[session.status] || session.status);
      card.appendChild(badge);

      listEl.appendChild(card);
    }
  }
}
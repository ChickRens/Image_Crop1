import { getPreviewImage, segmentImage, undoEdit, redoEdit, saveImage, getCompletedImage } from './api.js';
import { createCanvasController } from './canvas.js';
import { showToast } from './toast.js';

/**
 * 画面には常に1枚分の大きな編集エリア（canvas+ツールバー）だけがあり、
 * 上部のストリップ（strip.js）でどのセッションを表示するかを切り替える。
 *
 * image_id はスナップショットのポインタで、segment/undo/redo のたびに
 * 新しい image_id が発行される。実質的にversionの役割を果たすが、
 * 複数の変更系リクエストが同時に飛んだ場合「どちらのimage_idが
 * 最終的に正しいか」はフロント側では判定できない。そのため
 * session.busy をロックとして使い、変更系リクエスト
 * (segment / undo / redo / save) を同時に2つ以上飛ばさないようにする。
 *
 * また、ストリップで別の画像に切り替えた直後に、前の画像への
 * 非同期処理（segment等）の応答が遅れて届くことがある。その場合
 * 「呼び出し時点の localId」と「今表示している currentLocalId」を
 * 比較し、一致しなければ描画を破棄する。これをしないと、切り替え後の
 * 画面に前の画像の結果が紛れ込むレースコンディションが起きる。
 *
 * undo/redoはサーバーがポイント一覧を返さない(image_idだけを返す)ため、
 * マーカー表示用の points 配列はクライアント側で
 * 「1回のundo = 直前に追加した1点を取り除く」
 * 「1回のredo = 取り除いた1点を戻す」
 * という前提でベストエフォートに同期する。実際に表示される画像自体は
 * 常に get-preview で取得し直すので、この前提が崩れてもマスクの表示は
 * 正しいままだが、マーカーの見た目だけはズレ得る。
 */
export function initEditor({ store, elements }) {
  const {
    canvasEl,
    modeToggleBtn,
    undoBtn,
    redoBtn,
    saveBtn,
    statusEl,
    emptyHintEl,
    editorSectionEl,
  } = elements;

  let currentLocalId = null;

  const controller = createCanvasController(canvasEl, {
    onPointAdd: handlePointAdd,
  });

  store.subscribe((state) => {
    if (currentLocalId) syncToolbar(state.sessions.get(currentLocalId));
  });

  modeToggleBtn.addEventListener('click', () => {
    const next = modeToggleBtn.dataset.mode === 'positive' ? 'negative' : 'positive';
    applyMode(next);
  });

  undoBtn.addEventListener('click', () => runHistory(undoEdit, popLastPoint));
  redoBtn.addEventListener('click', () => runHistory(redoEdit, restoreLastPoint));
  saveBtn.addEventListener('click', () => runSave());

  applyMode('positive');

  async function open(localId) {
    currentLocalId = localId;
    const session = store.getSession(localId);
    if (!session || !session.imageId) {
      showToast({ type: 'warning', message: 'この画像はまだアップロードが完了していません' });
      return;
    }

    emptyHintEl.hidden = true;
    editorSectionEl.hidden = false;

    setStatus('画像を読み込み中…');
    syncToolbar(session);

    try {
      const url = await getPreviewImage(session.imageId);
      if (localId !== currentLocalId) return; // その間に別画像へ切り替えられていたら破棄

      store.updateSession(localId, { fullUrl: url, status: 'editing' });
      const { width, height } = await controller.loadBaseImage(url);
      if (localId !== currentLocalId) return;

      store.updateSession(localId, { naturalWidth: width, naturalHeight: height });
      controller.setPoints(session.points);
      controller.loadOriginalImage(session.thumbUrl); // 元画像の薄い表示（既にあるBlob URLを再利用、通信不要）
      setStatus('');
    } catch (err) {
      setStatus('');
      showToast({ type: 'danger', message: `読み込みに失敗しました: ${err.message}`, code: err.code });
    }
  }

  function applyMode(mode) {
    modeToggleBtn.dataset.mode = mode;
    modeToggleBtn.textContent = mode === 'positive' ? '➕ 含める（タップで追加）' : '➖ 除外する（タップで追加）';
    controller.setPointMode(mode);
  }

  function handlePointAdd(point) {
    if (!currentLocalId) return;
    const session = store.getSession(currentLocalId);
    if (!session) return;

    if (session.busy) {
      // 処理中の変更系リクエストと重ねて送らない。タップ自体を無視する。
      setStatus('処理中はポイントを追加できません。完了までお待ちください');
      return;
    }

    // 新しくポイントを打ったらredoの前提が崩れるのでredoStackは破棄する
    const points = [...session.points, point];
    store.updateSession(currentLocalId, { points, redoStack: [] });
    controller.setPoints(points);
    runSegment(point);
  }

  async function runSegment(point) {
    const localId = currentLocalId;
    const session = store.getSession(localId);
    if (!session || session.busy) return;

    store.updateSession(localId, { busy: true });
    setStatus('セグメンテーション実行中…');
    try {
      const { imageId } = await segmentImage(session.sessionId, point);
      const url = await getPreviewImage(imageId);

      store.updateSession(localId, { imageId, fullUrl: url });

      if (localId === currentLocalId) {
        await controller.loadBaseImage(url);
        setStatus('');
      }
    } catch (err) {
      if (localId === currentLocalId) {
        setStatus('');
        showToast({ type: 'danger', message: `セグメンテーションに失敗しました: ${err.message}`, code: err.code });
      }
    } finally {
      store.updateSession(localId, { busy: false });
    }
  }

  function popLastPoint(session) {
    if (session.points.length === 0) return;
    const points = session.points.slice(0, -1);
    const removed = session.points[session.points.length - 1];
    const redoStack = [...session.redoStack, removed];
    store.updateSession(session.localId, { points, redoStack });
    controller.setPoints(points);
  }

  function restoreLastPoint(session) {
    if (session.redoStack.length === 0) return;
    const redoStack = session.redoStack.slice(0, -1);
    const restored = session.redoStack[session.redoStack.length - 1];
    const points = [...session.points, restored];
    store.updateSession(session.localId, { points, redoStack });
    controller.setPoints(points);
  }

  async function runHistory(apiFn, localPointSync) {
    const localId = currentLocalId;
    const session = store.getSession(localId);
    if (!session || session.busy) return;

    store.updateSession(localId, { busy: true });
    setStatus('処理中…');
    try {
      const { imageId } = await apiFn(session.sessionId);
      const url = await getPreviewImage(imageId);

      store.updateSession(localId, { imageId, fullUrl: url });

      if (localId === currentLocalId) {
        localPointSync(store.getSession(localId));
        await controller.loadBaseImage(url);
        setStatus('');
      }
    } catch (err) {
      if (localId === currentLocalId) {
        setStatus('');
        showToast({ type: 'danger', message: `失敗しました: ${err.message}`, code: err.code });
      }
    } finally {
      store.updateSession(localId, { busy: false });
    }
  }

  function syncToolbar(session) {
    const busy = !session || session.busy;
    undoBtn.disabled = busy;
    redoBtn.disabled = busy;
    saveBtn.disabled = busy;
  }

  async function runSave() {
    const localId = currentLocalId;
    const session = store.getSession(localId);
    if (!session || session.busy) return;

    store.updateSession(localId, { busy: true, status: 'saving' });
    setStatus('保存用のフル画質版を作成中…');
    try {
      const { imageId: completedImageId } = await saveImage(session.sessionId);
      setStatus('ダウンロード準備中…');
      const blob = await getCompletedImage(completedImageId);

      if (localId === currentLocalId) {
        store.updateSession(localId, { status: 'saved' });
        setStatus('');
        showToast({ type: 'success', message: '保存が完了しました' });
        triggerDownload(blob, session.file ? session.file.name : null);
      }
    } catch (err) {
      if (localId === currentLocalId) {
        setStatus('');
        showToast({ type: 'danger', message: `保存に失敗しました: ${err.message}`, code: err.code });
      }
      store.updateSession(localId, { status: 'ready' }); // 失敗したら状態を戻す
    } finally {
      store.updateSession(localId, { busy: false });
    }
  }

  function triggerDownload(blob, originalFileName) {
    const url = URL.createObjectURL(blob);
    const baseName = originalFileName ? originalFileName.replace(/\.[^./\\]+$/, '') : 'image';
    const a = document.createElement('a');
    a.href = url;
    a.download = `${baseName}_cropped.png`;
    document.body.appendChild(a);
    a.click();
    a.remove();
    // ダウンロード開始直後に取り消すとブラウザによっては失敗するので少し待つ
    setTimeout(() => URL.revokeObjectURL(url), 1000);
  }

  function setStatus(text) {
    statusEl.textContent = text;
  }

  return { open };
}

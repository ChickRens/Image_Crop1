/**
 * 画面上部からスライドインするトースト通知。
 *
 * type ごとに tokens.css の色変数とアイコンを対応させる。
 * 全種類とも一定時間で自動的に消える。消えるタイミングが分かるよう、
 * カード背景下端にカウントダウンの帯（残り時間に応じて縮む）を表示する。
 */

/**
 * mask-image をインラインスタイルとして直接設定する。
 *
 * CSSカスタムプロパティ(--icon-src)にurl()を入れて外部スタイルシート側の
 * mask-image: var(--icon-src) で使うと、そのURLは「変数を定義した場所」
 * ではなく「varを使っている側のスタイルシート」を基準に解決されてしまう
 * （ブラウザの仕様上の挙動）。ここでは css/style.css からの相対パスとして
 * 解決されてしまい css/assets/... を探しにいくバグの原因になっていた。
 * インラインスタイルとして直接設定すれば、HTMLページ(index.html)を基準に
 * 解決されるので、ルート直下の assets/ を正しく参照できる。
 */
function applyMaskIcon(el, path) {
  const url = `url("${path}")`;
  el.style.setProperty('-webkit-mask-image', url);
  el.style.setProperty('mask-image', url);
}

const TYPE_CONFIG = {
  info: { colorVar: '--info', icon: '/assets/info.svg' },
  warning: { colorVar: '--warning', icon: '/assets/warning.svg' },
  danger: { colorVar: '--danger', icon: '/assets/danger.svg' },
  success: { colorVar: '--success', icon: '/assets/success.svg' },
};

const AUTO_DISMISS_MS = {
  info: 4000,
  warning: 6000,
  danger: 6000, // エラーコードを確認する時間を考慮し、他よりやや長め
  success: 4000,
};

function getContainer() {
  let container = document.getElementById('toast-container');
  if (!container) {
    container = document.createElement('div');
    container.id = 'toast-container';
    container.className = 'toast-container';
    container.setAttribute('aria-live', 'assertive');
    container.setAttribute('aria-atomic', 'true');
    document.body.prepend(container);
  }
  return container;
}

/**
 * @param {Object} opts
 * @param {'info'|'warning'|'danger'|'success'} opts.type
 * @param {string} opts.message - ユーザー向けの説明文
 * @param {string|null} [opts.code] - バックエンドのエラーコード等。あれば表示する
 */
export function showToast({ type = 'info', message, code = null }) {
  const container = getContainer();
  const config = TYPE_CONFIG[type] || TYPE_CONFIG.info;

  const card = document.createElement('div');
  card.className = `toast-card toast-${type}`;
  card.style.setProperty('--toast-accent', `var(${config.colorVar})`);

  const icon = document.createElement('span');
  icon.className = 'toast-icon';
  applyMaskIcon(icon, config.icon);
  card.appendChild(icon);

  const body = document.createElement('div');
  body.className = 'toast-body';

  const messageEl = document.createElement('p');
  messageEl.className = 'toast-message';
  messageEl.textContent = message;
  body.appendChild(messageEl);

  if (code) {
    const codeEl = document.createElement('p');
    codeEl.className = 'toast-code';
    codeEl.textContent = `code: ${code}`;
    body.appendChild(codeEl);
  }
  card.appendChild(body);

  const closeBtn = document.createElement('button');
  closeBtn.type = 'button';
  closeBtn.className = 'toast-close';
  closeBtn.setAttribute('aria-label', '閉じる');
  const closeIcon = document.createElement('span');
  closeIcon.className = 'toast-close-icon';
  applyMaskIcon(closeIcon, '/assets/X.svg');
  closeBtn.appendChild(closeIcon);
  card.appendChild(closeBtn);

  container.appendChild(card);

  const autoDismissMs = AUTO_DISMISS_MS[type] ?? 4000;
  if (autoDismissMs > 0) {
    const progress = document.createElement('div');
    progress.className = 'toast-progress';
    progress.style.animationDuration = `${autoDismissMs}ms`;
    card.appendChild(progress);
  }

  // 追加した直後だとtransitionが発火しないブラウザがあるので1フレーム待ってからクラスを付ける
  requestAnimationFrame(() => {
    card.classList.add('is-visible');
  });

  function dismiss() {
    card.classList.remove('is-visible');
    card.classList.add('is-leaving');
    card.addEventListener('transitionend', () => card.remove(), { once: true });
  }

  closeBtn.addEventListener('click', dismiss);

  if (autoDismissMs > 0) {
    setTimeout(dismiss, autoDismissMs);
  }

  return { dismiss };
}

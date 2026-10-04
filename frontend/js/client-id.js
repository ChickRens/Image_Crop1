/**
 * client-id: ユーザーを識別するためのUUID。
 *
 * - 1画像(1セッション)ごとではなく、**1ユーザー(=1ブラウザ)につき1つ**。
 * - 初回アクセス時に1度だけ発行して localStorage に保存し、リロードしても
 *   同じ値を使い続ける（リロードのたびに再発行しない）。同じブラウザの別タブ
 *   からも同じ値が見える。
 * - バックエンドへの各リクエストのheader `client-id` として送る（js/api.js）。
 *
 * 注意: `localId`(UI用, js/id.js)とは別物。localId は画面内のセッション区別用で
 * リロードで消えてよい。client-id はユーザーを指すので、消えずに残る必要がある。
 */

const STORAGE_KEY = 'goodbye-background:client-id';

// 保存済みの値が壊れていた(手で編集された等)場合に、そのまま送ってサーバーに
// 弾かれないよう、UUIDの形式かどうかを確かめてから使う。
const UUID_PATTERN = /^[0-9a-f]{8}-[0-9a-f]{4}-[1-8][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i;

// localStorage が使えない環境(プライベートブラウズ等で例外になる場合)でも、
// 少なくともページを開いている間は同じ値を返せるようメモリにも持っておく。
let cachedId = null;

/**
 * UUID v4 を生成する。
 *
 * crypto.randomUUID() はセキュアコンテキスト(https:// か localhost)限定で、
 * LAN内の http:// にiPhoneからアクセスすると存在しない(README「セキュアコンテキスト
 * の制約について」参照)。そのため、使えるときだけ使い、無ければ
 * getRandomValues（こちらは非セキュアコンテキストでも使える）から組み立てる。
 */
function generateUuidV4() {
  const c = globalThis.crypto;
  if (c && typeof c.randomUUID === 'function') {
    return c.randomUUID();
  }

  const bytes = new Uint8Array(16);
  if (c && typeof c.getRandomValues === 'function') {
    c.getRandomValues(bytes);
  } else {
    // 乱数APIが全く無い環境の最終手段。client-idは識別子であって秘密の値では
    // ないので、この場合は暗号学的な強さより「動くこと」を優先する。
    for (let i = 0; i < bytes.length; i += 1) {
      bytes[i] = Math.floor(Math.random() * 256);
    }
  }

  // RFC 4122: version(4) と variant(10xx) のビットを立てる。
  // これを怠るとUUIDとしてパースできず、サーバー側で弾かれ得る。
  bytes[6] = (bytes[6] & 0x0f) | 0x40;
  bytes[8] = (bytes[8] & 0x3f) | 0x80;

  const hex = Array.from(bytes, (b) => b.toString(16).padStart(2, '0')).join('');
  return `${hex.slice(0, 8)}-${hex.slice(8, 12)}-${hex.slice(12, 16)}-${hex.slice(16, 20)}-${hex.slice(20)}`;
}

function readStoredId() {
  try {
    const value = window.localStorage.getItem(STORAGE_KEY);
    return value && UUID_PATTERN.test(value) ? value : null;
  } catch {
    return null; // ストレージへのアクセスが禁止されている環境
  }
}

function writeStoredId(id) {
  try {
    window.localStorage.setItem(STORAGE_KEY, id);
  } catch {
    /* 保存できなくても、メモリ上の値でこのページの間は動く */
  }
}

/**
 * このユーザーの client-id を返す。初回だけ発行し、以降は同じ値を返す。
 */
export function getClientId() {
  if (cachedId) return cachedId;

  let id = readStoredId();
  if (!id) {
    id = generateUuidV4();
    writeStoredId(id);
    // 別タブがほぼ同時に初回発行していた場合に、保存されている方の値に揃える
    id = readStoredId() ?? id;
  }

  cachedId = id;
  return id;
}

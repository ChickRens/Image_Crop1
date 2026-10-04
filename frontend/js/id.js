/**
 * UIキー用のローカルIDを生成する。
 *
 * crypto.randomUUID() はセキュアコンテキスト(https:// または localhost)
 * でしか使えない。LAN内に http:// でホストしてiPhone(Safari)からアクセス
 * すると crypto.randomUUID が存在せず TypeError で処理が止まるため、
 * ここでは使わない。localId は画面内でセッションを区別できれば十分で、
 * 暗号学的な安全性(推測不可能性)は不要なので、この用途には過剰な要件。
 */
export function createLocalId() {
  const random = Math.random().toString(36).slice(2, 10);
  const timestamp = Date.now().toString(36);
  return `local-${timestamp}-${random}`;
}

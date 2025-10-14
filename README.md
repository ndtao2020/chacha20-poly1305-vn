# chacha-poly-wasm-web

[![npm](https://img.shields.io/npm/v/chacha-poly-wasm-web)](https://www.npmjs.com/package/chacha-poly-wasm-web)

### 🛠️ Installing `wasm-pack`

```
cargo install wasm-pack
```

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build --target web
```

### 🎁 Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```

## Usage

```js
import init, { XChaCha20Poly1305 } from "chacha-poly-wasm-web";

const NONCE_LENGTH = 24;
const SECRET_LENGTH = 32;

const Z = "ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz0123456789@#";

const random = (characters, length) => {
    let result = "";
    const charactersLength = characters.length;
    for (let i = 0; i < length; i++) {
        result += characters.charAt(Math.floor(Math.random() * charactersLength));
    }
    return result;
};

const encoder = new TextEncoder();
const decoder = new TextDecoder();

init().then(() => {

    const secrectKey = random(Z, SECRET_LENGTH);
    const secrectBytes = encoder.encode(secrectKey)

    console.log("secrect key: ", secrectKey);

    const xchacha20 = new XChaCha20Poly1305(secrectKey);

    const nonceKey = random(Z, NONCE_LENGTH);
    const data = random(Z, 50);

    console.log("nonce key: ", nonceKey);
    console.log("data: ", data);

    // ========================= [Encrypt] =========================

    const encrypted = xchacha20.encrypt(encoder.encode(nonceKey), encoder.encode(data))

    console.log("encrypted: ", decoder.decode(encrypted));

    // ========================= [Decrypt] =========================

    const decrypted = xchacha20.decrypt(encoder.encode(nonceKey), encrypted)

    const decryptedStr = decoder.decode(decrypted)

    console.log("decrypted: ", decryptedStr);

    // ========================= [Verification] =========================
    if (data === decryptedStr) {
        console.log("Ok !");
    }
});
```

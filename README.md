# chacha20-poly1305-vn
[![npm](https://img.shields.io/npm/v/chacha20-poly1305-vn)](https://www.npmjs.com/package/chacha20-poly1305-vn)

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
import init, { XChaCha20Poly1305 } from "chacha20-poly1305-vn";

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

const textEncoder = new TextEncoder();
const textDecoder = new TextDecoder();

init().then(() => {

    const secrectKey = random(Z, SECRET_LENGTH);
    const secrectBytes = textEncoder.encode(secrectKey)

    console.log("secrect key: ", secrectKey);

    const xchacha20 = new XChaCha20Poly1305(secrectKey);

    const nonceKey = random(Z, NONCE_LENGTH);
    const data = random(Z, 50);

    console.log("nonce key: ", nonceKey);
    console.log("data: ", data);

    // ========================= [Encrypt] =========================

    const encryptedBytes = xchacha20.encrypt(textEncoder.encode(nonceKey), textEncoder.encode(data))

    const encryptedStr = textDecoder.decode(encryptedBytes)

    console.log("encrypted: ", encryptedStr);

    // ========================= [Decrypt] =========================

    const decryptedBytes = xchacha20.decrypt(textEncoder.encode(nonceKey), encryptedBytes)

    const decryptedStr = textDecoder.decode(decryptedBytes)

    console.log("decrypted: ", decryptedStr);

    // ========================= [Verification] =========================
    if (data === decryptedStr) {
        console.log("Ok !");
    }
});
```

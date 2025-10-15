# chacha-poly-wasm-web

[![npm](https://img.shields.io/npm/v/chacha-poly-wasm-web)](https://www.npmjs.com/package/chacha-poly-wasm-web)

## Installing `wasm-bindgen-cli`

```sh
cargo install wasm-bindgen-cli
cargo install wasm-opt --locked
```

## Building via `wasm-bindgen-cli`

* bundler: (produces code for usage with bundlers like Webpack)
* web: (directly loadable in a web browser)
* nodejs: (loadable via require as a CommonJS Node.js module)
* deno: (usable as a Deno module)
* no-modules: (like the web target but doesn't use ES Modules).

```sh
chmod +x build.sh
./build.sh
```

### Publish to NPM

```sh
cd pkg && npm publish
```

## Usage

```ts
import instantiate, { XChaCha20Poly1305 } from 'chacha-poly-wasm-web'

const encrypt = async (secret: Uint8Array, nonce: Uint8Array, data: Uint8Array): Promise<Uint8Array | null> => {
  if (typeof WebAssembly !== 'undefined') {
    let xchacha20: XChaCha20Poly1305 | null = null
    try {
      await instantiate()
      xchacha20 = new XChaCha20Poly1305(secret, nonce)
      return xchacha20.encrypt(data)
    } catch (err) {
      console.error(err)
    } finally {
      if (xchacha20) {
        xchacha20.free()
      }
    }
  }
  return null
}

const decrypt = async (secret: Uint8Array, nonce: Uint8Array, encrypted_data: Uint8Array): Promise<Uint8Array | null> => {
  if (typeof WebAssembly !== 'undefined') {
    let xchacha20: XChaCha20Poly1305 | null = null
    try {
      await instantiate()
      xchacha20 = new XChaCha20Poly1305(secret, nonce)
      return xchacha20.decrypt(encrypted_data)
    } catch (err) {
      console.error(err)
    } finally {
      if (xchacha20) {
        xchacha20.free()
      }
    }
  }
  return null
}
```

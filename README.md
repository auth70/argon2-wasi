# argon2-wasi

This wraps the Rust crate [`argon2`](https://crates.io/crates/argon2) in a very simple WASI (or commandline) compatible program.

Compatible with [`workerd`](https://github.com/cloudflare/workerd) / CloudFlare Workers' [WASI support](https://blog.cloudflare.com/announcing-wasi-on-workers/). You need to run the worker in unbounded mode for it to not time out.

[`bcrypt-wasi`](https://github.com/lufsorg/bcrypt-wasi) provides an identical API for `bcrypt`.

## Usage

Simple example:

```ts
import { WASI } from "@cloudflare/workers-wasi";
import argon2 from './argon2-wasi.wasm';

export async function invoke(args: string[]) {
  const stdout = new TransformStream();
  const stderr = new TransformStream();
  const wasi = new WASI({
    args: ["argon2-wasi.wasm", ...args],
    stdout: stdout.writable,
    stderr: stderr.writable,
  });
  const instance = new WebAssembly.Instance(argon2, {
    wasi_snapshot_preview1: wasi.wasiImport,
  });
  await wasi.start(instance);
  const errors = await stderr.readable.getReader().read();
  const errorsValue = new TextDecoder().decode(errors.value);
  if (errorsValue) {
    console.error('[invoke] stderr: ', errorsValue);
    throw new Error(errorsValue);
  }
  const ret = await stdout.readable.getReader().read();
  const retValue = new TextDecoder().decode(ret.value);
  return retValue.trim();
}

export async function argon2Hash(password: string): Promise<string> {
  return await invoke(["hash", password]);
}

export async function argon2Verify(password: string, hash: string): Promise<boolean> {
  return await invoke(["verify", password, hash]) === "true";
}
```

Then just use `await argon2Hash('somepwd');` or `await argon2Verify('somepwd', '$argon2id$v..')`

## License

Same license as for the `argon2` crate; licensed under either of:

 * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
 * [MIT license](http://opensource.org/licenses/MIT)

at your option.


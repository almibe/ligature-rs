# wander-wasm

This project exposes functionality from the Rust implementation of Wander to WASM and JS runtimes thanks to wasm-bindgen and wasm-pack.
It can be used by people with no knowledge or interest in Rust or WASM since it is published to NPM and has a purely JavaScript or TypeScript interface available.

## Using

TODO

## Developing

If you are interested in building this project, see https://rustwasm.github.io/ for information on setting up a development environment.

## Common Commands for Developers

```bash
wasm-pack build
wasm-pack build --target web
wasm-pack test --headless --firefox
```

## Publishing

To publish run `wasm-pack build` and then run (assuming you have https://deno.land installed):

```bash
deno run --allow-all ./merge_template.ts
```

This will update the package.json.
From there just publish to npm as normal and update the template accordingly.

To target JS directly instead of Wasm see https://rustwasm.github.io/wasm-bindgen/examples/wasm2js.html.

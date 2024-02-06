# Word Clock

This is a multi language wordclock application
implemented in yew & gloo / rust / wasm.

## How to develop

[trunk](https://trunkrs.dev/) is used as bundler, dev server etc.

Turnaround development

```
trunk server --open
```

Package into a cdn

```
trunk build
```

generates the output to to `dist` while the rust wasm output is located at
`target/...´


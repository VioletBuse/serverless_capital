
### Implemented WinterCG Minimum Common Api

#### Interfaces:

- [ ] AbortController
- [ ] AbortSignal
- [ ] Blob
- [ ] ByteLengthQueuingStrategy
- [ ] CompressionStream
- [ ] CountQueuingStrategy
- [ ] Crypto
- [ ] CryptoKey
- [ ] DecompressionStream
- [ ] DOMException
- [ ] Event
- [ ] EventTarget
- [ ] File
- [ ] FormData
  - The FormData constructor optionally takes `HTMLFormElement` and `HTMLElement` as parameters.
    - TODO: Figure out what implementations without DOM support should do here. Node.js and Deno throw if the first parameter is not undefined but ignore the second parameter. Cloudflare Workers ignores all parameters.
- [ ] Headers
- [ ] ReadableByteStreamController
- [ ] ReadableStream
- [ ] ReadableStreamBYOBReader
- [ ] ReadableStreamBYOBRequest
- [ ] ReadableStreamDefaultController
- [ ] ReadableStreamDefaultReader
- [ ] Request
- [ ] Response
- [ ] SubtleCrypto
- [ ] TextDecoder
- [ ] TextDecoderStream
- [ ] TextEncoder
- [ ] TextEncoderStream
- [ ] TransformStream
- [ ] TransformStreamDefaultController
- [ ] URL
- [ ] URLSearchParams
- [ ] WebAssembly.Global
- [ ] WebAssembly.Instance
- [ ] WebAssembly.Memory
- [ ] WebAssembly.Module
- [ ] WebAssembly.Table
- [ ] WritableStream
- [ ] WritableStreamDefaultController

#### Global methods / properties:

- [ ] globalThis
- [x] globalThis.atob()
- [x] globalThis.btoa()
- [ ] globalThis.console
- [ ] globalThis.crypto
- [ ] globalThis.fetch()
- [ ] globalThis.navigator.userAgent
- [ ] globalThis.performance.now()
- [ ] globalThis.performance.timeOrigin
- [ ] globalThis.queueMicrotask()
- [ ] globalThis.setTimeout() / globalThis.clearTimeout()
- [ ] globalThis.setInterval() / globalThis.clearInterval()
- [ ] globalThis.structuredClone()
- [ ] globalThis.WebAssembly.compile()
- [ ] globalThis.WebAssembly.compileStreaming()
- [ ] globalThis.WebAssembly.instantiate()
- [ ] globalThis.WebAssembly.instantiateStreaming()
- [ ] globalThis.WebAssembly.validate()

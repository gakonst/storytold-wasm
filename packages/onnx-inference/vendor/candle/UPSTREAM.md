# Provenance and local modifications

Copied from `storytold/placeholder-artcraft`, exact source snapshot commit
`0daf83dba9b36e2d9a870114cb12ec873913bd58`, directory `crates/vendor/candle`:
https://github.com/storytold/placeholder-artcraft/tree/0daf83dba9b36e2d9a870114cb12ec873913bd58/crates/vendor/candle

That snapshot vendors Hugging Face Candle, with workspace package version 0.8.3.
It does not identify the exact original Hugging Face Git revision. The hash above
is the exact verified source repository commit, not an invented Candle revision.
Retained subset: candle-core, candle-nn, candle-onnx and both upstream license texts.

Local changes:
- Expand workspace manifest dependencies into standalone manifests, set package
  versions/licenses and use local path dependencies; retain optional accelerator
  source/feature declarations but do not enable or build them.
- Exclude `ug` on wasm32 and retain the existing wasm CPU-compatible implementation.
- Use locked vendored protoc for ONNX protobuf generation when PROTOC is unset.
- Replace Rayon CPU parallel iterators with ordinary iterators, preserve numeric
  implementations, run CPU helper loops sequentially and return one CPU thread.
  GEMM consequently uses Parallelism::None. This applies on native targets too.
- Pin half 2.4.1 and wasm-bindgen 0.2.129 in the wrapper; commit its Cargo.lock.

The MIT and Apache-2.0 licenses are unchanged. Other dependency licenses remain
those of their respective upstream crates.

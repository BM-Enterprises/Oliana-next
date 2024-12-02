
# Experiment status

## `Oliana-Lib`

**Goal:** For all behaviors common to other tools, place them here and include into the other programs.

**Status:** `ollama_lib` gives some utility functions such as:

 - `oliana_lib::files::get_cache_file(<file-name>)`
    - uses `dirs` to join file paths to a local app-specific folder (ie `%LocalAppData%\AppName\<file-name>` on windows, `~/.cache/AppName/<file-name>` on linux)
 - `oliana_lib::files::existinate(<local-file-path>, <url>)`
    - Downloads file if it does not exist, returning the file path

 - `oliana_lib::err::eloc!()`
    - Useful for adding line numbers to rust Error returns; we commonly use `-> Result<THE_TYPE_WE_WANT, Box<dyn std::error::Error>>` to avoid caring about detailed errors, but line numbers are nice to add to these!

## `Oliana-Images`

**Goal:** Build a stand-alone executable that can

1. Download all files it needs to some local cache folder
2. Execute a GPU-Accelerated text-to-image pipeline

**Status:** Success! At the moment the results are all hard-coded, but we have the minimum needed to be useful. We currently download all of `https://huggingface.co/lmz/rust-stable-diffusion-v2-1/resolve/main/weights/*.safetensors` and run a GPU-accelerated image-generation, which takes approximately `10s` for 24 steps of inference producing a `512x512` image using an Nvidia A5000 (approx `0.4s/step`, including process-start, model-load, and image-save overhead)

```bash
TORCH_CUDA_VERSION=cu124 cargo run --release --bin oliana_images
```

## `Oliana-Text`

**Goal:** Build a stand-alone executable that can

1. Download all files it needs to some local cache folder
2. Execute a GPU-Accelerated context-question-answer pipeline

**Status:** Nothing yet, but https://github.com/EricLBuehler/mistral.rs looks promising!




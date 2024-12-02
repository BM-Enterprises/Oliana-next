
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

Requirements for running bare `oliana_images[.exe]`:

 - Must add the folder containing `libtorch_cuda.so` to `LD_LIBRARY_PATH`. We will handle this in the launcher.

## `Oliana-Text`

**Goal:** Build a stand-alone executable that can

1. Download all files it needs to some local cache folder
2. Execute a GPU-Accelerated context-question-answer pipeline

**Status:** The current implementation runs `microsoft/Phi-3.5-mini-instruct` on the GPU, but we don't control where model files are saved to.


```bash
cargo run --release --bin oliana_text
```

Requirements for running bare `oliana_text[.exe]`:

 - None! `\o/`


## `Oliana-Server`

**Goal:** Build a stand-alone webserver that allows bi-directional communication between a system without a GPU and a system WITH a GPU to run the following sub-tools:

 - `oliana_images[.exe]`
    - Given some text prompt, return in-progress images and the final image from a diffusion run on a GPU.
 - `oliana_text[.exe]`
    - Given some text prompt, return tokens as they are generated w/ a sentinel value to indicate the end at the final token.

**Stretch Goal:** Keep the same model files in-memory so clients don't have to pay start-up costs for each request to generate an image or text.

**Status:** Nothing so far, need to find a good RPC mechanism to use.




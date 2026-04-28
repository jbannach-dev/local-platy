# Loacal Platy
A desktop application built with Tauri and React for running Large Language Models locally. It uses llama-cpp-2 to load and interact with GGUF models directly on your machine. The goal is to provide a simple, one-click solution that works without complex setup.

## Features
* Local Desktop Application: Built with Tauri to run entirely offline.
* UI: Built entirely on React with TypeScript.
* GGUF Support: Built-in compatibility for GGUF models via llama-cpp-2.
* Open Source: Simple codebase designed for modification and personal use.

## Installation 
For the quickest setup with Qwen3 1.7b, download the pre-compiled executable for your platform from the latest GitHub release.

## How to build from source:
1. Install Dependencies: Run `deno install` to fetch all necessary packages.
2. Prepare Model: Copy your GGUF model to `./src-tauri/models/` and rename it to `model.gguf`.
3. Development: Run the development task via `deno task tauri dev`.
4. Build: Create the executable for your OS using `deno task tauri build`.

the path is currently hardcoded

### Note 
This application is currently developed and tested on Linux using the Qwen3-1.7B model. Support for other platforms and models is ongoing.
# Tauri Plugin torchlight

A simple Tauri plugin to control the smartphone's flashlight (ON/OFF) functionality.
This plugin is designed for mobile applications using Tauri v2.

## Features

- Turn the smartphone's flashlight on or off.
- Cross-platform support for Android and iOS.
- Simple API for integration with your Tauri-based applications.

## Installation

To use this plugin, you need to add it to your Tauri project. Here are the steps:

### Rust Setup

Add the following line to your `src-tauri/Cargo.toml` file to include the plugin:

```toml
[dependencies]
tauri-plugin-torchlight = { version = "1.0.0", git = "https://github.com/lembryo/tauri-plugin-torchlight.git" }
```

### JavaScript/TypeScript Setup

Currently, there is no particular need

## Usage

First, register the plugin in your Tauri application. Add the following in your `src-tauri/src/main.rs` file:

- src-tauri/src/lib.rs
    ```rust
    fn main() {
        tauri::Builder::default()
            .plugin(tauri_plugin_torchlight::init()) // Add this line 
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }
    ```

Next, add the required permissions to your Tauri application. Add the following in your `src-tauri/capabilities/default.json` file:

- src-tauri/capabilities/default.json
    ```json
    {
        "permissions": [
            "torchlight:default"
        ]
    }
    ```

In your JavaScript/TypeScript code, you can use the plugin like this:

```javascript
invoke("plugin:torchlight|torch", {
    enabled: true // true to turn on the flashlight, false to turn it off
})
    .then(() => {
        // If successful, the then function is called.
    })
    .catch((e) => {
        // Handle the error
        console.error(e)
    })
```

## License

This plugin is licensed under the MIT or Apache 2.0 license, at your option.

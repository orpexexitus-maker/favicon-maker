\# Favicon Maker



A tiny native Windows tool that turns any image into a multi-resolution .ico favicon.



\## Download



\*\*Quick download (raw exe):\*\*

\[dist/favicon-maker.exe](dist/favicon-maker.exe) - click the link, then click the download button.



\*\*Official releases:\*\*

\[Releases page](../../releases) - versioned builds.



\### Requirements



\- Windows 10 or Windows 11 (x64)

\- No runtime, no installer, no dependencies



Just download and double-click the exe.



\## Features



\- Loads PNG, JPG, JPEG, BMP, GIF, WebP

\- Outputs a proper multi-resolution .ico (16, 32, 48, 64, 128, 256 px)

\- Preserves transparency (RGBA)

\- Uses Lanczos3 downscaling for clean edges

\- Single \~700 KB native executable, no runtime required



\## Usage



1\. Run favicon-maker.exe

2\. Click Browse and pick your source image

3\. Choose which sizes to embed (defaults are good for web use)

4\. Set the output .ico path

5\. Click Generate Favicon



\## Build from source



Requirements:



\- Rust (https://rustup.rs) - MSVC toolchain

\- Visual Studio Build Tools with the "Desktop development with C++" workload



&#x20;   git clone https://github.com/YOUR-USERNAME/favicon-maker.git

&#x20;   cd favicon-maker

&#x20;   cargo build --release



The executable lands in target/release/favicon-maker.exe.



To update the committed binary in dist/:



&#x20;   cargo build --release

&#x20;   copy target\\release\\favicon-maker.exe dist\\favicon-maker.exe

&#x20;   git add dist/favicon-maker.exe

&#x20;   git commit -m "Update dist binary"

&#x20;   git push



\## Project layout



&#x20;   favicon-maker\\

&#x20;     assets\\app.ico          (icon embedded in the exe)

&#x20;     dist\\favicon-maker.exe  (committed build artifact)

&#x20;     src\\main.rs             (application source)

&#x20;     build.rs                (embeds assets\\app.ico into the exe)

&#x20;     Cargo.toml

&#x20;     Cargo.lock

&#x20;     LICENSE

&#x20;     README.md



\## License



MIT - see LICENSE.


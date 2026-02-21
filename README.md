# Engram

A desktop application for searching quotes in your local video files. Think [Yarn](https://getyarn.io), but for your personal media collection.

By default it searches for video files in your entire computer along with their `.srt` files. Optionally, if the file is not found it uses whisper.rs with GPU acceleration to generate the subtitle files. The first use is going to be the slowest since it has to index everything but every subsequent use is going to be fast.

You can download each quote in various different file formats. FFMPEG is required to be installed.

The name engram comes from the neuroscientific term for a memory trace. This project was inspired by how I often remember quotes but fail to remember where they're from.

## Project Structure

The repository is organized into two main components:

- **`./src`** — User interface and frontend code
- **`./engram-lib`** — Core application logic and backend

## License

This project is licensed under the Apache License 2.0. See the [LICENSE](LICENSE.md) file for details. You are free to use, modify, and distribute this software in accordance with the terms of the Apache 2.0 license.

## Planned features

- [ ] Add Settings
- [ ] Add SQLite integration to cache everything.
- [ ] FFMPEG-Next?
- [ ] Video
- [ ] Making the frontend semi-usable

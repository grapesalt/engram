# Engram

A desktop application for searching quotes in your local video files. Think [Yarn](https://getyarn.io), but for your personal media collection.

By default it searches for video files in your entire computer along with their `.srt` files or any subtitle tracks in the video itself. Optionally, if the file is not found it uses whisper with GPU acceleration to generate the subtitle files. The first use is going to be the slowest since it has to index everything but every subsequent use is going to be fast.

You can download each quote in various different file formats.

The name engram comes from the neuroscientific term for a memory trace. This project was inspired by how I often remember quotes but fail to remember where they're from.

## Project Structure

The repository is organized into two main components:

- **`./src`** — User interface and frontend code
- **`./engram-lib`** — Core application logic and backend

## License

This project is licensed under the GNU General Public License v3.0 (GPL-3). See the [LICENSE](LICENSE.md) file for details. You are free to use, modify, and distribute this software in accordance with the terms of the GPL-3 license.

## Planned features

- [ ] Add Settings
- [ ] Support other subtitle formats
- [ ] Add SQLite integration to cache everything
- [ ] Preview page
- [ ] Making the frontend semi-usable

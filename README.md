[![codecov](https://codecov.io/gh/Eraden/rider/branch/master/graph/badge.svg)](https://codecov.io/gh/Eraden/rider)
[![Build Status](https://travis-ci.com/Eraden/rider.svg?branch=master)](https://travis-ci.com/Eraden/rider)

# rider
Text editor in rust

## Build

```bash
curl https://sh.rustup.rs -sSf | sh
sudo apt-get install -q -y libsdl2-dev libsdl2-2.0-0 libsdl2-gfx-dev libsdl2-image-dev libsdl2-mixer-dev libsdl2-net-dev libsdl2-ttf-dev
rustup run nightly cargo build --all -rr
rustup run nightly cargo run -p rider-editor
```

## Keyboard mapping

* `DELETE` - delete next character
* `BACKSPACE` - delete prev character
* `SHIFT + DELETE` - delete line
* `CTRL + O` - open file
* `CTRL + S` - save current file
* `ESCAPE` - close current modal

## Road map

### v1.0

* [x] Basic lexer based highlight
* [x] Scrolling
* [x] Handle click based caret movement
* [x] Handle caret movement with arrow keys
* [x] Add text content
* [x] Open file menu
* [x] `Save file` with button
* [x] `Save file` with shortcut
* [x] Theme based menu UI
* [x] Lock scroll when no available content
* [x] Config edit menu
* [x] Project tree
* [x] Cover `rider` with tests at least 50%
* [x] Handle resize window
* [ ] Selection

### v1.1

* [ ] Debugger
* [ ] Open file from CLI
* [ ] Tabs
* [ ] Git support
* [ ] Context menu
* [ ] Keep indent
* [ ] Multi-selection
* [ ] Cover `rider` with tests at least 75%

### v1.2
* [ ] Multi-caret
* [ ] Projects menu

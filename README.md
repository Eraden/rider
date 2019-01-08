[![codecov](https://codecov.io/gh/Eraden/rider/branch/master/graph/badge.svg)](https://codecov.io/gh/Eraden/rider)
[![CircleCI](https://circleci.com/gh/Eraden/rider.svg?style=svg&circle-token=546aae50b559665bd1f77a6452eff25e26a9d966)](https://circleci.com/gh/Eraden/rider)

# rider
Text editor in rust

## Build

```bash
curl https://sh.rustup.rs -sSf | sh
sudo apt-get install -q -y libsdl2-dev libsdl2-2.0-0 libsdl2-gfx-dev libsdl2-image-dev libsdl2-mixer-dev libsdl2-net-dev libsdl2-ttf-dev
rustup run nightly cargo build
```

## Road map

### v1.0

* [x] Basic lexer based highlight
* [x] Scrolling
* [x] Handle click based caret movement
* [ ] Handle caret movement with arrow keys
* [x] Add text content
* [ ] Open file menu
* [ ] `Save file` with button
* [ ] `Save file` with shortcut
* [ ] `Save file as...` with shortcut
* [x] Theme based menu UI
* [ ] Lock scroll when no available content
* [ ] Config edit menu
* [ ] Project tree
* [ ] Cover `rider` with tests at least 50%
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

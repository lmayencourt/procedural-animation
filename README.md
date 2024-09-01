# Procedural animation

This repo contains the code to generate animated creatures based on *procedural animation*.

This project implement in Rust the concept showed in the [A simple procedural animation technique](https://www.youtube.com/watch?v=qlfh_rv6khY).

Try it [online](https://lmayencourt.github.io/procedural-animation/) !

## Requirements overview
Essential features are :
- Different kind of creature can be animated (fish, reptile, mollusc, ...)
- The creatures can have legs that move automatically.

## Solution strategy
- [Rust](https://www.rust-lang.org) as a development language.
- [bevy engine](https://bevyengine.org) as game engine.
- [Web Assembly]() as the deployment target.

The creatures are developed for a 2d top-down perspective. They are drawn by combining primitive shapes.

## License
Licensed under MIT license ([LICENSE-MIT](LICENSE.txt) or http://opensource.org/licenses/MIT)
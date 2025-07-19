# Desktop Cube Baby

Desktop Cube Baby is a desktop companion that you can knock around with your mouse.

Made with love by transgender spiders 🏳️‍⚧️🕷️🕸️

## Installation

Desktop Cube Baby can be installed through one of the following methods:

### Download the latest release

Desktop Cube Baby's latest releases will be available through [this repository's 'releases' section][1].

### Compile from source

You may alternatively download Desktop Cube Baby's source code directly,
compile, and install it yourself.

Please note that the texture and sounds must be extracted manually,
however,
as they are not distributed with this repository.
The extracted files must be placed within `./src/cube_baby.png`
and `./src/cube_baby_kick_0*.wav`.
There should be exactly four audio files, labeled `01` through `04`.

These files can be sourced from the game's installation directory,
with the texture being located within `extracted_resources/resources/gfx/familiar/familiar_cube_baby.png`
and the audio being located within `extracted_resources/resource/sfx/cubebaby_kick_0*.wav`.

Tools for resource extraction are bundled with the game,
located within `tools/ResourceExtractor`.

```sh
git clone https://github.com/Jaxydog/desktop-cube-baby.git
cd ./desktop-cube-baby
cargo build --release
cp ./target/release/desktop-cube-baby /path/to/destination
cargo clean
```

You may optionally enable specific feature flags to enable additional functionality:

- `multi_threaded` - Use multiple threads to update and render the application.
- `visible_console` - Display the internal console on Windows builds.
- `wayland` - Allow the application to render using Wayland.
- `x11` - Allow the application to render using X11.

To use these flag(s),
add `--features` followed by a comma-separated list of flags
to the `cargo build` command.

By default, the `multi_threaded`, `wayland`, and `x11` flags are enabled.

When building for Windows, the `wayland` and `x11` flags can be safely disabled
using the `--no-default-features` and `--features multi_threaded` flags.

## Usage

To use Desktop Cube Baby,
just run the executable.

The application is set to be always-on-top,
so that the baby is always perfectly visible.

Please note that on some graphics devices,
the application will not have proper window transparency.
Unfortunately,
this is not something that I can resolve myself.

## License

Desktop Cube Baby is free software:
you can redistribute it and/or modify it under the terms of the
GNU General Public License as published by the Free Software Foundation,
either version 3 of the License,
or (at your option) any later version.

Desktop Cube Baby is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY;
without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with Desktop Cube Baby.
If not, see <https://www.gnu.org/licenses/>.

[1]: https://github.com/Jaxydog/desktop-cube-baby/releases

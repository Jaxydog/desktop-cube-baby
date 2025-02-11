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

Please note that the texture must be extracted manually,
however,
as it is not distributed with this repository.
The extracted file must be placed within `./src/cube_baby.png`.

```sh
git clone https://github.com/Jaxydog/desktop-cube-baby.git
cd ./desktop-cube-baby
cargo build --release
cp ./target/release/desktop-cube-baby /path/to/destination
cargo clean
```

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

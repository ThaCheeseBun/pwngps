# pwngps
paw-gps but less hacky for pwnagotchi.

# DISCLAIMER
Early access software, not thoroughly tested yet, bugs will occur. Please report any issues you encounter :).

# Installation
- Get [Termux](https://f-droid.org/en/packages/com.termux/) and [Termux:API](https://f-droid.org/en/packages/com.termux.api/) from F-Droid.
- Open Termux and run `pkg upgrade` to upgrade all packages. Answer `y` to any questions that come up.
- Run `pkg install termux-api` to install API support.
- Run `curl -o pwngps "https://github.com/ThaCheeseBun/pwngps/releases/download/latest/pwngps-aarch64" && chmod +x pwngps` to download the binary and make it executable. Change `aarch64` to your architecture if your phone uses something else.
- Add this to your `config.toml`. Replace ip address if necessary, this example uses the android bluetooth address.
```toml
main.plugins.paw-gps.enabled = true
main.plugins.paw-gps.ip = "192.168.44.1:42069"
```
- (Optional) Exclude Termux from power saving features. Varies per phone so use your favorite search engine to figure it out.

# Usage
Just run `./pwngps` to start the server everytime you're going to use your pwnagotchi.

# License
Copyright (C) 2023  ThaCheeseBun

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
# pwngps
paw-gps but less hacky for pwnagotchi.

# DISCLAIMER
Early access software, not thoroughly tested yet, bugs will occur. Please report any issues you encounter :).

# Installation
- Get [Termux](https://f-droid.org/en/packages/com.termux/) and [Termux:API](https://f-droid.org/en/packages/com.termux.api/) from F-Droid.
- Open Termux and run <code>pkg upgrade</code> to upgrade all packages. Answer <code>y</code> to any questions that come up.
- Run <code>pkg install termux-api</code> to install API support.
- Run <code>curl -o pwngps https://github.com/ThaCheeseBun/pwngps/releases/download/latest/pwngps-`uname-m` && chmod +x pwngps</code> to download the binary and make it executable.
- Add this to your <code>config.toml</code>. Replace ip address if necessary, this example uses the android bluetooth address.
```toml
main.plugins.paw-gps.enabled = true
main.plugins.paw-gps.ip = "192.168.44.1:42069"
```
- (Optional) Exclude Termux from power saving features. Varies per phone so use your favorite search engine to figure it out.

# Usage
Just run <code>./pwngps</code> to start the server everytime you're going to use your pwnagotchi.

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
# Frigatify
<h4 align="center">
 A Rust client providing <strong>FRIGAT</strong>e NVR Linux desktop <strong>NOTIF</strong>ications using only <code>MQTT</code>. No Home Assistant required!
</h4>
<p align="center">
 <a href="https://opensource.org/license/gpl-3-0/"><img
  alt="License: GPLv3"
  src="https://img.shields.io/github/license/justaCasualCoder/Frigatify"></a>
  <a href="https://img.shields.io/github/issues/justaCasualCoder/Frigatify"><img
  alt="Open Issues"
  src="https://img.shields.io/github/issues/justaCasualCoder/Frigatify"
  ></a>
 <a href="https://github.com/justaCasualCoder/Frigatify/actions/workflows/build_and_release.yml"><img
  alt="Build Status"
  src="https://github.com/justaCasualCoder/Frigatify/actions/workflows/build_and_release.yml/badge.svg?branch=master"                                                         ></a>
 <a href="https://www.rust-lang.org/"><img
  alt="Coded in Rust"
  src="https://img.shields.io/badge/Rust-CE422B"></a>
</p>

## Getting Started

Download [latest release](https://github.com/justaCasualCoder/Frigatify/releases/tag/latest) and install it:
```bash
wget https://github.com/justaCasualCoder/Frigatify/releases/download/latest/linux_x86_64.zip
unzip linux_x86_64.zip
chmod +x frigatify
sudo mv frigatify /bin/
mkdir -p ~/.config/frigatify/
mv config.toml ~/.config/frigatify/
```
Then modify the config to work for you:
```bash
nano ~/.config/frigatify/config.toml
```
Enable & start it if you want to:
```bash
cp frigatify.service /home/lucas/.config/systemd/user/
systemctl --user enable --now frigatify
```
## How to Contribute

- Feel free to open issues for bug reports, feature requests, or general feedback.
- If you're ready to contribute code:
  1. Check the open issues or create a new one to discuss your proposed changes.
  2. Ensure your code follows the project's coding style and conventions.
  3. Open a pull request with a clear title and description.

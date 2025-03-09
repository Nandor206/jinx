## Simple web server in Rust
It can only serve 1 page (index.html) everything else brings you to the error page.
It checks for the files in the Documents so you need to have a folder named public in the documents directory with an index.html (404.html is not needed).

Some times I use questionable commit messeges, please don't take it seriusly. TY

#### Download my app:
- Linux/Mac (Tested on Fedora 41)
```sh
curl -O https://github.com/Nandor206/rust_web/releases/download/v1.0.1/rust_web
```
- Windows (Not tested)
```sh
curl -O https://github.com/Nandor206/rust_web/releases/download/v1.0.1/rust_web.exe
```
##### You can copy and move my html files to the right directory with this command:
- Linux/Mac
```sh
cd ~/Documents
mkdir public && cd public
curl -O https://github.com/Nandor206/rust_web/releases/download/v1.0.1/index.html
```
- Windows
```sh
cd %USERPROFILE%\Documents
mkdir public && cd public
curl -O https://github.com/Nandor206/rust_web/releases/download/v1.0.1/index.html
```

## Simple web server in Rust
I started this project as a web app at first, now it's gonna be an Nginx copy, because I got bored doing it web app (I didn't even had any idea what to do).

Some times I use questionable commit messeges, please don't take it seriusly. TY

### Download my app:
#### Linux/Mac (Tested on Fedora 41)
- Fire up the terminal and run this:
```sh
curl -O https://github.com/Nandor206/rust_web/releases/download/v2.0.0/Jinx
```
#### Windows (Not tested)
- Fire up CMD or Powershell and run this:
```sh
curl -O https://github.com/Nandor206/rust_web/releases/download/v2.0.0/Jinx.exe
```

### Config.yaml (doesn't work without it)
Don't worry, if you installed the app right it should be in the same directory as the runnible. (Must be in the same directory as the runnible!)
If you don't happen to have one you can create one with this template:
```yaml
# If path or main is empty ("") default is going to be used
# File's name that needs to be served (defaults to index.html)
main: ""
# If you want custom error 404 page put a 404.html file in the same directory as the other html file
# Path where the html can be found (defaults to the same directory where the launcher is found)
path: ""
# Port number is needed, there is no default yet. Might support it later.
port: 7878
# Log will be putted into jinx.log (if false, log is going to be on terminal)
log: false
```



### To run the app:
- On Windows:
Run the .exe file
- On Linux/Mac:
Run the file in the terminal:
```sh
./Jinx
```
Note that the additional files are going to be made in the same directory where you run the app.

## TODO:
- Multi page support
- Other error support

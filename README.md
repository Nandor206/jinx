## Simple web server in Rust
I started this project as a web application, then got bored and continued it as a Nginx copy. 
Don't expect too much from it, but it does the very basic stuff. You can share 1 page, you can log the things it does. 

Some times I use questionable commit messeges, please don't take it seriusly. TY

### To download and run my app:
#### On Linux/Mac (Tested on Fedora 41)
- Fire up the terminal and run these commands:
```sh
mkdir ~/Documents/Jinx && cd ~/Documents/Jinx
curl -O https://github.com/Nandor206/rust_web/releases/download/v2.0.0/Jinx
./Jinx
```
- This creates a folder in the Documents folder and downloads the additional stuff needed (via the app)

#### On Windows (Not tested)
- Fire up CMD or Powershell and run these commands:
```sh
mkdir ~/
mkdir $env:USERPROFILE\Documents\Jinx; cd $env:USERPROFILE\Documents\Jinx
curl -O https://github.com/Nandor206/rust_web/releases/download/v2.0.0/Jinx.exe
./Jinx.exe
```
- This creates a folder in the Documents folder and downloads the additional stuff needed (via the app)

### Configuration file:
If you run the app it should create one file called 'config.yaml' in the same directory as the app.
In the file everything's explained
Here is a template if you need it:
```yaml
# There is a default will be used unless specifically askd for

# Path where the files can be found (defaults to current directory if the string is empty)
path: ""

# The name of the file that will be served (defaults to index.html if the string is empty)
# If you want a custom 404 page put a file named '404.html' in the same directory
main: ""
# .html supported, .php is not yet tested

# Port number:
port: 7878
# Unsigned intager (u32), needed!
# This is where you can find your website

# Logging in to file:
log: false
# Boolean, needed!
# If set true: will create a file named 'jinx.log'
# If yet false: everything goes to the terminal
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

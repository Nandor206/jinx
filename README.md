## Simple web server in Rust
I started this project as a web application, then got bored and continued it as a Nginx copy. 
Don't expect too much from it, but it does the very basic stuff. You can share 1 page, you can log the things it does, etc. Really basic stuff. I hope it will get better by the time. Seems like a really good project.
I hate the fact, that I don't know much about Rust yet, so I'm using ChatGPT here and there. I cannot take responsibility for anything.

Some times I use questionable commit messeges, please don't take it seriusly. TY

### To download and run my app:
#### On Linux/Mac (Tested on Fedora 41)
- Fire up the terminal and run these commands:
```sh
sudo curl -Lo github.com/Nandor206/jinx/releases/latest/download/jinx &&
sudo chmod +x jinx && sudo mv jinx /usr/bin/
```
- This creates a folder in the Documents folder and downloads the additional stuff needed (via the app)

#### On Windows (Not tested)
- Download it with this link:
https://github.com/Nandor206/jinx/releases/latest/Jinx.exe
- The app will download the additional stuff needed

#### To run the app:
- Windows: start Jinx.exe (It's built, but not supported!)
- Linux/Mac: run 
```sh
jinx
```

#### To edit the config file:
```sh
jinx -e
```

### Configuration file:
If you run the app it should create one file called 'config.yaml' in the same directory as the app.
In the file everything's explained
Here is a template if you need it:
```yaml
# There is a default will be used unless specifically askd for

# Path where the files can be found (defaults to current directory if the string is empty)
path: ""

# The name of the file that will be served (defaults to index.html if the string is empty)
# If not found serves: support page
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

# Whether you'd like to open the webbrowser
browser: false
# Boolean, needed!
# If set true: will open default browser
# If set false: won't open nothing
```

## TODO:
- [x] Multi page support - Done, tho not the way I wanted it
- [ ] Other error support - Probably never gonna be implemented
- [x] Logging to file
- [x] Config file - Was the first stuff to do
- [ ] Testing more computers
- [x] Cleaning the code - Done, thos might get dirty again

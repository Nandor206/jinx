## Simple web server in Rust
I started this project as a web application, then got bored and continued it as a Nginx copy. It's made for web development mainly, but works for hosting too (I use it for my own website). 
Don't expect too much from it, but it does the very basic stuff. You can host multiple pages, you can log the things it does, etc. Really basic stuff. I hope it will get better by the time. Seems like a really good project.
I hate the fact, that I don't know much about Rust yet, so I'm using ChatGPT here and there. I cannot take responsibility for anything.

Some times I use questionable commit messeges, please don't take it seriusly. TY

### To download and run my app:
#### On Linux/Mac (Tested on Fedora 41)
- Fire up the terminal and run these commands:
```sh
sudo curl -L -O https://github.com/Nandor206/jinx/releases/download/v5.0.0/Jinx
sudo chmod +x Jinx && sudo mv Jinx /usr/bin/
```
- The app will do the additional stuff

#### On Windows (Not tested)
- Download the .exe file trough this link:
https://github.com/Nandor206/jinx/releases/latest/
- Run the .exe file and it will start the app
- The app will do the additional stuff needed
- Windows is not suppoerted!

#### How to use:
```sh
# Starting the app:
Jinx
# Editing the config file:
Jinx -e, --edit
# Enable logging:
Jinx -l, --log
```

### Configuration file:
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

log_dir: ""
# If left empty the log is going to be next to the config file

# Whether you'd like to open the webbrowser
browser: false
# Boolean, needed!
# If set true: will open default browser
# If set false: won't open nothing
```

## TODO:
- [x] Multi page support - Done, tho not the way I wanted it
- [x] Other error support - Probably never gonna be implemented
- [x] Logging to file
- [x] Config file - Was the first stuff to do
- [x] Cleaning the code - Done, thos might get dirty again
- [ ] Testing more computers

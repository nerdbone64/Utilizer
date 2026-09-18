# utilizer

a simple Lua-driven "boot-up helper".

## how to use

running the program should automatically create a `.utilizer` folder in the root directory, relative to the application.

inside, the folder contains:
- scripts (folder)
- output.log (file)
- settings.json (file)

outputs save across sessions.

### startup

as of right now, the application does not automatically tell your device to automatically start the application upon boot.

this application has not been tested on Linux, and will **never** be tested on Mac.

### api

there is no useful API, yet. it's just `log(string, number)`.
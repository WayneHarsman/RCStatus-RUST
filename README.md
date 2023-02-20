# RCStatus-RUST
[WIP] utility made to simpify work with rclone. Allows to fetch or push changes to the remote of your choice just by running "rcstat fetch" or "rcstat push" in console.

## Installation
1. Download the latest release
2. Put the executable in folder of your choice and add it to PATH or use an alias
3. Newer version requires a sqlite database to properly function. Right now you can create one by running migrations(that are bundled with a source code) from a diesel cli tool. Current version takes database url from the env variable DATABASE_URL. You can set it to the database file like that: `DATABASE_URL=configs.sqlite
4. After that you are completely set.


## Usage
1. Use 'rcstat init' to add directory to sync list. Run 'rcstat init -h' for more info
2. Use 'rcstat fetch' or 'rcstat push' to sync your files. Run 'rcstat fetch -h' or 'rcstat push -h' for more info
3. Use 'rcstat status' to check if you are in the synced directory or its subdirectory. Run 'rcstat status -h' for more info
4. Use 'rcstat forget' to remove directory from sync list. Run 'rcstat forget -h' for more info
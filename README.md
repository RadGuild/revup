# revup

## Install
### Linux
First build the executable\
`cargo build --release`

Then move the binary to your executables folder\
`sudo mv target/release/revup /usr/local/bin`

You should now be able to use `revup` in your terminal, if not open an issue and
I will help you.

## Usage

Make shure you are in the root folder of your scrypto project and run\
`revup -i`

This should've created a default revup.json file in the folder, you only need to
run this once.

You can now run\
`revup`

This will execute all the rev2 commands that are stored in the revup.json file
and it should've created a .env file that has stored all the variables.

You can now run\ 
`source .env`

This will make the .env variables usable in your shell, make shure you re-run
`source .env` everytime you've run `revup`


Todo:
* Windows and MacOS support
* Testing
* Append .gitignore to include .env

License: MIT


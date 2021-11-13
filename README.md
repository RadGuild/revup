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

Make sure you are in the root folder of your scrypto project and run\
`revup -i`

This will prompt for the first function call. If you are running the HelloToken demo then this would be\
`Hello new`

Then there will be a prompt for names for the results in order. For HelloToken this could be\
`tokenHT helloCOMP`

This should've created a default revup.json file in the folder, you only need to
run this once.

You can now run\
`revup`

This will execute all the resim commands that are stored in the revup.json file
and it should've created a .env file that has stored all the variables.

You can now run\ 
`source .env`

This will make the .env variables usable in your shell, make sure that you re-run
`source .env` everytime you've run `revup`.

When using revup in a project under git source code control you will want to update your .gitignore file
to include\
`revup.json
.env`

Todo:
* Update for APE release (DONE but minimally tested)
* Windows and MacOS support
* Additional Testing
* Extensions

License: MIT


# revup

## Install
### Linux
First build the executable\
`cargo build --release`

Then move or copy the binary to your preferred executables folder. For example:\
`sudo mv target/release/revup ~/.cargo/bin` or\
`sudo mv target/release/revup /usr/local/bin`

You should now be able to use `revup --help` in your terminal. If not, open an issue.

## Usage
Make sure you are in the root folder of your scrypto project and run\
`revup -i`

This will prompt for the first function call. If you are running the HelloToken demo then this would be\
`Hello new`

Then there will be a prompt for names for the results in order. For HelloToken this could be\
`tokenHT helloCOMP`

This will create a _revup.json_ config file in the folder.

You can now run\
`revup`

This will execute all the resim commands that are stored in the _revup.json_ file
and also create a _.env_ file that has stored all of the generated environment variables.

You can now run\
`source .env`

This will make the .env variables usable in your shell, make sure that you re-run
`source .env` again after each time that you run `revup`.

### Advanced Usage
If your blueprint has multiple constructors then you will want to make multiple `revup.json` config files.
After you make the first, rename it to something like:\
`mv revup.json revup_new.json`

Now use `revup -i` again to target your other constructor. You can rename it too:\
`mv revup.json revup_other.json`

Now you can quickly do your setup for either constructor using:\
`revup -f revup_new.json` or\
`revup -f revup_other.json`

### Power User Mode
Brand new for revup is a new command file format that you can create yourself. Simply review this example file to get the basic idea:

```
// Revup command file
// Double slash comments are allowed and ignored.
// Blank lines are also ignored.

reset // If revup sees you calling reset, then it clears your _.env_ file.
new-account -> account pubkey
new-account -> account2 pubkey2

new-token-fixed 10000 --name emunie --symbol EMT -> tokenEMT
new-token-fixed 10000 --name gmunie --symbol GMT -> tokenGMT

publish . -> package // we can allow comments here as well

// Now we call the function.
// (We only allow envvars that are defined previously in the session.)

call-function $package CandyShop new -> component
```

You run a command file like this:\
`revup -r CandyShop.rev`

Your command file can have any name but I recommend using the _.rev_ suffix.

Hopefully that is clear enough for power users. More details are coming soon.

### gitignore
When using revup in a project under git source code control you will want to update your .gitignore file
to include:

```
revup*.json
*.rev
.env
```

### Going forward:
Normal maintenace is underway. Feel free to add an issue, comment on issue or even fix an open issue.
We also have defined some projects outlining future directions. Feel free to contribute in any way.

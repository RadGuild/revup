# revup

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

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
and also create a _.env_ file (as well as _env.bat_ and _env.ps1_ if you are on Windows)
that has stored all of the generated environment variables.

You can now run\
`source .env` (bash)
`env.bat` (cmd)
`env.ps1` (PowerShell)

This will make the variables usable in your shell, make sure that you re-run
the command again after each time that you run `revup`.

### Epoch

View the current epoch:

`revup --epoch` or\
`revup -e`

Increment the epoch by a given value:

`revup --epoch 10` or\
`revup -e 10`

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
env.bat
env.ps1
```

### Going forward:

Normal maintenance is underway. Feel free to add, comment upon or even fix an open issue.

We also have defined some projects outlining future directions. To reward our developers and
help us move forward with our plans. you may send a donation of XRD to:

```
rdx1qspk7l8jfqmafwfcpzhx25p3qczj0nczqd9yvaux4892lrw2xgksnfgmkkr69
```

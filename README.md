# UTF-Nate

A discord bot made in rust. Play music, set an intro clip for yourself, and extend with custom commands!

## Features

### Getting started

First get the bot into a channel using `summon`, and it will join whatever channel you are in.

Get rid of it again with `banish`

### Herald

Announce your arrival or departure from a channel automatically with a short clip, for any channel the bot is in!

* `intro` and `outro` let you select from a selection of clips (try `list` to find what comes with the bot), or give it any mp3 or wav link on the internet!
* You can change how the bot introduces *itself*, with the `botintro` command.
* The commands fuzzy search the built in clips. Try finding what you like by putting in text close to what you want!

### Play music and clips

You can play both the in built clips, and youtube content. Because both can be searched, there are two separate commands for them.

* `play` lets you pass in a youtube link, spotify link, or any text that it will then get the first result from youtube an play it!
* Both spotify and youtube playlists are supported! Get a link and it will queue everything at once.
* `clip` searches the built in clips for the best matching name, and plays that.

There's a lot more you can do when playing youtube links, and modifying the queue of audio coming up.

* `playnext` and `playnow` allow you to skip the line, and change where your addition will start in the queue.
* `queue` lets you see what's coming up.
* `skip` can be used to skip what's playing now, or can be passed a number or range of numbers to change the queue (use `queue` to identify the numbers)
* `shuffle` can be used to mix up the entire queue.
* `pause` the queue, or `stop` it entirely

And even more! Take a look at `help` for the full list of commands

### External integration

Whoever sets up your bot can set up some scripts on their server, which you can run using `cmd`. It will vary quite a bit, but setting something up to start and stop a dedicated video game server is a great example of something that is useful for anyone in your server to want to do!

## Building

Debian is the only explicitly supported environment at the moment. However, if you want to run it on a different distro, `cargo build` will get you close, but you may have to install the occasional system library to get compilation to finish. See what happens when compilation errors, and adjust accordingly. `setup/debian-build-setup.sh` may give you a good starting point for what build libraries may be required.

### Debian

```sh
# install the required external dependencies
setup/debian-build-setup.sh
# install cargo-deb to create a deb package, for most convenience
cargo install cargo-deb
```

Now each subsequent build is very simple.
```sh
# build a deb package (into `target/debian`)
cargo deb
# or build the executable itself (into `target/release`)
carg build --release
```

If you want to build with specific features only, you can disable default features, and add features, or any mix thereof.
```sh
# minimal features with --no-default-features (see cargo for details)
cargo deb -- --no-default-features
# or
cargo build --release --no-default-features

# add features with --features (see cargo for details)
cargo deb -- --features http-interface
# or
cargo build --release --features http-interface

# --features and --no-default-features can be mixed
```

The deb package is not required but additionally specifies dependencies when installing.

No matter what build, the resources folder must be delivered separately.

## Setup

Currently, because maximum stability is gained from using the latest yt-dlp, it is installed with the latest version, not from debian packages. If you want to go this route, use the `setup/debian-run-setup.sh` script. It will also install python for yt-dlp.
```sh
setup/debian-run-setup.sh
```

All paths are managed by the working directory. Run the bot from a directory with the `resources/` folder in it, as well as a `keys.toml` and `config.toml` file (expanded upon below).

### Database

You will need a database, either postgresql or sqlite will work. For sqlite, there are examples in `keys.template.toml` where you can easily just make a file database. For postgresql, you'll have to set one up.

### File setup

Once you have the executable or deb package installed, the usage requires just one more important note. It uses the working directory to find the `resources/` folder, `keys.toml` file, and `config.toml` file.

Copy the `resources/` folder directly, this is already set up. You can add more commands integrated into your bot by adding to `resources/cmd/`, or more sounds by adding to `resources/clips/`

For `keys.toml` and `config.toml`, take a look at the corresponding templates (`keys.template.toml` and `config.template.toml`). They come with examples and descriptions of each field required.

### Concurrent bots

It is very easy to share the `resources/` folder between bots with a symlink, and have separate `keys.toml` and `config.toml`. All you need to do is set up separate working directories they launch in. This is why they are not installed using any `etc/` files or anything. The setup requires slightly more care than just a deb package, but you can have multiple instances playing different queues sharing the same database for user settings!

## Running

Now that you've got your folder and config files setup, all you have to do, from that same directory, is run it. If it's on your path, no arguments are required.

```sh
utf-nate
```

For information on arguments available, use
```sh
utf-nate --help
```
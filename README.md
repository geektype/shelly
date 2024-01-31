> Project state: **Under development**. Likely to contain undocumented bugs + security and safety vulnerabilities. Use at own risk.

# Shelly AI
#### A GPT-4 powered shell assistant


### Example usage

```
$ shelly "give me a list of all unique contributors to this repo and the number of commits they have made"

git shortlog -sne
```

## Installation

At the moment the only way to use shelly is by building from source. Eventually there will be more options available for installation.

### Build from source

#### Pre-requisites
To build this project you must have a functioning rust toolchain setup

* `rustc` >= 1.77
* `cargo` >= 1.77

#### Build
```shell
cargo build --release
```

#### Install
You simply need to place the compiled binary in a directory that exists on `PATH`. `/usr/local/bin` is normally considered a good place for programs you compile yourself.

```
cp ./target/release/shelly /usr/local/bin/shelly
```

Reload your shell and you should be able to run shelly

```
$ which shelly
/usr/local/bin/shelly

$ shelly --version
Shelly AI CLI assistant v0.1.0-alpha
```

*Optional:* If you want to use another keyword to invoke shelly instead of `shelly` you can create a symlink
```
sudo ln -s /usr/local/bin/shelly /usr/local/bin/ai
```

You can then invoke shelly using `ai` keyword

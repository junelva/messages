### messages

`messages` is a minimal terminal screensaver that outputs positive messages to your terminal.

#### installation

you will need [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html). then, from within the `messages` folder after cloning this repository:

```
cargo build --release
./target/release/messages
```

to quit, press `q` or `Ctrl+c`.

if you like it, you can install it system-wide.

```
cargo install --path .
```

you can then easily bring up the program.

```
messages
```

#### command line arguments

for more information, try:

```
messages --help
```

the two positional options are `wait` and `clear`.

- `wait`: number of milliseconds to wait between messages.
- `clear`: clear the screen after this many messages.

this allows you to spam messages very quickly and never clear by running:

```
messages 0 0
```

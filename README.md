# `pingme`: send yourself pings

`pingme` is a simple command line utility to send yourself a ping (push notification) via the [Pushover](https://pushover.net) app.

## Installation & Usage

### Rust Toolchain

You need the Rust build toolchain installed. Use [Rustup](https://rustup.rs) to install one if you need it.

### Pushover Account & Application

You'll need to create a [Pushover](https://pushover.net) account and an application. If you want to send to all devices at once, you'll also need a Delivery Group.

### Installation

Clone the repository and `cargo install` the binary:

```
git clone https://github.com/oko/pingme
cd pingme
cargo install --path .
```

### Configuration

Generate your config file more or less like so:

```
cat <<-EOF > $HOME/.pingme.toml
app_token = "$APP_TOKEN"
user_token = "$USER_TOKEN"
EOF
```

If you're on Windows, your config will be at `%APPDATA%\pingme.toml` instead and a modified version of the above to work in Powershell.

### Execution

You should then be able to run:

```
pingme hello
```

Which will send the message "hello" as a push notification.

Or alternatively, you can have `pingme` run a command as a subprocess and then send you a message with the exit code once that command completes.

```
pingme -c whoami
```

This gives you flexibility in how you use `pingme`:

1. As a component of scripts where you can just use it to send messages
2. As a wrapper for manual command line invocations that take a long time
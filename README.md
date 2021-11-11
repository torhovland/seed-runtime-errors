# Seed runtime errors

This repo contains a reproducible sample that triggers runtime errors in a Seed app.

## Building and running

### 1. Install / check required tools

1. Make sure you have basic tools installed:

   - [Rust](https://www.rust-lang.org)
     - Check: `$ rustc -V` => `rustc 1.43.1 (8d69840ab 2020-05-04)`
     - Install: https://www.rust-lang.org/tools/install
   - [cargo-make](https://sagiegurari.github.io/cargo-make/)
     - Check: `$ cargo make -V` => `cargo-make 0.30.7`
     - Install: `$ cargo install cargo-make`

### 2. Run the app

1. Open a new terminal tab / window and run: `cargo make serve`
1. Open a second terminal tab and run: `cargo make watch`

# See the errors

Open [localhost:8000](http://localhost:8000) in a browser.

Note that you can move the map around without issue, and it will increment the number in the Seed model each time you
do.

However, if you click on "Move home", you should see some runtime errors in the browser console.

Note that if you replace the closure

```
move || app.update(msg_mapper(Msg::Increment))
```

with simply

```
|| log!("on_move")
```

there are no runtime errors.

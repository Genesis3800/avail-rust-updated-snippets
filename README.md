# Some scripts that interact with Avail using the `avail-rust` SDK

## Steps to use

1. Clone the repository

2. Run the following command in the terminal to install & build everything in one go

``` bash
cargo build
```

3. Create a `.env` file in the root directory and add your seed phrase

``` bash
SEED="your_seed_phrase"
```

4. Run the script you want to use from within the `src/bin` directory

``` bash
cargo run --bin <script_name>
```
For example, to run the `submit_data.rs` script, you would run the following command

``` bash
cargo run --bin submit_data
```

## Notes

- Please take care while managing your seed phrase.
- Using a `.env` file is NOT recommended for production use, but is good enough for testing purposes.

- 
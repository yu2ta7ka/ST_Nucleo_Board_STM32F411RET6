# st_nucleo_board_stm32_f411_ret6
Program to ST Nucleo Board STM32F411RET6 with Rust.  
ST Nucleo Board STM32F411RET6を対象にRustでプログラミングしたリポジトリです。  
https://os.mbed.com/platforms/ST-Nucleo-F411RE/

This is implemented by referring to the following.
- "基礎から学ぶ 組込みRust" https://www.c-r.com/book/detail/1403
- https://github.com/melodyaheath/stm32f411re-embedded-rust-ST7735-lcd

# requirements
| Item | Version |
| ------------- | ------------- |
| Ubuntu  | 20.04  |
| Rust  | 1.52.1  |

# Getting Started
1. Clone this repository
2. Add crate
```
sudo apt-get install libssl-dev
cargo install cargo-edit
```
3. Add target
```
cd st_nucleo_board_stm32_f411_ret6
rustup target add thumbv7em-none-eabihf
cargo install cargo-binutils
rustup component add llvm-tools-preview
```
4. Build
```
cargo build
```
5. Flash binary file
```
sudo apt install openocd
openocd -f nucleo.cfg -c"program target/thumbv7em-none-eabihf/debug/st_nucleo_board_stm32_f411_ret6 verify reset exit"
```

# What i did
- LED blink
- Button control

![Screenshot from 2021-05-30 06-09-58](https://user-images.githubusercontent.com/44434953/120084773-fc814d00-c10d-11eb-9dc0-a801e8dbd2a9.png)  
Movie:https://twitter.com/UGKGbrothers/status/1398394247082962951


# 詳細な情報
RustでST Nucleo Board STM32F411RET6をLチカさせる  
https://www.yu2ta7ka-emdded.com/entry/2021/05/25/175150

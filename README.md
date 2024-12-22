# dioxus study


## toolchain
1. 安装rust，rust官网
```sh
# https://www.rust-lang.org/zh-CN/tools/install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
2. 安装dioxus
```sh
cargo install cargo-binstall # 这个编译了很久
cargo binstall dioxus-cli

3. 准备tailwindcss环境
```sh
cd ui
npx tailwindcss init
npm install

# start tailwindcss
# 准备一个input文件, 然后启动tailwind
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```


4. 添加Dioxus.tomal启动dioxus
```sh
dx serve
```

## tailwind style website
```rust
https://flowbite.com/docs/components/
https://tailwindflex.com/    //官网的包不需要额外的安装
https://daisyui.com/components/mockup-code/
```

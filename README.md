# test_rust

https://doc.rust-jp.rs/book-ja/ch01-03-hello-cargo.html

cargo new hello_cargo

cargo newを使ってプロジェクトを作成できる
cargo buildを使ってプロジェクトをビルドできる
cargo runを使うとプロジェクトのビルドと実行を1ステップで行える
cargo checkを使うとバイナリを生成せずにプロジェクトをビルドして、エラーがないか確認できる
Cargoは、ビルドの成果物をコードと同じディレクトリに保存するのではなく、target/debugディレクトリに格納する

https://qiita.com/notakaos/items/9f3ee8a3f3a0caf39f7b

cargo install cargo-edit

cargo install cargo-watch

rustup component add rustfmt

rustup component add clippy

rustup component add rls rust-src rust-analysis

### vscode extension

rust-analyzer

#### debug

CodeLLDB

https://doc.rust-jp.rs/book-ja/ch04-01-what-is-ownership.html

Rustの中心的な機能は、所有権です。


所有権規則

- Rustの各値は、所有者と呼ばれる変数と対応している。
- いかなる時も所有者は一つである。
- 所有者がスコープから外れたら、値は破棄される。


https://qiita.com/uasi/items/3b08a5ba81fede837531
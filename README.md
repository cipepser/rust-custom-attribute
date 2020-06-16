# 下町.rs#2のデモコード

以下イベントで発表するための、デモコードです。

[下町\.rs \#2 \(オンライン） \- connpass](https://shitamachi.connpass.com/event/177861/)


## 環境

動作確認済みの環境は以下です。  
（もっと広く動くのでそんなに気にしなくてよさそう）

```
❯ cargo --version
cargo 1.44.0 (05d080faa 2020-05-06)

❯ rustup --version
rustup 1.21.1 (7832b2ebe 2019-12-20)
```

## 動かし方

```
❯ git clone https://github.com/cipepser/rust-custom-attribute
❯ cd app
❯ cargo run
```

### 実行結果

`failed`は意図的なものです。

```
❯ cargo run
start running tests
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `2`', app/src/main.rs:12:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
testing app::fail ... failed!
testing app::check ... ok!

failures:
    app::fail

test result FAILED. 2 tested, 1 passed, 1 failed
```

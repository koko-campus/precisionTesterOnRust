# precisionTesterOnRust

Rustでの精度テスト用のモジュールです。


# 環境情報

| 機能 | バージョン |
| ---- | ---- |
| Linux / Ubuntu | 20.04.5 |
| Rust | 1.63.0 |



# 環境構築

## apt アップデート

```bash
# イロイロ最新に
sudo apt update
sudo apt upgrade
```

## Rust インストール

```bash
curl https://sh.rustup.rs -sSf | sh
# インストール設定はデフォルト(1)で!!!
# 次に環境変数(PATH)を設定します。
export PATH="$HOME/.cargo/bin:$PATH"
# 最後に正しくインストール、パスの設定がされたか、以下のコマンドで確認します。
cargo --version
# -> cargo 1.63.0
rustc 1.63.0
# -> rustc 1.63.0
rustdoc --version
# -> rustdoc 1.63.0
```


## 多倍長計算用のモジュールのインストール


```bash
# gccとかmakeとか諸々インストール
sudo apt install build-essential

sudo apt install m4 m4-doc


```

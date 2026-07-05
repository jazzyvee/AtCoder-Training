# AtCoder学習（Rust）

Rustと競プロの学習用リポジトリ。

## AtCoder

`atcoder`フォルダでAtCoderを解く。`cargo-compete`を使用。

## mdbook

`book`フォルダで、回答コードを参照するmdbookを管理する。

後で自分が見返す用。github pagesでホスト。

`atcoder/<contest>/`配下の回答コード（`todo!()`のまま未着手のものは除く）から、コンテストごとに1ページを`book/generate.sh`が自動生成する。`atcoder/<contest>/memo.md`を置いておくと、そのコンテストのページの先頭に取り込まれる。

`book/src/*.md`・`book/src/SUMMARY.md`は生成物のためコミットしない（`book/.gitignore`参照）。pushすると GitHub Actions が`generate.sh`を実行してからビルド・デプロイする。ローカルでプレビューする場合は先に`bash book/generate.sh`を実行してから`mdbook serve book`を使う。

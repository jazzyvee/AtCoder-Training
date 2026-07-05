# AtCoder学習（Rust）

Rustと競プロの学習用リポジトリ。

## AtCoder

`atcoder`フォルダでAtCoderを解く。`cargo-compete`を使用。

## mdbook

`book`フォルダで、回答コードを参照するmdbookを管理する。

後で自分が見返す用。github pagesでホスト。

`atcoder/<contest>/`配下から、コンテストごとに1ページを`book/generate.sh`が自動生成する。回答済みの問題はコードを埋め込み、`todo!()`のままの未着手の問題は「（未回答）」と分かる形でページに残す。`atcoder/<contest>/memo.md`を置いておくと、そのコンテストのページの先頭に取り込まれる。問題タイトルはビルド時ではなくブラウザ側のJS（`book/theme/title-fetch.js`）がAtCoder Problemsの公開APIから取得して補完する。

`book/src/*.md`・`book/src/SUMMARY.md`は生成物のためコミットしない（`book/.gitignore`参照）。pushすると GitHub Actions が`generate.sh`を実行してからビルド・デプロイする。ローカルでプレビューする場合は先に`bash book/generate.sh`を実行してから`mdbook serve book`を使う。

GitHub側ではリポジトリの Settings → Pages → Build and deployment → Source を `GitHub Actions` に設定しておく必要がある（初回のみ）。

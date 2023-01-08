# helpman

**helpman** is Cli Application.

# Usage

## ドキュメント操作

```
helpman [Option] [FILE]
```

* Option 

|  Option  |  説明  |
| ---- | ---- |
|  -w  |  ドキュメントの作成、編集  |
|  -r  |  ドキュメントの参照  |
|  -m, --markdown  |  Markdown(他のコマンドと併用)  |
|  -d  |  ドキュメントの削除  |

* File : ファイル名(拡張子なし)

## Path

```
helpman [Option] Path
```

|  Option  |  説明  |
| ---- | ---- |
|  -S  |  パスのセット  |
|  -R  |  パス初期化  |
|  -v  |  現在のパスを表示  |

デフォルトでは ~/Document/に設定されている。

## その他

```
helpman [Option]
```

|  Option  |  説明  |
| ---- | ---- |
|  -l  |  ドキュメントリストの表示  |
|  -D  |  ドキュメントデータの削除  |
|  -V, --version  |  version表示  |
|  -i, --init  |  helpmanディレクトリ初期化　|
| -h, --help | usage表示 |
`-i`はディレクトリが存在しない場合だけ初期化される。
すでにフォルダが存在する場合は何にもしない。

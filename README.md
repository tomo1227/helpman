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
|  -m  |  Markdown(他のコマンドと併用)  |

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

## その他

```
helpman [Option]
```

|  Option  |  説明  |
| ---- | ---- |
|  -l  |  ドキュメントリストの表示  |
|  -d  |  ドキュメントデータの削除  |
|  -V  |  version表示  |
|  -i  |  helpmanディレクトリ初期化　|

`-i`はディレクトリが存在しない場合だけ初期化される。
すでにフォルダが存在する場合は何にもしない。

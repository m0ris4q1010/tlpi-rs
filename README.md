# The Linux Programming Interface

## [4] ファイル I/O : 統一されたインターフェース

### (4.3) ファイルのオープン : open()

- file access mode flags
- file creation flags
- (open) file status flags

![open()の引数flags](images/tlpi-table-4-3.png)

## [5] ファイル I/O : その詳細

### (5.4) ファイルディスクリプタとオープンしたファイルの関係

![ファイルディスクリプタ、オープンファイル条法、i-nodeの関係](images/tlpi-fig-5-2.png)

## [6] プロセス

### (6.4) 仮想メモリ管理

![プロセスメモリレイアウト](images/tlpi-fig-6-1.png)

## [14] ファイルシステム

### (14.3) ファイルシステム

![ディスクパーティションとファイルシステムのレイアウト](images/tlpi-fig-14-1.png)

### (14.4) i-node

![ext2ファイルシステムでのファイルブロック](images/tlpi-fig-14-2.png)

## [18] ディレクトリとリンク

### (18.1) ディレクトリと（ハード）リンク

i-node番号
- 0: 未使用
- 1: bad block

![i-nodeとディレクトリ構造の関係(/etc/passwdの例)](images/tlpi-fig-18-1.png)

![ハードリンクとシンボリックリンクの内部表現](images/tlpi-fig-18-2.png)

## [20] シグナル：基礎

### (20.2) シグナル種類とディフォルト動作

![Linuxのシグナル](images/tlpi-fig-20-1-1.png)

![Linuxのシグナル続き](images/tlpi-fig-20-1-2.png)

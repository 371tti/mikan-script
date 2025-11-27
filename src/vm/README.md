# VM モジュール

このディレクトリ `src/vm` に含まれるファイルの簡単な説明です。

- `code_manager.rs`: **CodeManager / デコード管理** — バイトコードの遅延デコード、関数テーブル (`latest_function_table`) の管理、所有する `Function` の保持。`RwLock` を使って共有・更新を行う。

- `function.rs`: **Function / FunctionPtr** — 命令列を `Pin<Box<[Instruction]>>` で保持する `Function` 構造体と、生ポインタを包む `FunctionPtr`。命令テーブルの参照を軽量に扱うための型。

- `memory.rs`: **Memory / Heep / RawHeep** — ヒープ管理。`Memory` が複数の `Heep` を保持し、各 `Heep` が内部で `RawHeep` を使って低レベルの `alloc`/`realloc`/`dealloc` を行う。ポインタ操作や unsafe を用いた高速メモリ管理実装。

- `mod.rs`: **モジュールエクスポート + VMPool** — `vm` サブモジュール群の公開と、複数VMをスレッドで起動する `VMPool` 実装（core affinity オプション、`Arc<RwLock<VM>>` を使った共有）。

- `operations.rs`: **命令実装（Operations）** — `Instruction` 型定義と多数の命令ハンドラ（整数/浮動小数点/論理/メモリ/atomic/制御/IO 等）。各命令は `vm.st.pc` の更新（fallthrough）やジャンプ/コール/ret を扱う。

- `pre_decoder.rs`: **PreDecoder（事前デコーダ）** — テキスト形式のバイトコードをパースして `Function`（命令配列）に変換する。opcode テーブルや引数パース、エラーハンドリングを含む。

- `vm.rs`: **VM 実行部（Direct-threaded VM）** — `VM` と `VMState` の定義、`run()` による命令ループ（関数ポインタ配列を参照する direct-threaded 実装、ループアンローリングあり）。`state_flag` を使った停止制御など。

- `README.md`: **このファイル**。

注意点:

- 多くの箇所で `unsafe` を使い、低レベルなポインタ/アロケータ操作を行っています。変更の際は安全性に注意してください。
- `FunctionPtr` は raw ポインタを使っており、`Send`/`Sync` を unsafe に実装しています。所有権やライフタイムの管理を間違えると未定義動作になる可能性があります。
- `CodeManager` は `RwLock` を使って共有状態を管理します。ランタイムでのデコード更新（`get_decode` 等）をサポートしています。

# todo
1. 命令を関数ポインタでなくjump tableにする

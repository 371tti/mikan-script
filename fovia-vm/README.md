# 現時点でのパフォーマンス

1. カウントループ(ディスパッチが支配的な最悪ベンチ)
    ```rust
    let b = 1000_000_000u64;
    let mut a = 0u64;
    while a < b {
        a += 1;
    }
    println!("{}", a);
    ```
    結果
    - ネイティブ 500ms
    - VM 2745ms

    => 相対 0.182× (5.5× 遅延)

2. メモリアクセスAtomicカウントループ(ディスパッチと仮想メモリアクセス最適化具合)
    ```rust
    const LIMIT: u64 = 1_000_000_000;
    let counter = AtomicU64::new(0);

    loop {
        counter.fetch_add(1, Ordering::Relaxed);
        let val = counter.load(Ordering::Relaxed);
        if val >= LIMIT {
            break;
        }
    }
    ```
    結果
    - ネイティブ 7400ms
    - VM 8900ms

    => 相対 0.831× (1.20× 遅延)
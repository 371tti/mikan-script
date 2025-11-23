struct Vec<T> {
    pub ptr: usize,
    pub len: usize,
    cap: usize,
}

enum Option<T> {
    Some(T),
    None
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

state Sorted(bool)
state Predicate {
    expr_buf: String
}

// state は state を操作するためのブロックを配置します
// satisfy の後にはbool を返す式が必要です。 式を満たす場合実装されます。
impl Vec<T> 
satisfy Self@Sorted.is_true()
{
    pub fn shuffle(&mut self) {
        //...
    } state {
        Self@Sorted.0 = false;
    }
}

impl i32 
{
    pub fn add(self: Self@Predicate, other: i32@Predicate) 
    {
        self += other
    } state {
        self@Predicate += other@Predicate;
    }
}
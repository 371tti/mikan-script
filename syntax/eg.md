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

state Sorted: FlagState {
    
}

impl Vec<T> 
where Self: Sorted
{
    def bin_search = {

    } state {}

    def shuffle = {

    } state {
        Self@Sorted.not()
    }
}

state Max: NumState {
    
}

state Min: NumSTate {
    
}

impl i32 {
    pub fn add(self: Self@Max@Min, other: i32@Max@Min) {

    } state {
        Self@Max = a@Max + b@Max
        Self@Min = a@Min + b@Min
    }
}
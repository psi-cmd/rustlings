// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)


// 在 rust 编程过程中尤其要注意的是，& 并不意味着传进去的参数是储存地址，这符号仅仅对 rust 编译器有效
// 即所属关系不改变，仅仅是借用。所以 var，&var，&mut var 三者的 dot 方法相同，不存在 C 中 -> 解引用访问符号。

fn main() {
    let mut vec0 = Vec::new();
    
    let mut vec1 = fill_vec(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec0.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec0.len(), vec0);
}

fn fill_vec(vec: &mut Vec<i32>) {
    // let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    // vec
}

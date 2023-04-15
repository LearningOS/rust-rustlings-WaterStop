// move_semantics1.rs
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand for a hint.

fn main() {
    let vec0 = Vec::new(); // 在堆上new一个vector

    let mut vec1 = fill_vec(vec0); // 调用函数

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88); // 上面定义vec1是一个不可变调用

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22); // 相当于Vec::push(&mut vec, 22)，其中&mut vec为一个可变借用
    vec.push(44);
    vec.push(66);

    vec // 无分号表达式的返回
}

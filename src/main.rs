
fn main() {
    let mut x: f64 = 0.1; // 初始的 x 值
    let lr: f64 = 0.01; // learning rate, or step size
    let iterations: i32 = 10000; // iteration number

    for _i in 0..iterations {
        x -= lr * gradient(x);
        if _i % 100 == 0{
            println!("Iteration: {}, X value: {}, Function output: {}", _i, x, function(x));
        }
    }
    print!("The local minimum occurs at {}", x);
}

// 我們的函數 f(x) = x^3
fn function(x: f64) -> f64 {
    x * x * x
}

// 函數 f(x) 的梯度 (導數) df(x) = 3x^2
fn gradient(x: f64) -> f64 {
    3.0 * x * x
}

// 首先通过 use 引入标准库中的 env 包，然后 env::args 方法会读取并分析传入的命令行参数，
// 最终通过 collect 方法输出一个集合类型 Vector。
//
// 可能有同学疑惑，为啥不直接引入 args ，例如 use std::env::args ，
// 这样就无需 env::args 来繁琐调用，直接args.collect() 即可。
// 原因很简单，args 方法只会使用一次，啰嗦就啰嗦点吧，
// 把相同的好名字让给 let args.. 这位大哥不好吗？毕竟人家要出场多次的。
use std::{env, process};

use minigrep::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();
    //collect 方法其实并不是std::env包提供的，而是迭代器自带的方法(env::args() 会返回一个迭代器)，
    // 它会将迭代器消费后转换成我们想要的集合类型，关于迭代器和 collect 的具体介绍，请参考这里。
    // 最后，代码中使用 dbg! 宏来输出读取到的数组内容，来看看长啥样：
    // dbg!(args);

    //上面两个版本分别是无参数和两个参数，其中无参数版本实际上也会读取到一个字符串，仔细看，是不是长得很像我们的程序名，
    // Bingo! env::args 读取到的参数中第一个就是程序的可执行路径名。

    //unwrap_or_else 是定义在 Result<T,E> 上的常用方法，如果 Result 是 Ok，
    //那该方法就类似 unwrap：返回 Ok 内部的值；如果是 Err，就调用闭包中的自定义代码对错误进行进一步处理
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        //当 Result 包含错误时，我们不再调用 panic 让程序崩溃，而是通过 process::exit(1) 来终结进程，
        //其中 1 是一个信号值(事实上非 0 值都可以)，通知调用我们程序的进程，程序是因为错误而退出的。
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // let connects = fs::read_to_string(config.file_path)
    //     .expect("Should have been able to read the file");
    //在 Rust 语言中，expect 是一个用于处理 Result 类型的方法。expect 方法接受一个字符串作为参数，
    // 当 Result 类型的值为 Err 时，会将这个字符串作为错误信息打印出来，并终止程序运行。
    // 当 Result 类型的值为 Ok 时，expect 方法会返回 Ok 中的值。
    // 在你的代码中，fs::read_to_string(file_path) 返回的是一个 Result 类型，其中 Ok 包含了读取到的文件内容，
    // 而 Err 包含了一个错误信息。通过调用 expect 方法，你在程序出现错误时会打印出自定义的错误信息，以便更好地了解发生了什么问题。
    // 在你的代码中，.expect("Should have been able to read the file") 表示如果读取文件的操作失败，
    // 会打印出 "Should have been able to read the file" 这个错误信息。
    // println!("With text:\n{connects}")
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
    println!("success")
}



/***
    在 Rust 中，`&` 符号表示引用。在上述代码中，`&args` 表示将 `args`
    变量的引用传递给 `parse_config` 函数。这样做可以避免将整个 `args` 变量的所有权传递给函数，而是只是借用它，从而在函数中访问它的值。
    在 `parse_config` 函数中，`args: &[String]` 表示参数 `args` 是一个 `String` 类型的切片引用。
    切片是一个对数组或向量的引用，它允许你引用集合中的一部分元素而不需要拥有整个集合。
    因此，`parse_config` 函数接受一个 `String` 类型的切片引用作为参数，然后从中提取出查询和文件路径，并返回一个元组包含这两个值。
 */
// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let file_path = args[2].clone();
//     Config { query, file_path }
// }

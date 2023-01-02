// 编译与运行Rust程序
// 编译：rustc main.rs
// 运行：
//  - windows: .\main.rs
//  - mac: ./main

fn main() {
    println!("Hello, world!");
}

/*
定义函数：fn main(){}
    没有参数，没有返回值

main函数很特别：他是每个Rust可执行程序最先运行的代码

打印文本：println!("Hello,world!");
    Rust的缩进是4个空格而不是tab
    println!是一个Rust macro（宏）
        如果是函数的话，就没有!
    "Hello World"是字符串，他是println!的参数
    这行代码以;结尾


编译和运行是单独的两步
运行Rust程序之前必须先编译，命令是：rustc 源文件名
    rustc main.rs

编译成功后，会生成一个二进制文件
    在windows上还会生成一个.pdb文件，里面包含调试信息

Rust是ahead-of-time编译的语言
    可以先编译程序，然后把可执行文件交给别人运行(无需安装Rust)

rustc只适合简单的Rust程序
 */

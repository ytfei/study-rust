// 全局导出呀，不需要用 use 就可以使用
#[macro_export]
macro_rules! foo {
    ($l:tt) => { bar!($l); }
}

#[macro_export]
macro_rules! bar {
    (3) => {
        println!("Here we are, bar!!!");
    }
}
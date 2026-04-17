mod app;
mod context_merge;
mod copy_stage;
mod renderer;
mod spec;
mod user_code;

/// 程序入口：只做错误打印，具体流程在 app::run 中。
fn main() {
    if let Err(e) = app::run() {
        println!("{}", e);
    }
}


// 1、集成测试：在项目根目录创建一个 tests 目录，与 src 同级
use adder;

// 2、运行： cargo test
// 有了三个部分的输出：单元测试、集成测试和文档测试

// 3、运行： cargo test --test integration_test 
// 指定测试函数的名称作为 cargo test 的参数来运行特定集成测试
#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}

// 4、集成测试中的子模块，tests/common/mod.rs 是 Rust 的一种命名规范
mod common;

#[test]
fn it_adds_two2() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
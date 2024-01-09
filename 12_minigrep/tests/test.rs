// 引入minigrep库和相关搜索函数
use minigrep;
use minigrep::search;
use minigrep::search_case_insensitive;

// 定义测试函数，测试大小写敏感的搜索
#[test]
fn case_sensitive() {
    // 定义查询字符串和搜索内容
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    // 断言：执行大小写敏感的搜索，期望结果为包含 "safe, fast, productive." 的向量
    assert_eq!(
        vec!["safe, fast, productive."],
        search(query, contents)
    );
}

// 定义测试函数，测试不区分大小写的搜索
#[test]
fn case_insensitive() {
    // 定义查询字符串和搜索内容
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    // 断言：执行不区分大小写的搜索，期望结果为包含 "Rust:" 和 "Trust me." 的向量
    assert_eq!(
        vec!["Rust:", "Trust me."],
        search_case_insensitive(query, contents)
    );
}

use std::error::Error;
use std::fs;
use std::env;

// 定义Config结构体
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

// 实现Config结构体的相关方法
impl Config {
    // 构造函数，根据输入参数创建Config对象
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        // 检查环境变量，确定是否区分大小写
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config { query, filename, case_sensitive })
    }
}

// 定义run函数，用于执行搜索并输出结果
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // 读取文件内容
    let contents = fs::read_to_string(config.filename)?;

    // 根据区分大小写的需求选择搜索函数
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    // 输出搜索结果
    for line in results {
        println!("{}", line);
    }

    // 返回Ok表示运行成功
    Ok(())
}

// 定义搜索函数，用于在内容中查找包含查询字符串的行
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

// 定义不区分大小写的搜索函数，用于在内容中查找包含查询字符串的行（不考虑大小写）
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // 将查询字符串和内容都转换为小写，以实现不区分大小写的比较
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}
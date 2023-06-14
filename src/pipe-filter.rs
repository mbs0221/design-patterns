// 定义过滤器 trait
trait Filter {
    fn process(&self, input: Vec<String>) -> Vec<String>;
}

// 过滤器1：读取文本文件
struct FileReaderFilter;
impl Filter for FileReaderFilter {
    fn process(&self, input: Vec<String>) -> Vec<String> {
        // 读取文本文件的逻辑
        // ...
        println!("Read file: {:?}", input);
        vec!["This", "is", "a", "text", "file"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    }
}

// 过滤器2：转换为大写
struct UppercaseFilter;
impl Filter for UppercaseFilter {
    fn process(&self, input: Vec<String>) -> Vec<String> {
        // 转换为大写的逻辑
        // ...
        println!("Uppercase: {:?}", input);
        input.iter().map(|s| s.to_uppercase()).collect()
    }
}

// 过滤器3：过滤掉特定单词
struct WordFilter {
    word_to_filter: String,
}
impl Filter for WordFilter {
    fn process(&self, input: Vec<String>) -> Vec<String> {
        // 过滤特定单词的逻辑
        // ...
        println!("Filter word: {:?}", input);
        input
            .iter()
            .filter(|&s| s != &self.word_to_filter)
            .cloned()
            .collect()
    }
}

// 过滤器4：排序
struct SortFilter;
impl Filter for SortFilter {
    fn process(&self, input: Vec<String>) -> Vec<String> {
        // 排序的逻辑
        // ...
        println!("Sort: {:?}", input);
        let mut sorted = input.clone();
        sorted.sort();
        sorted
    }
}

// 过滤器5：写入输出文件
struct FileWriterFilter;
impl Filter for FileWriterFilter {
    fn process(&self, input: Vec<String>) -> Vec<String> {
        // 写入输出文件的逻辑
        // ...
        println!("Write file: {:?}", input);
        input
    }
}

// 管道处理器
struct Pipeline {
    filters: Vec<Box<dyn Filter>>,
}

impl Pipeline {
    fn new() -> Self {
        Pipeline { filters: vec![] }
    }

    fn add_filter(&mut self, filter: Box<dyn Filter>) {
        self.filters.push(filter);
    }

    fn run(&self, input: Vec<String>) -> Vec<String> {
        let mut data = input;
        for filter in &self.filters {
            data = filter.process(data);
        }
        data
    }
}

// 使用示例
fn main() {
    let mut pipeline = Pipeline::new();
    pipeline.add_filter(Box::new(FileReaderFilter));
    pipeline.add_filter(Box::new(UppercaseFilter));
    pipeline.add_filter(Box::new(WordFilter {
        word_to_filter: "text".to_string(),
    }));
    pipeline.add_filter(Box::new(SortFilter));
    pipeline.add_filter(Box::new(FileWriterFilter));

    let input = vec!["This", "is", "a", "text", "file"]
        .iter()
        .map(|s| s.to_string())
        .collect();

    let output = pipeline.run(input);

    println!("Final output: {:?}", output);
}

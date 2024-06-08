//https://stackoverflow.com/questions/42275777/how-to-trace-the-cause-of-an-error-result

#[macro_export]
macro_rules! code_loc {
    () => {
        format!("{}:{}", file!(), line!())
    };
}

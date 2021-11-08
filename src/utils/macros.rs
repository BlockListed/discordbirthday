#[macro_export]
macro_rules! task {
    ( $method : expr ) => {
        tokio::spawn(async move { $method })
    };
}

#[macro_export]
macro_rules! lock {
    ( $method : expr ) => {
        db_lock($method)
    };
}
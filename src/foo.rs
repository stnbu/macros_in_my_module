pub fn bar() {
    println!("bar");
}

macro_rules! bish {
    () => {
        println!("bish");
    };
}

// THIS LINE IS A TOP-SECRET TRICK AND ONLY 2018 KIDS WILL GET THIS
// (https://stackoverflow.com/a/31749071/1808166)
pub(crate) use bish;

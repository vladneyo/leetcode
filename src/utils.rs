#[macro_export]
macro_rules! varname {
    ($i:ident) => {stringify!($i).to_string()};
}

#[macro_export]
macro_rules! printv {
    ($i:ident) => {
        println!("{:?}: {:?}", stringify!($i).to_string(), $i);
    };
}
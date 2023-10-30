/// This is taken from https://stackoverflow.com/questions/46557608/what-is-the-null-pointer-optimization-in-rust#:~:text=the%20null%20pointer%20optimization%20kicks,contains%20a%20non%2Dzero%20pointer.
pub enum WithNullPtrOptimization{
    A,
    B(String),
}

pub enum WithoutNullPtrOptimization{
    A,
    B(u32),
}

#[allow(dead_code)]
fn null_optimization()  {
    println!("{} {}", std::mem::size_of::<WithNullPtrOptimization>(), std::mem::size_of::<String>()); // 24 24
    println!("{} {}", std::mem::size_of::<WithoutNullPtrOptimization>(), std::mem::size_of::<u32>()); // 8 4
}

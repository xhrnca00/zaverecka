fn error_fn() -> Result<i32, String> {
    Ok(5)
}

fn do_stuff(num: i32) -> Result<i32, String> {
    let result = error_fn()?;
    if num == result {
        return Err(format!("{} is equal to {}", num, result));
    }
    Ok(result + num)
}

fn main() -> Result<(), String> {
    let code = do_stuff(5).unwrap();
    println!("Code: {}", code);
    Ok(())
}

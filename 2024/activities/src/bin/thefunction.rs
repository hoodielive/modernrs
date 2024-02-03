use std::thread::sleep;
use std::time::Duration;

fn main()
{
    
    let seconds = 3;

    let useme = greeting(print_time(seconds), || {
        println!("Aight.");
        Ok(())
    });
    
    println!("{:?}", useme);
}

fn greeting<F>(seconds: u64, func: F)->Result<(), Box<dyn std::error::Error>>
where
    F: Fn() -> Result<(), Box<dyn std::error::Error>>
{
    let duration = Duration::from_secs(seconds);
    sleep(duration);
    func()?;
    Ok(())
}

fn print_time(a: u64)->u64
{
    a    
}

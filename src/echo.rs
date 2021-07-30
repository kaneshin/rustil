pub const SYNOPSIS: &str = "echo [OPTION]... [STRING]...";

pub fn run(v: &[String]) -> std::io::Result<()> {
    println!("{}", v.join(" "));
    Ok(())
}

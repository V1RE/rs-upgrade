use subprocess::Exec;

pub fn run() {
    let output = Exec::shell("pnpm ls --global")
        .capture()
        .map(|cmd| cmd.stdout_str());

    match output {
        Ok(out) => print!("{}", out),
        Err(err) => println!("Error: {}", err),
    }
}

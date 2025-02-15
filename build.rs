use {
    std::{env, io},
    winres::WindowsResource,
};

fn main() -> io::Result<()> {
    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        WindowsResource::new()
            .set_icon("assets/icons/icon.ico")
            .compile()?;
    }
    Ok(())
}

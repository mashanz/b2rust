use b2rust::run;

fn main() -> std::io::Result<()> {
    actix_web::rt::System::new().block_on(async { run()?.await })?;
    Ok(())
}


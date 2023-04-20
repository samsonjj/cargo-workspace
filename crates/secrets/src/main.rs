use secrets::get_database_password;

fn main() -> Result<(), secrets::SecretsError> {
    let pass = get_database_password()?;
    dbg!(&pass, pass.len());
    Ok(())
}

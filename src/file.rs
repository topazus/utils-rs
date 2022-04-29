use std::io::Read;
/// read a system file into a string
pub fn read_file<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<String> {
    let mut file = std::fs::File::open(path.as_ref())?;
    let mut contents = String::new();
    // use std::io::Read;
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

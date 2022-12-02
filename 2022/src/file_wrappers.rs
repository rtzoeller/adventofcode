use anyhow::Context;

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Asset;

pub fn get_lines_from_embedded_file(name: &str) -> anyhow::Result<Vec<String>> {
    let file = Asset::get(name).with_context(|| name.to_string())?;
    let contents = std::str::from_utf8(file.data.as_ref())?;
    let lines = contents.lines();
    Ok(lines.map(|s| s.to_string()).collect())
}

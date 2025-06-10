use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use reqwest::blocking::Client;
use reqwest::cookie::Jar;

pub fn get_input(year: u16, day: u8) -> Result<String> {
    let local_path = format!("input/{}/day{:02}.txt", year, day);
    
    if Path::new(&local_path).exists() {
        return fs::read_to_string(&local_path)
            .with_context(|| format!("Failed to read local input file: {}", local_path));
    }
    
    fetch_input_from_web(year, day)
}

fn fetch_input_from_web(year: u16, day: u8) -> Result<String> {
    let cookie_jar_path = "cookies.txt";
    
    if !Path::new(cookie_jar_path).exists() {
        anyhow::bail!("Cookie jar file '{}' not found. Please create it with your AOC session cookie.", cookie_jar_path);
    }
    
    let cookie_content = fs::read_to_string(cookie_jar_path)
        .with_context(|| "Failed to read cookie jar file")?;
    
    let jar = Jar::default();
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    
    for line in cookie_content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if line.contains("session=") {
            jar.add_cookie_str(line, &url.parse().unwrap());
        }
    }
    
    let client = Client::builder()
        .cookie_provider(jar.into())
        .user_agent("github.com/your-username/advent-of-code-all by your-email@example.com")
        .build()
        .context("Failed to build HTTP client")?;
    
    let response = client
        .get(&url)
        .send()
        .with_context(|| format!("Failed to fetch input from {}", url))?;
    
    if !response.status().is_success() {
        anyhow::bail!("HTTP error {}: {}", response.status(), response.status().canonical_reason().unwrap_or("Unknown"));
    }
    
    let content = response
        .text()
        .context("Failed to read response body")?;
    
    Ok(content)
}
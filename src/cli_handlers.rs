use crate::config::Config;
use crate::contexter::{concatenate_files, gather_relevant_files};
use crate::utils::{generate_api_key, hash_api_key};
use log::info;
use std::path::PathBuf;

pub fn handle_gather(
    directory: PathBuf,
    extensions: Vec<String>,
    ignore: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let files = gather_relevant_files(
        directory.to_str().unwrap(),
        extensions.iter().map(AsRef::as_ref).collect(),
        ignore,
    )?;
    let (content, _) = concatenate_files(files)?;
    println!("{}", content);
    Ok(())
}

pub fn handle_config_add_project(
    config: &mut Config,
    name: String,
    path: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    config.add_project(name.clone(), path.clone());
    config.save()?;
    info!("Project '{}' added successfully with path {:?}", name, path);
    Ok(())
}

pub fn handle_config_remove_project(
    config: &mut Config,
    name: String,
) -> Result<(), Box<dyn std::error::Error>> {
    if config.remove_project(&name).is_some() {
        config.save()?;
        info!("Project '{}' removed successfully", name);
    } else {
        println!("Project '{}' not found", name);
    }
    Ok(())
}

pub fn handle_config_generate_key(
    config: &mut Config,
    name: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let new_key = generate_api_key();
    let hashed_key = hash_api_key(&new_key);
    config.add_api_key(name.clone(), hashed_key);
    config.save()?;
    println!("New API key generated for '{}': {}", name, new_key);
    println!("Please store this key securely. It won't be displayed again.");
    info!("New API key generated successfully for '{}'", name);
    Ok(())
}

pub fn handle_config_remove_key(
    config: &mut Config,
    name: String,
) -> Result<(), Box<dyn std::error::Error>> {
    config.remove_api_key(&name);
    config.save()?;
    info!("API key '{}' removed successfully", name);
    Ok(())
}

pub fn handle_config_list_keys(config: &Config) {
    println!("API Keys:");
    for (name, _) in &config.api_keys {
        println!("  {}: {}", name, "*".repeat(40)); // Hide the hashed key in the output
    }
}

pub fn handle_config_set_port(
    config: &mut Config,
    port: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    config.port = port;
    config.save()?;
    info!("Port set to {} successfully", port);
    Ok(())
}

pub fn handle_config_set_address(
    config: &mut Config,
    address: String,
) -> Result<(), Box<dyn std::error::Error>> {
    config.listen_address = address.clone();
    config.save()?;
    info!("Listen address set to {} successfully", address);
    Ok(())
}

pub fn handle_config_list(config: &Config) {
    println!("Current Configuration:");
    println!("Port: {}", config.port);
    println!("Listen Address: {}", config.listen_address);
    println!("Projects:");
    for (name, path) in &config.projects {
        println!("  {}: {:?}", name, path);
    }
    println!("API Keys:");
    for (name, _) in &config.api_keys {
        println!("  {}: {}", name, "*".repeat(40)); // Hide the hashed key in the output
    }
}

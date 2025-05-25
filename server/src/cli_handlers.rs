#![allow(clippy::print_stdout)] // CLI tools appropriately use println! for output

use crate::config::Config;
use crate::contexter::{concatenate_files, gather_relevant_files};
use crate::repo_mapper::RepositoryMapper;
use crate::utils::{generate_api_key, hash_api_key};
use log::info;
use std::fmt::Write;
use std::path::{Path, PathBuf};

pub fn handle_gather(
    directory: &Path,
    extensions: &[String],
    ignore: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let files = gather_relevant_files(
        directory
            .to_str()
            .expect("Directory path should be valid UTF-8"),
        &extensions.iter().map(AsRef::as_ref).collect::<Vec<_>>(),
        ignore,
    )?;
    let (content, _) = concatenate_files(files)?;
    println!("{content}");
    Ok(())
}

pub fn handle_config_add_project(
    config: &mut Config,
    name: &str,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    config.add_project(name.to_string(), path.to_path_buf());
    config.save()?;
    info!(
        "Project '{name}' added successfully with path {}",
        path.display()
    );
    Ok(())
}

pub fn handle_config_remove_project(
    config: &mut Config,
    name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if config.remove_project(name).is_some() {
        config.save()?;
        info!("Project '{name}' removed successfully");
    } else {
        println!("Project '{name}' not found");
    }
    Ok(())
}

pub fn handle_config_generate_key(
    config: &mut Config,
    name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let new_key = generate_api_key();
    let hashed_key = hash_api_key(&new_key);
    config.add_api_key(name.to_string(), hashed_key);
    config.save()?;
    println!("New API key generated for '{name}': {new_key}");
    println!("Please store this key securely. It won't be displayed again.");
    info!("New API key generated successfully for '{name}'");
    Ok(())
}

pub fn handle_config_remove_key(
    config: &mut Config,
    name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    config.remove_api_key(name);
    config.save()?;
    info!("API key '{name}' removed successfully");
    Ok(())
}

pub fn handle_config_list_keys(config: &Config) {
    println!("API Keys:");
    for name in config.api_keys.keys() {
        println!("  {}: {}", name, "*".repeat(40)); // Hide the hashed key in the output
    }
}

pub fn handle_config_set_port(
    config: &mut Config,
    port: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    config.port = port;
    config.save()?;
    info!("Port set to {port} successfully");
    Ok(())
}

pub fn handle_config_set_address(
    config: &mut Config,
    address: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    config.listen_address.clone_from(&address.to_string());
    config.save()?;
    info!("Listen address set to {address} successfully");
    Ok(())
}

pub fn handle_config_list(config: &Config) {
    println!("Current Configuration:");
    println!("Port: {}", config.port);
    println!("Listen Address: {}", config.listen_address);
    println!("Projects:");
    for (name, path) in &config.projects {
        println!("  {name}: {}", path.display());
    }
    println!("API Keys:");
    for name in config.api_keys.keys() {
        println!("  {}: {}", name, "*".repeat(40)); // Hide the hashed key in the output
    }
}

// Repository mapping handlers

#[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
pub fn handle_repo_map_generate(
    path: &PathBuf,
    show_dependencies: bool,
    show_order: bool,
    output: Option<PathBuf>,
    json_format: bool,
    focus_component: Option<&String>,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Generating repository map for: {}", path.display());

    let mut mapper = RepositoryMapper::new();
    mapper.analyze_repository(path)?;

    let mut result = String::new();

    if json_format {
        // JSON output with all the data
        let json_data = serde_json::json!({
            "repository": path,
            "components": mapper.graph.components.len(),
            "entry_points": mapper.insights.entry_points,
            "cycles": mapper.graph.cycles.len(),
            "most_connected": mapper.insights.most_connected_components,
            "processing_order": if show_order { Some(&mapper.topological_order) } else { None },
            "dependency_graph": if show_dependencies { Some(&mapper.graph.edges) } else { None },
            "focus": focus_component.and_then(|comp| {
                mapper.graph.components.get(comp).map(|c| serde_json::json!({
                    "component": c,
                    "dependencies": mapper.get_dependencies(comp),
                    "dependents": mapper.get_dependents(comp)
                }))
            })
        });
        result = serde_json::to_string_pretty(&json_data)?;
    } else {
        // Clean, focused text output
        result.push_str("Repository Map\n");
        result.push_str("==============\n\n");

        // Quick stats
        writeln!(&mut result, "📍 {}", path.display())?;
        write!(
            &mut result,
            "📊 {} components, {} entry points, {} cycles\n\n",
            mapper.insights.total_components,
            mapper.insights.entry_points.len(),
            mapper.graph.cycles.len()
        )?;

        // Focus on specific component if requested
        if let Some(focus) = focus_component {
            if let Some(component) = mapper.graph.components.values().find(|c| c.name == *focus) {
                writeln!(&mut result, "🎯 Focus: {}", component.name)?;
                writeln!(
                    &mut result,
                    "   Type: {:?} | Visibility: {:?}",
                    component.component_type, component.visibility
                )?;
                writeln!(&mut result, "   File: {}", component.file_path.display())?;
                write!(
                    &mut result,
                    "   Dependencies: {} | Used by: {}\n\n",
                    component.dependencies.len(),
                    component.dependents.len()
                )?;

                if !component.dependencies.is_empty() {
                    result.push_str("Dependencies:\n");
                    for dep_id in &component.dependencies {
                        if let Some(dep) = mapper.graph.components.get(dep_id) {
                            writeln!(
                                &mut result,
                                "  → {} ({})",
                                dep.name,
                                dep.file_path.display()
                            )?;
                        }
                    }
                    result.push('\n');
                }

                if !component.dependents.is_empty() {
                    result.push_str("Used by:\n");
                    for dep_id in &component.dependents {
                        if let Some(dep) = mapper.graph.components.get(dep_id) {
                            writeln!(
                                &mut result,
                                "  ← {} ({})",
                                dep.name,
                                dep.file_path.display()
                            )?;
                        }
                    }
                    result.push('\n');
                }
            } else {
                write!(&mut result, "Component '{focus}' not found\n\n")?;
            }
        }

        // Group components by file for clean overview
        let mut file_groups: std::collections::HashMap<
            PathBuf,
            Vec<&crate::repo_mapper::CodeComponent>,
        > = std::collections::HashMap::new();
        for component in mapper.graph.components.values() {
            file_groups
                .entry(component.file_path.clone())
                .or_default()
                .push(component);
        }

        result.push_str("Structure:\n");
        for (file_path, components) in file_groups {
            writeln!(&mut result, "  {}", file_path.display())?;
            for component in components {
                let type_indicator = match component.component_type {
                    crate::repo_mapper::ComponentType::Function
                    | crate::repo_mapper::ComponentType::Method => "fn",
                    crate::repo_mapper::ComponentType::Class => "struct",
                    crate::repo_mapper::ComponentType::Module => "mod",
                    crate::repo_mapper::ComponentType::Interface => "trait",
                };
                let visibility = match component.visibility {
                    crate::repo_mapper::Visibility::Public => "pub",
                    _ => "",
                };
                writeln!(
                    &mut result,
                    "    {} {} {} ({}→{})",
                    visibility,
                    type_indicator,
                    component.name,
                    component.dependencies.len(),
                    component.dependents.len()
                )?;
            }
        }
        result.push('\n');

        // Show entry points (important for understanding)
        if !mapper.insights.entry_points.is_empty() {
            result.push_str("Entry Points:\n");
            for entry_id in &mapper.insights.entry_points {
                if let Some(entry) = mapper.graph.components.get(entry_id) {
                    writeln!(
                        &mut result,
                        "  {} ({})",
                        entry.name,
                        entry.file_path.display()
                    )?;
                }
            }
            result.push('\n');
        }

        // Show most connected components (architectural insights)
        if !mapper.insights.most_connected_components.is_empty() {
            result.push_str("Key Components:\n");
            for (i, comp_id) in mapper
                .insights
                .most_connected_components
                .iter()
                .take(5)
                .enumerate()
            {
                if let Some(component) = mapper.graph.components.get(comp_id) {
                    writeln!(
                        &mut result,
                        "  {}. {} ({})",
                        i + 1,
                        component.name,
                        component.file_path.display()
                    )?;
                }
            }
            result.push('\n');
        }

        // Show processing order if requested
        if show_order {
            result.push_str("Processing Order (dependencies first):\n");
            for (i, component_id) in mapper.topological_order.iter().take(10).enumerate() {
                if let Some(component) = mapper.graph.components.get(component_id) {
                    writeln!(&mut result, "  {}. {}", i + 1, component.name)?;
                }
            }
            if mapper.topological_order.len() > 10 {
                writeln!(
                    &mut result,
                    "  ... and {} more",
                    mapper.topological_order.len() - 10
                )?;
            }
            result.push('\n');
        }

        // Show dependency details if requested
        if show_dependencies && !mapper.graph.edges.is_empty() {
            result.push_str("Dependencies:\n");
            for edge in mapper.graph.edges.iter().take(10) {
                if let (Some(from), Some(to)) = (
                    mapper.graph.components.get(&edge.from),
                    mapper.graph.components.get(&edge.to),
                ) {
                    writeln!(&mut result, "  {} → {}", from.name, to.name)?;
                }
            }
            if mapper.graph.edges.len() > 10 {
                writeln!(
                    &mut result,
                    "  ... and {} more",
                    mapper.graph.edges.len() - 10
                )?;
            }
        }
    }

    if let Some(output_path) = output {
        std::fs::write(output_path, &result)?;
        println!("Repository map saved");
    } else {
        println!("{result}");
    }

    Ok(())
}

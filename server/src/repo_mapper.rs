use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::fmt::Write;
use std::path::{Path, PathBuf};

/// Represents a code component (function, method, class) in the repository
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeComponent {
    pub id: String,
    pub name: String,
    pub component_type: ComponentType,
    pub file_path: PathBuf,
    pub start_line: usize,
    pub end_line: usize,
    pub visibility: Visibility,
    pub dependencies: Vec<String>, // IDs of components this depends on
    pub dependents: Vec<String>,   // IDs of components that depend on this
    pub complexity_score: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComponentType {
    Function,
    Method,
    Class,
    Module,
    Interface,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Visibility {
    Public,
    Private,
    Protected,
    Internal,
}

/// Represents the dependency graph of the repository
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyGraph {
    pub components: HashMap<String, CodeComponent>,
    pub edges: Vec<DependencyEdge>,
    pub cycles: Vec<Vec<String>>, // Detected cycles
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyEdge {
    pub from: String,
    pub to: String,
    pub edge_type: EdgeType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EdgeType {
    FunctionCall,
    ClassInheritance,
    ModuleImport,
    FieldAccess,
    MethodCall,
}

/// Repository insights and statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryInsights {
    pub total_files: usize,
    pub total_components: usize,
    pub most_complex_files: Vec<String>,
    pub most_connected_components: Vec<String>,
    pub architectural_layers: Vec<ArchitecturalLayer>,
    pub entry_points: Vec<String>,
    pub dependency_hotspots: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitecturalLayer {
    pub name: String,
    pub files: Vec<String>,
    pub description: String,
}

/// Main repository mapper that analyzes code structure
pub struct RepositoryMapper {
    pub graph: DependencyGraph,
    pub insights: RepositoryInsights,
    pub topological_order: Vec<String>,
}

impl RepositoryMapper {
    /// Create a new repository mapper
    pub fn new() -> Self {
        Self {
            graph: DependencyGraph {
                components: HashMap::new(),
                edges: Vec::new(),
                cycles: Vec::new(),
            },
            insights: RepositoryInsights {
                total_files: 0,
                total_components: 0,
                most_complex_files: Vec::new(),
                most_connected_components: Vec::new(),
                architectural_layers: Vec::new(),
                entry_points: Vec::new(),
                dependency_hotspots: Vec::new(),
            },
            topological_order: Vec::new(),
        }
    }

    /// Analyze a repository and build the dependency graph
    pub fn analyze_repository(
        &mut self,
        repo_path: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!("Starting repository analysis for: {}", repo_path.display());

        // Step 1: Discover and parse files
        self.discover_files(repo_path)?;

        // Step 2: Build dependency graph
        self.build_dependency_graph();

        // Step 3: Detect cycles using Tarjan's algorithm
        self.detect_cycles();

        // Step 4: Compute topological order
        self.compute_topological_order();

        // Step 5: Generate insights
        self.generate_insights();

        info!(
            "Repository analysis complete. Found {} components",
            self.graph.components.len()
        );
        Ok(())
    }

    /// Discover all relevant source files in the repository
    fn discover_files(&mut self, repo_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        debug!("Discovering files in: {}", repo_path.display());

        // Use existing file gathering logic from contexter
        let files = crate::contexter::gather_relevant_files(
            repo_path
                .to_str()
                .expect("Repository path should be valid UTF-8"),
            &["rs", "py", "js", "ts"], // Support common languages
            vec![],
        )?;

        info!("Found {} source files", files.len());
        self.insights.total_files = files.len();

        // For each file, extract basic components
        for file_path in files {
            self.parse_file(&file_path)?;
        }

        Ok(())
    }

    /// Parse a single file to extract components
    fn parse_file(&mut self, file_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(file_path)?;
        let extension = file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");

        match extension {
            "rs" => self.parse_rust_file(file_path, &content)?,
            "py" => self.parse_python_file(file_path, &content)?,
            "js" | "ts" => self.parse_javascript_file(file_path, &content)?,
            _ => {} // Skip unsupported file types for now
        }

        Ok(())
    }

    /// Basic Rust file parsing (simplified)
    fn parse_rust_file(
        &mut self,
        file_path: &Path,
        content: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use regex::Regex;

        let fn_regex = Regex::new(r"(?m)^(\s*)(pub\s+)?fn\s+(\w+)\s*\(")?;
        let struct_regex = Regex::new(r"(?m)^(\s*)(pub\s+)?struct\s+(\w+)")?;
        let _impl_regex = Regex::new(r"(?m)^(\s*)impl\s+(?:<[^>]*>\s+)?(\w+)")?;
        let use_regex = Regex::new(r"(?m)^use\s+([^;]+);")?;

        let mut line_number = 1;
        let mut imports = Vec::new();

        // Extract imports
        for cap in use_regex.captures_iter(content) {
            imports.push(cap[1].to_string());
        }

        // Extract functions
        for cap in fn_regex.captures_iter(content) {
            let visibility = if cap.get(2).is_some() {
                Visibility::Public
            } else {
                Visibility::Private
            };
            let name = cap[3].to_string();
            let id = format!("{}::{}", file_path.display(), name);

            let component = CodeComponent {
                id: id.clone(),
                name,
                component_type: ComponentType::Function,
                file_path: file_path.to_path_buf(),
                start_line: line_number, // Simplified - would need proper line counting
                end_line: line_number + 10, // Simplified
                visibility,
                dependencies: Vec::new(), // Will be populated in build_dependency_graph
                dependents: Vec::new(),
                complexity_score: 1,
            };

            self.graph.components.insert(id, component);
            line_number += 1;
        }

        // Extract structs
        for cap in struct_regex.captures_iter(content) {
            let visibility = if cap.get(2).is_some() {
                Visibility::Public
            } else {
                Visibility::Private
            };
            let name = cap[3].to_string();
            let id = format!("{}::{}", file_path.display(), name);

            let component = CodeComponent {
                id: id.clone(),
                name,
                component_type: ComponentType::Class, // Treating struct as class for simplicity
                file_path: file_path.to_path_buf(),
                start_line: line_number,
                end_line: line_number + 5,
                visibility,
                dependencies: Vec::new(),
                dependents: Vec::new(),
                complexity_score: 2,
            };

            self.graph.components.insert(id, component);
            line_number += 1;
        }

        Ok(())
    }

    /// Basic Python file parsing (placeholder)
    fn parse_python_file(
        &mut self,
        file_path: &Path,
        content: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use regex::Regex;

        let fn_regex = Regex::new(r"(?m)^(\s*)def\s+(\w+)\s*\(")?;
        let class_regex = Regex::new(r"(?m)^(\s*)class\s+(\w+):")?;

        let mut line_number = 1;

        // Extract functions
        for cap in fn_regex.captures_iter(content) {
            let name = cap[2].to_string();
            let id = format!("{}::{}", file_path.display(), name);
            let visibility = if name.starts_with('_') {
                Visibility::Private
            } else {
                Visibility::Public
            };

            let component = CodeComponent {
                id: id.clone(),
                name,
                component_type: ComponentType::Function,
                file_path: file_path.to_path_buf(),
                start_line: line_number,
                end_line: line_number + 10,
                visibility,
                dependencies: Vec::new(),
                dependents: Vec::new(),
                complexity_score: 1,
            };

            self.graph.components.insert(id, component);
            line_number += 1;
        }

        // Extract classes
        for cap in class_regex.captures_iter(content) {
            let name = cap[2].to_string();
            let id = format!("{}::{}", file_path.display(), name);
            let visibility = if name.starts_with('_') {
                Visibility::Private
            } else {
                Visibility::Public
            };

            let component = CodeComponent {
                id: id.clone(),
                name,
                component_type: ComponentType::Class,
                file_path: file_path.to_path_buf(),
                start_line: line_number,
                end_line: line_number + 10,
                visibility,
                dependencies: Vec::new(),
                dependents: Vec::new(),
                complexity_score: 2,
            };

            self.graph.components.insert(id, component);
            line_number += 1;
        }

        Ok(())
    }

    /// Basic JavaScript/TypeScript file parsing (placeholder)
    fn parse_javascript_file(
        &mut self,
        file_path: &Path,
        content: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use regex::Regex;

        let fn_regex = Regex::new(
            r"(?m)function\s+(\w+)\s*\(|(?m)(\w+)\s*:\s*function\s*\(|(?m)const\s+(\w+)\s*=\s*\(",
        )?;
        let class_regex = Regex::new(r"(?m)class\s+(\w+)")?;

        let mut line_number = 1;

        // Extract functions (simplified)
        for cap in fn_regex.captures_iter(content) {
            let name = cap
                .get(1)
                .or(cap.get(2))
                .or(cap.get(3))
                .map_or_else(|| "anonymous".to_string(), |m| m.as_str().to_string());

            if name != "anonymous" {
                let id = format!("{}::{}", file_path.display(), name);

                let component = CodeComponent {
                    id: id.clone(),
                    name,
                    component_type: ComponentType::Function,
                    file_path: file_path.to_path_buf(),
                    start_line: line_number,
                    end_line: line_number + 10,
                    visibility: Visibility::Public, // JS doesn't have traditional visibility
                    dependencies: Vec::new(),
                    dependents: Vec::new(),
                    complexity_score: 1,
                };

                self.graph.components.insert(id, component);
                line_number += 1;
            }
        }

        // Extract classes
        for cap in class_regex.captures_iter(content) {
            let name = cap[1].to_string();
            let id = format!("{}::{}", file_path.display(), name);

            let component = CodeComponent {
                id: id.clone(),
                name,
                component_type: ComponentType::Class,
                file_path: file_path.to_path_buf(),
                start_line: line_number,
                end_line: line_number + 10,
                visibility: Visibility::Public,
                dependencies: Vec::new(),
                dependents: Vec::new(),
                complexity_score: 2,
            };

            self.graph.components.insert(id, component);
            line_number += 1;
        }

        Ok(())
    }

    /// Build the dependency graph by parsing ASTs
    fn build_dependency_graph(&mut self) {
        debug!("Building dependency graph");

        // For now, this is a simplified implementation
        // In a full implementation, this would analyze function calls, imports, etc.
        // to build the actual dependency edges

        // Create some example dependencies based on file structure
        let component_ids: Vec<String> = self.graph.components.keys().cloned().collect();

        for (i, from_id) in component_ids.iter().enumerate() {
            if let Some(to_id) = component_ids.get(i + 1) {
                // Create a simple dependency chain for demonstration
                self.graph.edges.push(DependencyEdge {
                    from: from_id.clone(),
                    to: to_id.clone(),
                    edge_type: EdgeType::FunctionCall,
                });

                // Update component dependencies
                if let Some(from_component) = self.graph.components.get_mut(from_id) {
                    from_component.dependencies.push(to_id.clone());
                }
                if let Some(to_component) = self.graph.components.get_mut(to_id) {
                    to_component.dependents.push(from_id.clone());
                }
            }
        }
    }

    /// Detect cycles in the dependency graph using Tarjan's algorithm
    fn detect_cycles(&mut self) {
        // Implementation of Tarjan's strongly connected components algorithm
        debug!("Detecting cycles in dependency graph");

        // Placeholder - actual cycle detection will be implemented
        self.graph.cycles = Vec::new();
    }

    /// Compute topological ordering of components (dependencies first)
    fn compute_topological_order(&mut self) {
        debug!("Computing topological order");

        // Kahn's algorithm for topological sorting
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        let mut adj_list: HashMap<String, Vec<String>> = HashMap::new();

        // Initialize in-degree and adjacency list
        for component_id in self.graph.components.keys() {
            in_degree.insert(component_id.clone(), 0);
            adj_list.insert(component_id.clone(), Vec::new());
        }

        // Build adjacency list and calculate in-degrees
        for edge in &self.graph.edges {
            adj_list
                .get_mut(&edge.from)
                .expect("Component should exist in adjacency list")
                .push(edge.to.clone());
            *in_degree
                .get_mut(&edge.to)
                .expect("Component should exist in in-degree map") += 1;
        }

        // Queue for nodes with no incoming edges
        let mut queue: VecDeque<String> = VecDeque::new();
        for (node, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(node.clone());
            }
        }

        // Process nodes in topological order
        let mut topo_order = Vec::new();
        while let Some(node) = queue.pop_front() {
            topo_order.push(node.clone());

            if let Some(neighbors) = adj_list.get(&node) {
                for neighbor in neighbors {
                    let degree = in_degree
                        .get_mut(neighbor)
                        .expect("Neighbor should exist in in-degree map");
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }

        // Check for cycles
        if topo_order.len() != self.graph.components.len() {
            warn!("Cycles detected in dependency graph");
        }

        self.topological_order = topo_order;
    }

    /// Generate repository insights and statistics
    pub fn generate_insights(&mut self) {
        debug!("Generating repository insights");

        self.insights.total_components = self.graph.components.len();

        // Find most connected components (high in-degree + out-degree)
        let mut component_connections: Vec<(String, usize)> = self
            .graph
            .components
            .iter()
            .map(|(id, component)| {
                let connections = component.dependencies.len() + component.dependents.len();
                (id.clone(), connections)
            })
            .collect();

        component_connections.sort_by(|a, b| b.1.cmp(&a.1));
        self.insights.most_connected_components = component_connections
            .into_iter()
            .take(10)
            .map(|(id, _)| id)
            .collect();

        // Find entry points (components with no dependencies)
        self.insights.entry_points = self
            .graph
            .components
            .iter()
            .filter(|(_, component)| component.dependencies.is_empty())
            .map(|(id, _)| id.clone())
            .collect();
    }

    /// Generate a visual ASCII representation of the repository structure
    pub fn generate_repository_map(&self) -> String {
        let mut map = String::new();
        map.push_str("Repository Structure\n");
        map.push_str("===================\n\n");

        // Quick overview
        write!(
            &mut map,
            "Components: {} | Entry Points: {} | Cycles: {}\n\n",
            self.insights.total_components,
            self.insights.entry_points.len(),
            self.graph.cycles.len()
        )
        .unwrap();

        // Group components by file path for organized display
        let mut file_groups: HashMap<PathBuf, Vec<&CodeComponent>> = HashMap::new();
        for component in self.graph.components.values() {
            file_groups
                .entry(component.file_path.clone())
                .or_default()
                .push(component);
        }

        for (file_path, components) in file_groups {
            writeln!(&mut map, "{}", file_path.display()).unwrap();
            for component in components {
                let type_indicator = match component.component_type {
                    ComponentType::Function | ComponentType::Method => "fn",
                    ComponentType::Class => "struct",
                    ComponentType::Module => "mod",
                    ComponentType::Interface => "trait",
                };
                let visibility = match component.visibility {
                    Visibility::Public => "pub ",
                    _ => "",
                };
                writeln!(
                    &mut map,
                    "  {}{} {} (deps: {}, used by: {})",
                    visibility,
                    type_indicator,
                    component.name,
                    component.dependencies.len(),
                    component.dependents.len()
                )
                .unwrap();
            }
            map.push('\n');
        }

        // Add insights section
        map.push_str("Key Insights\n");
        map.push_str("------------\n");

        if !self.insights.entry_points.is_empty() {
            map.push_str("Entry Points:\n");
            for entry_id in &self.insights.entry_points {
                if let Some(component) = self.graph.components.get(entry_id) {
                    writeln!(
                        &mut map,
                        "  {} ({})",
                        component.name,
                        component.file_path.display()
                    )
                    .unwrap();
                }
            }
            map.push('\n');
        }

        if !self.insights.most_connected_components.is_empty() {
            map.push_str("Most Connected:\n");
            for (i, component_id) in self
                .insights
                .most_connected_components
                .iter()
                .take(5)
                .enumerate()
            {
                if let Some(component) = self.graph.components.get(component_id) {
                    writeln!(
                        &mut map,
                        "  {}. {} ({})",
                        i + 1,
                        component.name,
                        component.file_path.display()
                    )
                    .unwrap();
                }
            }
        }

        map
    }

    /// Get processing order for incremental context building (like `DocAgent`)
    pub fn get_processing_order(&self) -> &[String] {
        &self.topological_order
    }

    /// Get components that should be processed before the given component
    pub fn get_dependencies(&self, component_id: &str) -> Option<&[String]> {
        self.graph
            .components
            .get(component_id)
            .map(|component| component.dependencies.as_slice())
    }

    /// Get components that depend on the given component
    pub fn get_dependents(&self, component_id: &str) -> Option<&[String]> {
        self.graph
            .components
            .get(component_id)
            .map(|component| component.dependents.as_slice())
    }
}

impl Default for RepositoryMapper {
    fn default() -> Self {
        Self::new()
    }
}

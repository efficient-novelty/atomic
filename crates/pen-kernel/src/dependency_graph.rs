use std::collections::{BTreeMap, BTreeSet};

const GRAPH_SCHEMA: &str = "pen-kernel-production-dependency-graph-v1";
const ACTIVE_WORKSPACE_LOCK: &str = include_str!("../../../Cargo.lock");
const KERNEL_MANIFEST: &str = include_str!("../Cargo.toml");
const REVIEWED_GRAPH: &str = include_str!("../production-dependency-graph.lock");

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct PackageId {
    name: String,
    version: String,
    source: Option<String>,
}

#[derive(Clone, Debug)]
struct LockPackage {
    id: PackageId,
    checksum: Option<String>,
    dependencies: Vec<String>,
}

#[derive(Default)]
struct LockPackageBuilder {
    name: Option<String>,
    version: Option<String>,
    source: Option<String>,
    checksum: Option<String>,
    dependencies: Vec<String>,
}

impl LockPackageBuilder {
    fn finish(self) -> Result<LockPackage, String> {
        Ok(LockPackage {
            id: PackageId {
                name: self
                    .name
                    .ok_or_else(|| "Cargo.lock package has no name".to_owned())?,
                version: self
                    .version
                    .ok_or_else(|| "Cargo.lock package has no version".to_owned())?,
                source: self.source,
            },
            checksum: self.checksum,
            dependencies: self.dependencies,
        })
    }
}

#[derive(Clone, Debug)]
struct DependencyReference {
    name: String,
    version: Option<String>,
    source: Option<String>,
}

pub(crate) fn reviewed_production_dependency_graph() -> Vec<u8> {
    let active = canonical_production_dependency_graph(ACTIVE_WORKSPACE_LOCK, KERNEL_MANIFEST)
        .unwrap_or_else(|error| panic!("cannot bind the kernel dependency graph: {error}"));
    let reviewed = normalize_newlines(REVIEWED_GRAPH).into_bytes();
    assert_eq!(
        active, reviewed,
        "the active Cargo.lock does not match \
         crates/pen-kernel/production-dependency-graph.lock"
    );
    active
}

fn canonical_production_dependency_graph(
    lockfile: &str,
    manifest: &str,
) -> Result<Vec<u8>, String> {
    let packages = parse_lockfile(lockfile)?;
    let mut package_by_id = BTreeMap::new();
    for package in &packages {
        if package_by_id.insert(package.id.clone(), package).is_some() {
            return Err(format!(
                "Cargo.lock contains duplicate package identity {} {}",
                package.id.name, package.id.version
            ));
        }
    }
    let kernel = packages
        .iter()
        .filter(|package| package.id.name == "pen-kernel" && package.id.source.is_none())
        .collect::<Vec<_>>();
    let [kernel] = kernel.as_slice() else {
        return Err(format!(
            "expected one local pen-kernel package in Cargo.lock, found {}",
            kernel.len()
        ));
    };

    let direct_names = production_dependency_names(manifest)?;
    let root_references = kernel
        .dependencies
        .iter()
        .map(|dependency| parse_dependency_reference(dependency))
        .collect::<Result<Vec<_>, _>>()?;
    let mut roots = BTreeSet::new();
    for name in direct_names {
        let references = root_references
            .iter()
            .filter(|reference| reference.name == name)
            .collect::<Vec<_>>();
        let [reference] = references.as_slice() else {
            return Err(format!(
                "expected one resolved reference for direct dependency {name}, found {}",
                references.len()
            ));
        };
        roots.insert(resolve_reference(reference, &package_by_id)?);
    }

    let mut closure = BTreeSet::new();
    let mut edges = BTreeSet::new();
    let mut frontier = roots.iter().cloned().collect::<Vec<_>>();
    while let Some(id) = frontier.pop() {
        if !closure.insert(id.clone()) {
            continue;
        }
        let package = package_by_id
            .get(&id)
            .ok_or_else(|| format!("resolved package disappeared: {}", id.name))?;
        validate_bound_source(package)?;
        for dependency in &package.dependencies {
            let child =
                resolve_reference(&parse_dependency_reference(dependency)?, &package_by_id)?;
            edges.insert((id.clone(), child.clone()));
            frontier.push(child);
        }
    }

    let mut output = String::new();
    output.push_str("schema\t");
    output.push_str(GRAPH_SCHEMA);
    output.push('\n');
    for root in roots {
        push_id_line(&mut output, "root", &root, None)?;
    }
    for id in &closure {
        let package = package_by_id
            .get(id)
            .ok_or_else(|| format!("resolved package disappeared: {}", id.name))?;
        push_id_line(
            &mut output,
            "package",
            id,
            Some(package.checksum.as_deref().unwrap_or("-")),
        )?;
    }
    for (parent, child) in edges {
        push_edge_line(&mut output, &parent, &child)?;
    }
    Ok(output.into_bytes())
}

fn validate_bound_source(package: &LockPackage) -> Result<(), String> {
    let source = package.id.source.as_deref().ok_or_else(|| {
        format!(
            "production dependency {} {} has no immutable source identity",
            package.id.name, package.id.version
        )
    })?;
    if source.starts_with("registry+") && package.checksum.is_none() {
        return Err(format!(
            "registry dependency {} {} has no checksum",
            package.id.name, package.id.version
        ));
    }
    Ok(())
}

fn push_id_line(
    output: &mut String,
    kind: &str,
    id: &PackageId,
    checksum: Option<&str>,
) -> Result<(), String> {
    let source = id
        .source
        .as_deref()
        .ok_or_else(|| format!("{} {} has no source", id.name, id.version))?;
    for value in [&id.name, &id.version, source] {
        validate_column(value)?;
    }
    output.push_str(kind);
    output.push('\t');
    output.push_str(&id.name);
    output.push('\t');
    output.push_str(&id.version);
    output.push('\t');
    output.push_str(source);
    if let Some(checksum) = checksum {
        validate_column(checksum)?;
        output.push('\t');
        output.push_str(checksum);
    }
    output.push('\n');
    Ok(())
}

fn push_edge_line(
    output: &mut String,
    parent: &PackageId,
    child: &PackageId,
) -> Result<(), String> {
    let parent_source = parent
        .source
        .as_deref()
        .ok_or_else(|| format!("{} {} has no source", parent.name, parent.version))?;
    let child_source = child
        .source
        .as_deref()
        .ok_or_else(|| format!("{} {} has no source", child.name, child.version))?;
    for value in [
        &parent.name,
        &parent.version,
        parent_source,
        &child.name,
        &child.version,
        child_source,
    ] {
        validate_column(value)?;
    }
    output.push_str("dependency\t");
    output.push_str(&parent.name);
    output.push('\t');
    output.push_str(&parent.version);
    output.push('\t');
    output.push_str(parent_source);
    output.push('\t');
    output.push_str(&child.name);
    output.push('\t');
    output.push_str(&child.version);
    output.push('\t');
    output.push_str(child_source);
    output.push('\n');
    Ok(())
}

fn validate_column(value: &str) -> Result<(), String> {
    if value.contains(['\t', '\r', '\n']) {
        return Err("dependency graph value contains a forbidden delimiter".to_owned());
    }
    Ok(())
}

fn production_dependency_names(manifest: &str) -> Result<BTreeSet<String>, String> {
    let mut names = BTreeSet::new();
    let mut production_section = false;
    for raw_line in manifest.lines() {
        let line = raw_line.split('#').next().unwrap_or_default().trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('[') {
            if !line.ends_with(']') {
                return Err(format!("malformed Cargo.toml section: {line}"));
            }
            let section = &line[1..line.len() - 1];
            if section.starts_with("dependencies.")
                || section.starts_with("build-dependencies.")
                || (section.starts_with("target.")
                    && (section.contains(".dependencies.")
                        || section.contains(".build-dependencies.")))
            {
                return Err(format!(
                    "table-form dependency declarations are not supported: [{section}]"
                ));
            }
            production_section = section == "dependencies"
                || section == "build-dependencies"
                || (section.starts_with("target.")
                    && (section.ends_with(".dependencies")
                        || section.ends_with(".build-dependencies")));
            continue;
        }
        if !production_section {
            continue;
        }
        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| format!("malformed dependency declaration: {line}"))?;
        let key = key.trim().trim_matches('"');
        if key.is_empty() {
            return Err("dependency name is empty".to_owned());
        }
        if value.contains("optional") && value.contains("true") {
            return Err(format!(
                "optional direct dependency {key} requires explicit feature-resolution binding"
            ));
        }
        let actual_name = inline_package_name(value)?.unwrap_or_else(|| key.to_owned());
        names.insert(actual_name);
    }
    if names.is_empty() {
        return Err("kernel manifest has no production dependencies".to_owned());
    }
    Ok(names)
}

fn inline_package_name(value: &str) -> Result<Option<String>, String> {
    let Some(offset) = value.find("package") else {
        return Ok(None);
    };
    let suffix = &value[offset + "package".len()..];
    let (_, encoded) = suffix
        .split_once('=')
        .ok_or_else(|| format!("malformed package alias: {value}"))?;
    Ok(Some(parse_toml_string(
        encoded.trim().trim_end_matches(','),
    )?))
}

fn parse_lockfile(lockfile: &str) -> Result<Vec<LockPackage>, String> {
    let mut packages = Vec::new();
    let mut current: Option<LockPackageBuilder> = None;
    let mut reading_dependencies = false;
    for raw_line in lockfile.lines() {
        let line = raw_line.trim();
        if line == "[[package]]" {
            if reading_dependencies {
                return Err("unterminated Cargo.lock dependency array".to_owned());
            }
            if let Some(builder) = current.take() {
                packages.push(builder.finish()?);
            }
            current = Some(LockPackageBuilder::default());
            continue;
        }
        let Some(builder) = current.as_mut() else {
            continue;
        };
        if reading_dependencies {
            if line == "]" {
                reading_dependencies = false;
                continue;
            }
            builder
                .dependencies
                .push(parse_toml_string(line.trim_end_matches(','))?);
            continue;
        }
        if line == "dependencies = [" {
            reading_dependencies = true;
        } else if let Some(value) = assignment_value(line, "name") {
            builder.name = Some(parse_toml_string(value)?);
        } else if let Some(value) = assignment_value(line, "version") {
            builder.version = Some(parse_toml_string(value)?);
        } else if let Some(value) = assignment_value(line, "source") {
            builder.source = Some(parse_toml_string(value)?);
        } else if let Some(value) = assignment_value(line, "checksum") {
            builder.checksum = Some(parse_toml_string(value)?);
        }
    }
    if reading_dependencies {
        return Err("unterminated Cargo.lock dependency array".to_owned());
    }
    if let Some(builder) = current {
        packages.push(builder.finish()?);
    }
    if packages.is_empty() {
        return Err("Cargo.lock has no package records".to_owned());
    }
    Ok(packages)
}

fn assignment_value<'a>(line: &'a str, field: &str) -> Option<&'a str> {
    let (key, value) = line.split_once('=')?;
    (key.trim() == field).then(|| value.trim())
}

fn parse_toml_string(encoded: &str) -> Result<String, String> {
    let body = encoded
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .ok_or_else(|| format!("expected a basic TOML string, found {encoded}"))?;
    let mut decoded = String::new();
    let mut chars = body.chars();
    while let Some(character) = chars.next() {
        if character != '\\' {
            decoded.push(character);
            continue;
        }
        let escaped = chars
            .next()
            .ok_or_else(|| "unterminated TOML escape".to_owned())?;
        decoded.push(match escaped {
            '"' => '"',
            '\\' => '\\',
            'n' => '\n',
            'r' => '\r',
            't' => '\t',
            other => return Err(format!("unsupported TOML escape: \\{other}")),
        });
    }
    Ok(decoded)
}

fn parse_dependency_reference(value: &str) -> Result<DependencyReference, String> {
    let (identity, source) = if let Some(index) = value.rfind(" (") {
        let source = value
            .get(index + 2..value.len().saturating_sub(1))
            .filter(|_| value.ends_with(')'))
            .ok_or_else(|| format!("malformed Cargo.lock dependency reference: {value}"))?;
        (&value[..index], Some(source.to_owned()))
    } else {
        (value, None)
    };
    let mut parts = identity.split_whitespace();
    let name = parts
        .next()
        .ok_or_else(|| "empty Cargo.lock dependency reference".to_owned())?
        .to_owned();
    let version = parts.next().map(str::to_owned);
    if parts.next().is_some() {
        return Err(format!(
            "malformed Cargo.lock dependency reference: {value}"
        ));
    }
    Ok(DependencyReference {
        name,
        version,
        source,
    })
}

fn resolve_reference(
    reference: &DependencyReference,
    packages: &BTreeMap<PackageId, &LockPackage>,
) -> Result<PackageId, String> {
    let candidates = packages
        .keys()
        .filter(|id| {
            id.name == reference.name
                && reference
                    .version
                    .as_ref()
                    .is_none_or(|version| id.version == *version)
                && reference
                    .source
                    .as_ref()
                    .is_none_or(|source| id.source.as_ref() == Some(source))
        })
        .cloned()
        .collect::<Vec<_>>();
    let [candidate] = candidates.as_slice() else {
        return Err(format!(
            "dependency reference {} resolves to {} packages",
            reference.name,
            candidates.len()
        ));
    };
    Ok(candidate.clone())
}

fn normalize_newlines(value: &str) -> String {
    value.replace("\r\n", "\n").replace('\r', "\n")
}

#[cfg(test)]
mod tests {
    use super::{
        ACTIVE_WORKSPACE_LOCK, KERNEL_MANIFEST, REVIEWED_GRAPH,
        canonical_production_dependency_graph, normalize_newlines,
        reviewed_production_dependency_graph,
    };

    const TEST_MANIFEST: &str = r#"
[package]
name = "pen-kernel"
version = "0.1.0"

[dependencies]
direct = "=1.0.0"

[dev-dependencies]
dev-only = "=1.0.0"
"#;

    const TEST_LOCK: &str = r#"
version = 4

[[package]]
name = "pen-kernel"
version = "0.1.0"
dependencies = [
 "dev-only",
 "direct",
]

[[package]]
name = "dev-only"
version = "1.0.0"
source = "registry+test"
checksum = "dev"

[[package]]
name = "direct"
version = "1.0.0"
source = "registry+test"
checksum = "direct"
dependencies = [
 "transitive",
]

[[package]]
name = "transitive"
version = "2.0.0"
source = "registry+test"
checksum = "transitive-a"
"#;

    #[test]
    fn reviewed_graph_matches_the_active_lock_resolution() {
        let active = canonical_production_dependency_graph(ACTIVE_WORKSPACE_LOCK, KERNEL_MANIFEST)
            .expect("active dependency graph");
        assert_eq!(active, normalize_newlines(REVIEWED_GRAPH).as_bytes());
        assert_eq!(reviewed_production_dependency_graph(), active);
    }

    #[test]
    fn graph_binds_transitive_checksums() {
        let first = canonical_production_dependency_graph(TEST_LOCK, TEST_MANIFEST)
            .expect("first dependency graph");
        let changed = TEST_LOCK.replace("transitive-a", "transitive-b");
        let second = canonical_production_dependency_graph(&changed, TEST_MANIFEST)
            .expect("changed dependency graph");
        assert_ne!(first, second);
    }

    #[test]
    fn graph_excludes_dev_only_dependencies() {
        let graph = canonical_production_dependency_graph(TEST_LOCK, TEST_MANIFEST)
            .expect("dependency graph");
        let graph = String::from_utf8(graph).expect("canonical graph is UTF-8");
        assert!(!graph.contains("dev-only"));
        assert!(graph.contains("transitive"));
    }

    #[test]
    fn table_form_dependencies_fail_closed_instead_of_being_omitted() {
        let manifest = r#"
[package]
name = "pen-kernel"
version = "0.1.0"

[dependencies.direct]
version = "=1.0.0"
"#;
        assert!(
            canonical_production_dependency_graph(TEST_LOCK, manifest)
                .expect_err("table-form dependency must be explicit")
                .contains("table-form")
        );
    }
}

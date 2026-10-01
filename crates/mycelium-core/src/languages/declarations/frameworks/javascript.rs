//! Only existing declarations in supported inline callback positions acquire evidence.
use super::*;

pub(super) fn detect(root: Node<'_>, source: &[u8], file: &str, detection: &mut Detection) {
    let mut imports: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for node in children(root)
        .into_iter()
        .filter(|n| n.kind() == "import_statement")
    {
        let Some(module) = node
            .child_by_field_name("source")
            .and_then(|n| literal(n, source))
        else {
            continue;
        };
        // Type-only imports are not runtime bindings.
        if keyword(node, "type") {
            continue;
        }
        visit(node, &mut |specifier| {
            if specifier.kind() == "identifier"
                && specifier
                    .parent()
                    .is_some_and(|p| p.kind() == "import_clause")
            {
                let export = if module == "node:test" {
                    "test"
                } else {
                    "default"
                };
                imports
                    .entry(text(specifier, source))
                    .or_default()
                    .push(format!("{module}/{export}"));
            }
            if keyword(specifier, "type") {
                return;
            }
            if specifier.kind() != "import_specifier" {
                return;
            }
            let Some(name) = field(specifier, "name", source) else {
                return;
            };
            let local = field(specifier, "alias", source).unwrap_or_else(|| name.clone());
            imports
                .entry(local)
                .or_default()
                .push(format!("{module}/{name}"));
        });
    }
    let mut commonjs = BTreeSet::new();
    if file.ends_with(".js") || file.ends_with(".jsx") {
        for declaration in children(root)
            .into_iter()
            .filter(|n| n.kind() == "lexical_declaration" && text(*n, source).starts_with("const "))
        {
            for variable in children(declaration) {
                let Some(value) = variable
                    .child_by_field_name("value")
                    .filter(|n| n.kind() == "call_expression")
                else {
                    continue;
                };
                if field(value, "function", source).as_deref() != Some("require") {
                    continue;
                }
                let Some(args) = value.child_by_field_name("arguments") else {
                    continue;
                };
                let args = children(args);
                if args.len() != 1 {
                    continue;
                }
                let Some(module) = literal(args[0], source) else {
                    continue;
                };
                let Some(pattern) = variable
                    .child_by_field_name("name")
                    .filter(|n| n.kind() == "object_pattern")
                else {
                    continue;
                };
                for member in children(pattern) {
                    let binding = if member.kind() == "shorthand_property_identifier_pattern" {
                        let name = text(member, source);
                        Some((name.clone(), name))
                    } else if member.kind() == "pair_pattern" {
                        member
                            .child_by_field_name("key")
                            .filter(|n| n.kind() == "property_identifier")
                            .zip(
                                member
                                    .child_by_field_name("value")
                                    .filter(|n| n.kind() == "identifier"),
                            )
                            .map(|(key, value)| (text(key, source), text(value, source)))
                    } else {
                        None
                    };
                    if let Some((name, local)) = binding {
                        imports
                            .entry(local)
                            .or_default()
                            .push(format!("{module}/{name}"));
                        commonjs.insert(variable.id());
                    }
                }
            }
        }
    }
    let mut shadows = BTreeSet::new();
    visit(root, &mut |node| {
        if matches!(
            node.kind(),
            "function_declaration"
                | "class_declaration"
                | "function_expression"
                | "generator_function_declaration"
                | "enum_declaration"
                | "internal_module"
        ) {
            if let Some(name) = field(node, "name", source) {
                shadows.insert(name);
            }
        }
        let target = match node.kind() {
            "variable_declarator" if !commonjs.contains(&node.id()) => {
                node.child_by_field_name("name")
            }
            "assignment_expression" | "augmented_assignment_expression" | "for_in_statement" => {
                node.child_by_field_name("left")
            }
            "formal_parameters" => Some(node),
            "arrow_function" | "catch_clause" => node.child_by_field_name("parameter"),
            "update_expression" => node.child_by_field_name("argument"),
            "namespace_import" => Some(node),
            _ => None,
        };
        if let Some(target) = target {
            bound_names(target, source, &mut shadows);
        }
    });
    if shadows.contains("require") || imports.contains_key("require") {
        // A locally defined require invalidates all CommonJS candidates, never ES imports.
        for declaration in children(root)
            .into_iter()
            .filter(|n| n.kind() == "lexical_declaration")
        {
            for variable in children(declaration)
                .into_iter()
                .filter(|n| commonjs.contains(&n.id()))
            {
                if let Some(pattern) = variable.child_by_field_name("name") {
                    bound_names(pattern, source, &mut shadows);
                }
            }
        }
    }
    imports.retain(|name, values| values.len() == 1 && !shadows.contains(name));
    visit(root, &mut |node| {
        if node.kind() != "call_expression" {
            return;
        }
        let Some(callee) = node
            .child_by_field_name("function")
            .filter(|n| n.kind() == "identifier")
        else {
            return;
        };
        let Some(bindings) = imports.get(&text(callee, source)).filter(|v| v.len() == 1) else {
            return;
        };
        let Some((module, name)) = bindings[0].rsplit_once('/') else {
            return;
        };
        let supported = matches!(name, "test" | "it" | "describe")
            || (name == "suite" && module != "@jest/globals");
        if !supported {
            return;
        }
        let Some(arguments) = node.child_by_field_name("arguments") else {
            return;
        };
        let args = children(arguments);
        let callback = match module {
            "node:test" if (1..=3).contains(&args.len()) => args.last(),
            "vitest" if (2..=3).contains(&args.len()) => args
                .get(1)
                .filter(|n| inline(**n))
                .or_else(|| args.get(2).filter(|_| args[1].kind() == "object")),
            "@jest/globals" if (2..=3).contains(&args.len()) => args.get(1),
            _ => None,
        };
        if let Some(callback) = callback.filter(|n| inline(**n)) {
            let rule = match module {
                "node:test" => "javascript.node-callback",
                "vitest" => "javascript.vitest-callback",
                _ => "javascript.jest-callback",
            };
            detection.mark(*callback, node, file, rule);
        }
    });
}

fn inline(node: Node<'_>) -> bool {
    matches!(node.kind(), "arrow_function" | "function_expression")
}

fn literal(node: Node<'_>, source: &[u8]) -> Option<String> {
    if node.kind() != "string" {
        return None;
    }
    let value = node.utf8_text(source).ok()?;
    if value.contains('\\') {
        return None;
    }
    value
        .get(1..value.len().checked_sub(1)?)
        .map(str::to_string)
}

// A file-wide binding check favours retaining code when lexical binding is uncertain.
fn bound_names(node: Node<'_>, source: &[u8], names: &mut BTreeSet<String>) {
    if matches!(
        node.kind(),
        "identifier" | "shorthand_property_identifier_pattern"
    ) {
        names.insert(text(node, source));
        return;
    }
    if node.kind() == "pair_pattern" {
        if let Some(value) = node.child_by_field_name("value") {
            bound_names(value, source, names);
        }
        return;
    }
    if node.kind() == "member_expression" {
        if let Some(object) = node.child_by_field_name("object") {
            bound_names(object, source, names);
        }
        return;
    }
    for child in children(node) {
        bound_names(child, source, names);
    }
}

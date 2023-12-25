// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            parse_function_text,
            evaluate_truth_table,
            generate_function
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

struct Operator {
    text: char,
    priority: i8,
}

#[tauri::command]
fn parse_function_text(text: String) -> Result<Vec<char>, String> {
    let mut operator_priorities: IndexMap<char, i8> = IndexMap::new();
    operator_priorities.insert('!', 4);
    operator_priorities.insert('&', 3);
    operator_priorities.insert('|', 2);
    operator_priorities.insert('^', 1);
    operator_priorities.insert('~', 0);

    let mut operator_stack: Vec<Operator> = Vec::new();
    let mut output_queue: Vec<char> = Vec::new();

    for character in text.chars() {
        match character {
            ' ' => (),
            '(' => operator_stack.push(Operator {
                text: '(',
                priority: -1,
            }),
            ')' => {
                while operator_stack.last().is_some_and(|op| op.text != '(') {
                    let output_operator = operator_stack.pop();
                    if output_operator.is_none() {
                        return Err("Invalid expression!".into());
                    }
                    output_queue.push(output_operator.unwrap().text);
                }
                operator_stack.pop();
            }
            other => {
                let priority = operator_priorities.get(&character);

                if priority.is_none() {
                    output_queue.push(other);
                    continue;
                }

                while operator_stack
                    .last()
                    .is_some_and(|op| character != '!' && op.priority >= *priority.unwrap())
                {
                    let output_operator = operator_stack.pop();
                    if output_operator.is_none() {
                        return Err("Invalid expression!".into());
                    }
                    output_queue.push(output_operator.unwrap().text);
                }
                operator_stack.push(Operator {
                    text: character,
                    priority: *priority.unwrap(),
                });
            }
        }
    }

    while let Some(operator) = operator_stack.pop() {
        output_queue.push(operator.text);
    }

    Ok(output_queue)
}

struct ExpressionTreeNode {
    children: Vec<ExpressionTreeNode>,
    value: char,
}

const OPERATORS: [char; 5] = ['!', '&', '|', '^', '~'];

use std::collections::HashMap;

use indexmap::IndexMap;

#[tauri::command]
fn evaluate_truth_table(postfix_expression: String) -> Result<Vec<Vec<(String, bool)>>, String> {
    let mut variables: IndexMap<char, bool> = IndexMap::new();

    let mut nodes: Vec<ExpressionTreeNode> = Vec::new();

    for character in postfix_expression.chars() {
        if OPERATORS.contains(&character) {
            let mut node = ExpressionTreeNode {
                children: Vec::new(),
                value: character,
            };
            let last = match nodes.pop() {
                Some(last) => last,
                None => return Err("Error constructing expression tree!".into()),
            };

            if character != '!' {
                let second_to_last = match nodes.pop() {
                    Some(last) => last,
                    None => return Err("Error constructing expression tree!".into()),
                };
                node.children.push(second_to_last);
            }

            node.children.push(last);

            nodes.push(node);

            continue;
        }

        if character != 'T' && character != 'F' && !variables.contains_key(&character) {
            variables.insert(character, false);
        }

        nodes.push(ExpressionTreeNode {
            children: Vec::new(),
            value: character,
        });
    }

    println!();
    println!("Successfully constructed expression tree.");
    display_node(&nodes[0]);
    println!();

    let mut truth_table: Vec<Vec<(String, bool)>> = Vec::new();

    println!();
    println!("All values evaluated");

    let variable_count = variables.len();
    for values in 0..2u32.pow(variable_count.try_into().unwrap()) {
        let mut partial_values: IndexMap<String, bool> = IndexMap::new();

        for (idx, var) in variables.clone().keys().enumerate() {
            let variable_bool = (values >> (variable_count - idx - 1)) % 2 != 0;
            variables.insert(*var, variable_bool);
            partial_values.insert(var.to_string(), variable_bool);
        }

        if nodes.first().is_none() {
            return Err("Expression tree is empty!".into());
        }

        evaluate_node(nodes.first().unwrap(), &variables, &mut partial_values)?;

        truth_table.push(
            partial_values
                .iter()
                .map(|(s, b)| (s.clone(), b.clone()))
                .collect::<Vec<(String, bool)>>(),
        );

        println!("{:?}", partial_values);
        // let table_row = ;
    }
    println!();

    Ok(truth_table)
}

fn evaluate_node(
    node: &ExpressionTreeNode,
    variables: &IndexMap<char, bool>,
    partial_values: &mut IndexMap<String, bool>,
) -> Result<(bool, String), String> {
    if variables.contains_key(&node.value) {
        return Ok((*variables.get(&node.value).unwrap(), node.value.to_string()));
    }

    if OPERATORS.contains(&node.value) {
        if node.children.first().is_none() {
            return Err("Invalid expression tree!".into());
        }

        let left_child = evaluate_node(node.children.first().unwrap(), &variables, partial_values)?;

        if node.value == '!' {
            let new_expression = format!("!{}", left_child.1.to_string());
            partial_values
                .entry(new_expression.clone())
                .or_insert(!left_child.0);
            return Ok((!left_child.0, new_expression));
        }

        if node.children.get(1).is_none() {
            return Err("Invalid expression tree!".into());
        }
        let right_child = evaluate_node(&node.children[1], &variables, partial_values)?;

        let new_bool = match node.value {
            '&' => left_child.0 && right_child.0,
            '|' => left_child.0 || right_child.0,
            '^' => !left_child.0 || right_child.0,
            '~' => left_child.0 == right_child.0,
            _ => return Err("Invalid expression tree!".into()),
        };

        let new_expression = format!(
            "({}{}{})",
            left_child.1.to_string(),
            node.value,
            right_child.1.to_string()
        );

        partial_values
            .entry(new_expression.clone())
            .or_insert(new_bool);

        return Ok((new_bool, new_expression));
    }

    if node.value == 'T' {
        return Ok((true, "T".into()));
    }

    if node.value == 'F' {
        return Ok((false, "F".into()));
    }

    Err("Invalid expression tree!".into())
}

fn display_node(node: &ExpressionTreeNode) {
    if node.children.is_empty() {
        return;
    }

    print!("{}: ", node.value);
    for n in &node.children {
        print!("{} ", n.value);
    }
    println!();

    for n in &node.children {
        display_node(&n);
    }
}

#[tauri::command]
fn generate_function(row_values: HashMap<u32, bool>, variables: Vec<String>) -> String {
    let variable_count: u32 = variables.len().try_into().unwrap();
    let row_count = 2u32.pow(variable_count);

    let use_true_rows = u32::try_from(
        row_values
            .iter()
            .filter(|value| *value.1)
            .collect::<Vec<(&u32, &bool)>>()
            .len(),
    )
    .unwrap()
        < (row_count / 2);

    fn get_row_truth_value(row_index: u32, variable_count: u32, variable_index: u32) -> bool {
        (row_index >> (variable_count - variable_index - 1)) % 2 != 0
    }

    let mut parts_of_function: Vec<String> = Vec::new();

    for row_index in 0..row_count {
        if (row_values.get(&row_index).is_none() || !row_values.get(&row_index).unwrap())
            ^ use_true_rows
        {
            parts_of_function.push(
                variables
                    .iter()
                    .enumerate()
                    .map(|(idx, variable)| {
                        if get_row_truth_value(row_index, variable_count, idx.try_into().unwrap())
                            ^ use_true_rows
                        {
                            format!("{}", variable)
                        } else {
                            format!("!{}", variable)
                        }
                    })
                    .collect::<Vec<String>>()
                    .join(if use_true_rows { "&" } else { "|" }),
            );
        }
    }

    if parts_of_function.is_empty() {
        if use_true_rows {
            return "F".into();
        } else {
            return "T".into();
        }
    }

    parts_of_function
        .iter()
        .map(|f| format!("({})", f))
        .collect::<Vec<String>>()
        .join(if use_true_rows { "|" } else { "&" })
}

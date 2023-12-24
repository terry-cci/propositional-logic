// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            parse_function_text,
            evaluate_truth_table
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

struct Operator {
    text: char,
    priority: i8,
}

use std::collections::HashMap;

#[tauri::command]
fn parse_function_text(text: String) -> Vec<char> {
    let mut operator_priorities: HashMap<char, i8> = HashMap::new();
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
                    output_queue.push(operator_stack.pop().unwrap().text);
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
                    .is_some_and(|op| op.priority >= *priority.unwrap())
                {
                    output_queue.push(operator_stack.pop().unwrap().text);
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

    output_queue
}

struct ExpressionTreeNode {
    children: Vec<ExpressionTreeNode>,
    value: char,
}

#[tauri::command]
fn evaluate_truth_table(postfix_expression: String) {
    struct Variable {
        name: char,
        value: bool,
    }

    const OPERATORS: [char; 5] = ['!', '&', '|', '^', '~'];
    let mut variables: Vec<Variable> = Vec::new();

    let mut nodes: Vec<ExpressionTreeNode> = Vec::new();

    for character in postfix_expression.chars() {
        if OPERATORS.contains(&character) {
            let mut node = ExpressionTreeNode {
                children: Vec::new(),
                value: character,
            };
            let last = nodes.pop().unwrap();

            if character != '!' {
                let second_to_last = nodes.pop().unwrap();
                node.children.push(second_to_last);
            }

            node.children.push(last);

            nodes.push(node);

            continue;
        }

        if !variables.iter().any(|var| var.name == character) {
            variables.push(Variable {
                name: character,
                value: false,
            });
        }

        nodes.push(ExpressionTreeNode {
            children: Vec::new(),
            value: character,
        });
    }

    println!();
    display_node(&nodes[0]);
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

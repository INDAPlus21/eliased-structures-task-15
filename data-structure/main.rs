use std::io;
use std::io::prelude::*;

fn main() {
    let input = io::stdin();

    let mut line_iter = 0;
    let mut length_of_commands = 0;

    let mut queue_order = vec![];
    let mut stack_order = vec![];
    let mut priority_order = vec![];

    let mut is_stack = true;
    let mut is_queue = true;
    let mut is_priority = true;

    for _line in input.lock().lines().map(|_line| _line.unwrap()) {
        let command: Vec<usize> = _line
            .split(' ')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();

        if command.len() == 1 {
            length_of_commands = command[0];
        } else if command[0] == 1 {
            queue_order.push(command[1]);
            stack_order.push(command[1]);
            priority_order.push(command[1]);
            line_iter += 1;
        } else if command[0] == 2 {
            if !priority_order.contains(&command[1]) && !queue_order.contains(&command[1]) && !stack_order.contains(&command[1]) {
                is_stack = false;
                is_queue = false;
                is_priority = false;
                line_iter = length_of_commands
            } else {
                let max_value = *priority_order.iter().max().unwrap();
                if command[1] != queue_order[0] {
                    is_queue = false;
                }
                if command[1] != stack_order[stack_order.len() - 1] {
                    is_stack = false;
                }
                if command[1] != max_value {
                    is_priority = false;
                }
                queue_order.remove(0);
                stack_order.remove(stack_order.len() - 1);
                let max_index = priority_order.iter().position(|&r| r == max_value).unwrap();
                priority_order.remove(max_index);
            }
            line_iter += 1;
        }

        if line_iter == length_of_commands && line_iter > 0 {
            if is_stack && !is_queue && !is_priority {
                println!("stack");
            } else if is_queue && !is_stack && !is_priority {
                println!("queue");
            } else if is_priority && !is_stack && !is_queue {
                println!("priority queue");
            } else if !is_priority && !is_stack && !is_queue {
                println!("impossible");
            } else {
                println!("not sure");
            }

            line_iter = 0;

            queue_order = vec![];
            stack_order = vec![];
            priority_order = vec![];

            is_stack = true;
            is_queue = true;
            is_priority = true;
        }
    }
}

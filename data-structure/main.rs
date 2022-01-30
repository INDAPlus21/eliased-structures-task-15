use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;

fn main() {
    let input = io::stdin();

    let mut commands = vec![];
    for _line in input.lock().lines().map(|_line| _line.unwrap()) {
        let command: Vec<u16> = _line
            .split(' ')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();
        commands.push(command);
        if commands.len() == (commands[0][0] + 1).into() {
            let mut line_iter = 0;
            let mut length_of_commands = 0;
        
            // The reason could be 1000 (u8 is 2^8-1)
            // let mut queue_order = vec![];
            let mut queue_order: VecDeque<u32> = VecDeque::with_capacity(commands[0][0].into());
            let mut stack_order = vec![];
            let mut priority_order = vec![];
        
            let mut is_stack = true;
            let mut is_queue = true;
            let mut is_priority = true;

            for command_i in commands {
                if command_i.len() == 1 {
                    length_of_commands = command_i[0];
                } else if command_i[0] == 1 {
                    // queue_order.push(command_i[1]);
                    queue_order.push_back(command_i[1].into());
                    stack_order.push(command_i[1]);
                    priority_order.push(command_i[1]);
                    line_iter += 1;
                } else if command_i[0] == 2 {
                    if is_stack || is_queue || is_priority {
                        if !priority_order.contains(&command_i[1])
                            && !queue_order.contains(&command_i[1].into())
                            && !stack_order.contains(&command_i[1])
                        {
                            is_stack = false;
                            is_queue = false;
                            is_priority = false;
                        } else {
                            let max_value = *priority_order.iter().max().unwrap();
                            if u32::from(command_i[1]) != queue_order[0] {
                                is_queue = false;
                            }
                            if command_i[1] != stack_order[stack_order.len() - 1] {
                                is_stack = false;
                            }
                            if command_i[1] != max_value {
                                is_priority = false;Guess the Data Structure!
                            }
                            queue_order.pop_front();
                            stack_order.remove(stack_order.len() - 1);
                            let max_index =
                                priority_order.iter().position(|&r| r == max_value).unwrap();
                            priority_order.remove(max_index);
                        }
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
                }
            }
            commands = vec![];
        }
    }
}

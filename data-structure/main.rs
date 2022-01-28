/*
There is a bag-like data structure, supporting two operations:

1 x: Throw an element x into the bag.

2: Take out an element from the bag.

Given a sequence of operations with return values, you’re going to guess the data structure.
It is a stack (Last-In, First-Out), a queue (First-In, First-Out),
a priority-queue (Always take out larger elements first) or something else that you can hardly imagine!

Input
There are several test cases. Each test case begins with a line containing a single integer n (1≤n≤1000).
Each of the next n lines is either a type-1 command, or an integer 2 followed by an integer x.
This means that executing the type-2 command returned the element x.
The value of x is always a positive integer not larger than 100.
The input is terminated by end-of-file (EOF). The size of input file does not exceed 1MB.

Output
For each test case, output one of the following:

stack
It’s definitely a stack.

queue
It’s definitely a queue.

priority queue
It’s definitely a priority queue.

impossible
It can’t be a stack, a queue or a priority queue.

not sure
It can be more than one of the three data structures mentioned above.

queue
not sure
impossible
stack
priority queue
impossible

6
1 1
1 2
1 3
2 1
2 2
2 3
6
1 1
1 2
1 3
2 3
2 2
2 1
2
1 1
2 2
4
1 2
1 1
2 1
2 2
7
1 2
1 5
1 1
1 3
2 5
1 4
2 4
1
2 1
*/

use std::io;
use std::io::prelude::*;

fn main() {
    let input = io::stdin();

    let mut line_iter = 0;
    let mut length_of_commands = 0;

    let mut structure = vec![];

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

        // eprintln!("command: {:?}", command);
        // you can delete structure 

        if command.len() == 1 {
            // eprintln!("resetting");
            length_of_commands = command[0];
        } else if command[0] == 1 {
            structure.push(command[1]);
            queue_order.push(command[1]);
            stack_order.push(command[1]);
            priority_order.push(command[1]);
            line_iter += 1;
            // eprintln!("{:?}", structure);
        } else if command[0] == 2 {
            if !priority_order.contains(&command[1]) && !queue_order.contains(&command[1]) && !stack_order.contains(&command[1]) {
                is_stack = false;
                is_queue = false;
                is_priority = false;
            } else {
                let take_out = &command[1];
                structure.retain(|e| e != &command[1]); // What if there are multiple of the same integers?
                // eprintln!("i took out {:?}", take_out);
                let max_value = *priority_order.iter().max().unwrap();
                // eprintln!("max value: {:?}", max_value);
                if command[1] == queue_order[0] {
                    // eprintln!("queue not falsified");
                } else {
                    is_queue = false;
                    // eprintln!("queue falsified!!!");
                }
                if command[1] == stack_order[stack_order.len() - 1] {
                    // eprintln!("stack not falsified");
                } else {
                    is_stack = false;
                    // eprintln!("stack falsified!!!");
                }
                if command[1] == max_value {
                    // eprintln!("priority queue not falsified");
                } else {
                    is_priority = false;
                    // eprintln!("priority queue falsified!!!");
                }
                queue_order.remove(0);
                stack_order.remove(stack_order.len() - 1);
                let max_index = priority_order.iter().position(|&r| r == max_value).unwrap();
                priority_order.remove(max_index);
                // eprintln!("queue_order {:?}", &queue_order);
            }
            line_iter += 1;
        }

        // eprintln!("line_iter: {}", line_iter);

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

            structure = vec![];

            queue_order = vec![];
            stack_order = vec![];
            priority_order = vec![];

            is_stack = true;
            is_queue = true;
            is_priority = true;
        }
    }
}

/*I hope you know the beautiful Union-Find structure. In this problem, you’re to implement something similar, but not identical.
The data structure you need to write is also a collection of disjoint sets, supporting 3 operations:

1 p q
Union the sets containing p and q. If p and q are already in the same set, ignore this command.
2 p q
Move p to the set containing q. If p and q are already in the same set, ignore this command
3 p
Return the number of elements and the sum of elements in the set containing p.
Initially, the collection contains n sets: {1},{2},{3},…,{n}.

As an example, consider the sequence of operations in sample input 1 below.

Initially: {1},{2},{3},{4},{5}

Collection after operation 1 1 2: {1,2},{3},{4},{5}

Collection after operation 2 3 4: {1,2},{3,4},{5} (we omit the empty set that is produced when taking out 3 from {3})

Collection after operation 1 3 5: {1,2},{3,4,5}

Collection after operation 2 4 1: {1,2,4},{3,5}

Input
There are several test cases. Each test case begins with a line containing two integers n and m (1≤n,m≤100000),
the number of integers, and the number of commands. Each of the next m lines contains a command.
For every operation, 1≤p,q≤n. The input is terminated by end-of-file (EOF).
There are at most 20 cases, and the size of the input file does not exceed 5 MB.

Output
For each type-3 command, output 2 integers: the number of elements and the sum of elements.

Sample Input 1	Sample Output 1
5 7             3 12
1 1 2           3 7
2 3 4           2 8
1 3 5
3 4
2 4 1
3 4
3 3
*/

// one optimization is to use the "n integers"

// is it related to multiple... 

use std::io;
use std::io::prelude::*;

fn command1_or_2(mut seq: Vec<Vec<usize>>, commands: Vec<usize>) -> Vec<Vec<usize>> {
    let mut index_to_move_to = 100001;

    for i in 0..seq.len() {
        if seq[i].contains(&commands[2]) {
            if index_to_move_to == 100001 {
                eprintln!("i'm changing index {}", i);
                index_to_move_to = i;
                break;
            }
        }
    }
    for i in 0..seq.len() {
        if seq[i].contains(&commands[1]) {
            if commands[0] == 1 {
                eprintln!("i'm changing vector");
                let mut array_to_concat = seq[i].to_vec();
                eprintln!("array_to_concat: {:?}", array_to_concat);
                seq.retain(|e| !e.contains(&commands[1]));
                if index_to_move_to > i { // if the index to move to is greater than the current index (that will merge)
                    index_to_move_to -= 1;
                }
                // And equal to doesn't make sense, because you don't move onto itself 
                // No wait. The empty array isn't removed... Eh. Yes it is. 
                // [1, 2, 3, 4, 5] -> 
                // [1, 2, 4, 5] // now moving to index [3] is wrong 
                // [1, 2, [3, 4], 5] // so it should be moved to index 2
                // And if it's the reverse
                // [1, 2, 3, 4, 5] 
                // [1, 2, 3, 5] // now moving to index 2 is still right
                // [1, 2, [3, 4], 5] // right 
                // no it's always because you merge 
                /*else {
                      index_to_move_to += 1;
                  }*/
                seq[index_to_move_to].append(&mut array_to_concat);
                eprintln!("{:?}", seq);
                break;
            } else if commands[0] == 2 {
                if seq[i].contains(&commands[1]) {
                    eprintln!("i'm changing vector");
                    seq[i].retain(|e| e != &commands[1]);

                    seq[index_to_move_to].push(commands[1]);
                    eprintln!("{:?}", seq);
                    break;
                }
            }
        }
    }

    return seq
}

fn command3(seq: &Vec<Vec<usize>>, commands: Vec<usize>) {
    for i in 0..seq.len() {
        if seq[i].contains(&commands[1]) {
            let sum: usize = seq[i].iter().sum();
            println!("{} {}", seq[i].len(), sum);
            return
        }
    }
}

fn main() {
    let input = io::stdin();

    let mut seq: Vec<Vec<usize>> = vec![]; // = Vec::with_capacity(commands[i] as usize);

    let mut line_iter = 0;

    for _line in input.lock().lines().map(|_line| _line.unwrap()) {
        let commands: Vec<usize> = _line
        .split(' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

        if line_iter == 0 {
            for i in 1..commands[0]+1 {
                seq.push(vec![i]);
            }
        } else if line_iter != 0 {
            eprintln!("{}", _line);
            eprintln!("{:?}", seq);
            eprintln!("{}", line_iter);    
            if commands[0] == 3 {
                command3(&seq, commands)
            } else {
                seq = command1_or_2(seq, commands)
            }
        }
        line_iter += 1;
    }
}
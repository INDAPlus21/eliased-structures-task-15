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

use std::io;
use std::io::prelude::*;

/// Kattis calls main function to run your solution.
fn main() {
    // get standard input stream
    let input = io::stdin();

    // get input lines as iterative
    /*let mut lines = input
    .lock()
    .lines()
    .map(|_line| _line.ok().unwrap());*/
    // and get one line at a time,
    // let next_line = lines.next().unwrap();

    let mut seq = vec![vec![1], vec![2], vec![3], vec![4], vec![5]];
    let commands = [3, 1, 2];
    if commands[0] == 1 {
        // let first_set : vec!
        // let second_set : vec!
        let mut index_to_move_to = 100001;
        // This will not work if they're not ordered
        for i in 0..seq.len() {
            eprintln!("i'm in loop: {}", i);
            if seq[i].contains(&commands[1]) {
                if index_to_move_to == 100001 {
                    eprintln!("i'm changing index");
                    index_to_move_to = i;
                }
            } else if seq[i].contains(&commands[2]) {
                eprintln!("i'm changing vector");
                let mut array_to_concat = seq[i].to_vec();
                eprintln!("array_to_concat: {:?}", array_to_concat);
                seq.retain(|e| !e.contains(&commands[2]));
                /*if index_to_move_to > i {
                    index_to_move_to += 1;
                } else {
                    index_to_move_to += 1;
                }*/
                seq[index_to_move_to].append(&mut array_to_concat);
                eprintln!("{:?}", seq);
                break;
                // seq.remove(seq.iter().position(|x| *x == array_to_concat));
                // let index = seq.iter().position(|x| *x == array_to_concat).unwrap();
                // seq.remove(index);
                // seq.retain(|&x| x != array_to_concat);
            }
        }
        // n.contains(&"-i")
        // seq[0].push(1);
    } else if commands[0] == 2 {
        let mut index_to_move_to = 100001;

        for i in 0..seq.len() {
            eprintln!("i'm in loop: {}", i);
            if seq[i].contains(&commands[1]) {
                if index_to_move_to == 100001 {
                    eprintln!("i'm changing index");
                    index_to_move_to = i;
                }
            } else if seq[i].contains(&commands[2]) {
                eprintln!("i'm changing vector");
                seq[i].retain(|e| e != &commands[2]);
                seq[index_to_move_to].push(commands[2]);
                eprintln!("{:?}", seq);
                break;
            }
        }                println!(seq[i].len())

    } else if commands[0] == 3 {
        let mut index_to_move_to = 100001;

        for i in 0..seq.len() {
            eprintln!("i'm in loop: {}", i);
            if seq[i].contains(&commands[1]) {
                let sum: u8 = seq[i].iter().sum();
                println!("{} {}", seq[i].len(), sum); 
                break; 
            }
        }
    }

    // eprintln!("{:?}", seq);

    // or loop all input lines,
    for _line in input.lock().lines().map(|_line| _line.unwrap()) {
        eprintln!("{}", _line);
        // intial
        eprintln!("{:?}", seq);
        // ...
    }

    // or read single line
    let mut line = String::new();
    input.read_line(&mut line).expect("IO Error");

    /* add code here ... */

    // eprintln!("Kattis skips this comment!");
    //println!("Print to standard output.");
}

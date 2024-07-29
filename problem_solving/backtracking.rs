/**
 * Backtracking
 */


//  fn search(y: i32, n: i32, column: &mut Vec<bool>, diag1: &mut Vec<bool>, diag2: &mut Vec<bool>, solution: &mut i32) {
//     if y == n {
//         *solution += 1;
//         return
//     } 

//     for i in 0..n {
//         if column[i as usize] || diag1[i as usize + y as usize] || diag2[i as usize - y as usize + n as usize -1] {
//             continue;
//         }
//         column[i as usize] = true;
//         diag1[i as usize + y as usize] = true;
//         diag2[i as usize - y as usize + n as usize -1] = true;

//         search(y+1,n, column, diag1, diag2, solution);

//         column[i as usize] = false;
//         diag1[i as usize + y as usize] = false;
//         diag2[i as usize - y as usize + n as usize - 1] = false;
//     }
// }

// fn main() {
//     let n: i32 = 4; // Slove for 8-Queens
//     let mut solution = 0;
//     let mut column = vec![false; n as usize];
//     let mut diag1 = vec![false; n as usize*2 - 1];
//     let mut diag2 = vec![false; n as usize*2 - 1];

//     // search(0, n, &mut column, &mut diag1, &mut diag2, &mut solution);

//     println!("Number of solution: {}, {}, {},", solution, column.len(), diag1.len());
// }

fn search(y: usize, n: usize, solution: &mut usize, column: &mut Vec<bool>, diag1: &mut Vec<bool>, diag2: &mut Vec<bool>) {
    if y == n {
        *solution += 1;
        return;
    }

    for x in 0..n {
        if column[x] || diag1[x + y] || diag2[x as usize - y as usize + n as usize - 1] {
            continue;
        }

        column[x] = true;
        diag2[x as usize - y as usize + n as usize - 1] = true;
        diag1[x + y] = true;
        search(y + 1, n, solution, column, diag1, diag2);
        column[x] = false;
        diag1[x + y] = false;
        diag2[x as usize - y as usize + n as usize - 1] = false;
    }
}

fn main() {
    let n = 8; // Example: Solve for 8-Queens
    let mut solution = 0;
    let mut column = vec![false; n];
    let mut diag1 = vec![false; 2 * n - 1];
    let mut diag2 = vec![false; 2 * n - 1];

    search(0, n, &mut solution, &mut column, &mut diag1, &mut diag2);

    println!("Number of solutions: {}", solution);
}

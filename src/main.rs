// TODO: remove this when you're done with your implementation.
// #![allow(unused_variables, dead_code)]

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut trans_matrix: [[i32; 3]; 3] = [[0; 3]; 3];

    for i in 0..3 {
        for j in 0..3 {
            trans_matrix[j][i] = matrix[i][j]
        }
    }

    trans_matrix
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for row in matrix {
        for item in row {
            print!("{item} ");
        }

        println!("");
    }
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
}

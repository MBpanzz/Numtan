/// Computes the dot product of two equal-length vectors.
pub fn dot(left: &[f64], right: &[f64]) -> Result<f64, String> {
    if left.len() != right.len() {
        return Err("vector lengths must match".to_string());
    }

    Ok(left.iter().zip(right).map(|(a, b)| a * b).sum())
}

/// Computes the Euclidean norm of a vector.
pub fn norm(vector: &[f64]) -> f64 {
    vector.iter().map(|value| value * value).sum::<f64>().sqrt()
}

/// Adds two equal-length vectors element by element.
pub fn add(left: &[f64], right: &[f64]) -> Result<Vec<f64>, String> {
    if left.len() != right.len() {
        return Err("vector lengths must match".to_string());
    }

    Ok(left.iter().zip(right).map(|(a, b)| a + b).collect())
}

/// Subtracts two equal-length vectors element by element.
pub fn sub(left: &[f64], right: &[f64]) -> Result<Vec<f64>, String> {
    if left.len() != right.len() {
        return Err("vector lengths must match".to_string());
    }

    Ok(left.iter().zip(right).map(|(a, b)| a - b).collect())
}

/// Multiplies every vector element by a scalar factor.
pub fn scale(vector: &[f64], factor: f64) -> Vec<f64> {
    vector.iter().map(|value| value * factor).collect()
}

/// Multiplies a dense row-major matrix by a vector.
pub fn mat_vec(matrix: &[Vec<f64>], vector: &[f64]) -> Result<Vec<f64>, String> {
    matrix.iter().map(|row| dot(row, vector)).collect()
}

/// Solves a dense linear system with Gaussian elimination and partial pivoting.
pub fn solve(mut matrix: Vec<Vec<f64>>, mut rhs: Vec<f64>) -> Result<Vec<f64>, String> {
    let size = rhs.len();
    if matrix.len() != size || matrix.iter().any(|row| row.len() != size) {
        return Err("matrix must be square and match rhs length".to_string());
    }

    for pivot in 0..size {
        let mut pivot_row = pivot;
        for row in pivot + 1..size {
            if matrix[row][pivot].abs() > matrix[pivot_row][pivot].abs() {
                pivot_row = row;
            }
        }

        if matrix[pivot_row][pivot].abs() < 1e-14 {
            return Err("matrix is singular".to_string());
        }

        matrix.swap(pivot, pivot_row);
        rhs.swap(pivot, pivot_row);

        for row in pivot + 1..size {
            let factor = matrix[row][pivot] / matrix[pivot][pivot];
            for column in pivot..size {
                matrix[row][column] -= factor * matrix[pivot][column];
            }
            rhs[row] -= factor * rhs[pivot];
        }
    }

    let mut solution = vec![0.0; size];
    for row in (0..size).rev() {
        let mut sum = rhs[row];
        for column in row + 1..size {
            sum -= matrix[row][column] * solution[column];
        }
        solution[row] = sum / matrix[row][row];
    }

    Ok(solution)
}

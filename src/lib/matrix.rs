
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

use rand::{thread_rng, Rng};

#[derive(Clone)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
    
}
impl Matrix {
    pub fn zeros(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows,
            cols,
            data: vec![vec![0.0; cols]; rows]
        }
    }

    pub fn random(rows: usize, cols: usize) -> Matrix {
        let mut rng = thread_rng();
        let mut res = Matrix::zeros(rows, cols);

        for i in 0..rows {
            for j in 0..cols {
                res.data[i][j] = rng.gen::<f64>() * 2.0 - 1.0;
            }
        }
        res
    }  

    pub fn dot_multiply(&mut self, other: &Matrix) -> Matrix {
        let mut new_matrix = Matrix::zeros(self.rows, self.cols); 
        for i in 0..self.rows {
            for j in 0..self.cols {
                new_matrix.data[i][j] = self.data[j][i] * other.data[j][i];
            }
        }
        new_matrix

    }

    pub fn from(data: Vec<Vec<f64>>) -> Matrix {
        Matrix {
            rows: data.len(),
            cols: data[0].len(),
            data: data,
        }
    }

    pub fn transpose(&mut self) -> Matrix {
        let mut new_matrix = Matrix::zeros(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                new_matrix.data[j][i] = self.data[i][j];
            }
        }
        new_matrix
    }

}
  
impl Add for Matrix {
    type Output = Matrix;

    fn add(self, other: Matrix) -> Matrix {
        let mut new_matrix = Matrix::zeros(self.rows, self.cols); 
        for i in 0..self.rows {
            for j in 0..self.cols {
                new_matrix.data[i][j] = self.data[j][i] + other.data[j][i];
            }
        }
        new_matrix
    }
}

impl Sub for Matrix {
    type Output = Matrix;

    fn sub(self, other: Matrix) -> Matrix {
        let mut new_matrix = Matrix::zeros(self.rows, self.cols); 
        for i in 0..self.rows {
            for j in 0..self.cols {
                new_matrix.data[i][j] = self.data[j][i] - other.data[j][i];
            }
        }
        new_matrix
    }
}


impl Mul for Matrix {
    type Output = Matrix;
    fn mul(self, other: Self) -> Self::Output {
        if self.cols != other.rows {
            panic!("Wrong number of dimensions");
        }


        let mut new_matrix = Matrix::zeros(self.rows, self.cols); 
        for i in 0..self.rows {
            for j in 0..self.cols {
                let mut sum: f64 = 0.0;
                for k in 0..other.cols {
                    sum += self.data[i][k] * other.data[k][j];
                }
                new_matrix.data[i][j] = sum;
            }
        }
        new_matrix
    }

}


impl std::string::ToString for Matrix {
    fn to_string(&self) -> String {
        let mut s = String::from("");
        for i in 0..self.cols {
            s.push_str("|");
            for j in 0..self.rows {
                let m = format!("{:.2} ", self.data[i][j]);
                s.push_str(&m);
            }
            s.push_str("|\n");
        }
        format!("{}", s)
    }
}
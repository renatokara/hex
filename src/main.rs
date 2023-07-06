use lib::matrix::Matrix;

pub mod lib;


fn main() {
    
    let mut m1 = Matrix::zeros(4,4);

    let mut m2 = Matrix::zeros(4, 4);

    let mut m3 = Matrix::zeros(4, 4);

    let mut m4 = Matrix::zeros(4, 4);

    m1.data[0][0] = 1.0;
    m1.data[0][1] = 2.0;
    m1.data[0][2] = 3.0;
    m1.data[0][3] = 4.0;

    m1.data[1][0] = 1.0;
    m1.data[1][1] = 2.0;
    m1.data[1][2] = 3.0;
    m1.data[1][3] = 4.0;

    m1.data[2][0] = 1.0;
    m1.data[2][1] = 2.0;
    m1.data[2][2] = 3.0;
    m1.data[2][3] = 4.0;

    m1.data[3][0] = 1.0;
    m1.data[3][1] = 2.0;
    m1.data[3][2] = 3.0;
    m1.data[3][3] = 4.0;


    m2.data[0][0] = 1.0;
    m2.data[0][1] = 2.0;
    m2.data[0][2] = 3.0;
    m2.data[0][3] = 4.0;

    m2.data[1][0] = 1.0;
    m2.data[1][1] = 2.0;
    m2.data[1][2] = 3.0;
    m2.data[1][3] = 4.0;

    m2.data[2][0] = 1.0;
    m2.data[2][1] = 2.0;
    m2.data[2][2] = 3.0;
    m2.data[2][3] = 4.0;

    m2.data[3][0] = 1.0;
    m2.data[3][1] = 2.0;
    m2.data[3][2] = 3.0;
    m2.data[3][3] = 4.0;
    
    m3.data[0][0] = 1.0;
    m3.data[0][1] = 2.0;
    m3.data[0][2] = 3.0;
    m3.data[0][3] = 4.0;
    m3.data[1][0] = 1.0;
    m3.data[1][1] = 2.0;
    m3.data[1][2] = 3.0;
    m3.data[1][3] = 4.0;
    m3.data[2][0] = 1.0;
    m3.data[2][1] = 2.0;
    m3.data[2][2] = 3.0;
    m3.data[2][3] = 4.0;
    m3.data[3][0] = 1.0;
    m3.data[3][1] = 2.0;
    m3.data[3][2] = 3.0;
    m3.data[3][3] = 4.0;
    
    m4.data[0][0] = 1.0;
    m4.data[0][1] = 2.0;
    m4.data[0][2] = 3.0;
    m4.data[0][3] = 4.0;
    m4.data[1][0] = 1.0;
    m4.data[1][1] = 2.0;
    m4.data[1][2] = 3.0;
    m4.data[1][3] = 4.0;
    m4.data[2][0] = 1.0;
    m4.data[2][1] = 2.0;
    m4.data[2][2] = 3.0;
    m4.data[2][3] = 4.0;
    m4.data[3][0] = 1.0;
    m4.data[3][1] = 2.0;
    m4.data[3][2] = 3.0;
    m4.data[3][3] = 4.0;



    let mm = m1 * m2;
    let ma = m3 + m4;
    println!("{}", mm.to_string());
    println!("{}", ma.to_string());

}

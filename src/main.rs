use lib::{matrix::Matrix, network::Network, activations::SIGMOID};

pub mod lib;


fn main() {
    let inputs = vec![ 
        vec![0.0 , 0.0 ],
        vec![0.0 , 1.0 ],
        vec![1. , 0.0 ],
        vec![0.1 , 1.0 ],
    ];

    let targets = vec![
        vec![0.0],
        vec![1.0],
        vec![1.0],
        vec![0.0]
    ];
    let mut network = Network::new(vec![2,3,1], 0.5, SIGMOID);
    network.train(inputs, targets, 10000);
    println!("0 and 0: {:?}", network.feed_forward(vec![0.0,0.0]));
    println!("1 and 0: {:?}", network.feed_forward(vec![1.0,0.0]));
    println!("0 and 1: {:?}", network.feed_forward(vec![0.0,1.0]));
    println!("1 and 1: {:?}", network.feed_forward(vec![1.0,1.0]));
}

fn test() {
    
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

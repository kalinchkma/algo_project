



fn main() {
    let inputs: [f64; 4] = [1.0, 2.0, 3.0, 2.5];

    // weights
    let weight1: [f64; 4] = [0.2, 0.8, -0.5, 1.0];
    let weight2: [f64; 4] = [0.5, -0.91, 0.26, -0.5];
    let weight3: [f64; 4] = [-0.26, -0.27, 0.17, 0.87];

    // biases
    let bias1: f64 = 2.0;
    let bias2: f64 = 3.0;
    let bias3: f64 = 0.5;

    let layer = [
        // layer 1:
        inputs[0]*weight1[0] +
        inputs[1]*weight1[1] +
        inputs[2]*weight1[2] +
        inputs[3]*weight1[3] + bias1,

        // layer 2:
        inputs[0]*weight2[0] +
        inputs[1]*weight2[1] +
        inputs[2]*weight2[2] +
        inputs[3]*weight2[3] + bias2,

        // layer 3
        inputs[0]*weight3[0] +
        inputs[1]*weight3[1] +
        inputs[2]*weight3[2] +
        inputs[3]*weight3[3] + bias3,
    ];

    println!("{:?}", layer);

    // using loops
    let weights = [[0.2, 0.8, -0.5, 1.0], [0.5, -0.91, 0.26, -0.5], [-0.26, -0.27, 0.17, 0.87]];
    let biases: [f64; 3] = [2.0, 3.0, 0.5];
    let mut layer: Vec<f64> = Vec::new();

    for i in 0..weights.len() {
        let mut layer_output: f64 = 0.0;
        for j in 0..inputs.len() {
            layer_output += inputs[j]*weights[i][j]
        }
        layer_output += biases[i];
        layer.push(layer_output);
    }
    println!("{:?}", layer);
    let lolol = [[[3, 4, 2],
                  [4, 5,1]],

                  [[4, 1, 7],
                  [5, 2, 1]],
                  
                  [[5, 1, 7],
                  [5, 1, 8]]];
    println!("{}", lolol.len());
}

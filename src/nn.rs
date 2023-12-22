use crate::grad;

//implementing a neural network, to give me a better idea of exactly what I need

//1d input rn, could collapse at some point
struct Layer<const INPUT_NODES: usize, const OUTPUT_NODES: usize> {
    activation: fn(f64) -> (f64, f64), //includes gradient
    weights: [[f64; INPUT_NODES]; OUTPUT_NODES],
}

//no simd/ndarray initially
//we actually may need to do that unfortunately
impl<const INPUT_NODES: usize, const OUTPUT_NODES: usize> Layer<INPUT_NODES, OUTPUT_NODES> {
    fn forward(self, input: [f64; INPUT_NODES]) -> [f64; OUTPUT_NODES] {
        let mut output: [f64; OUTPUT_NODES] = [0.0; OUTPUT_NODES];
        for output_index in 0..output.len() {
            let mut sum = 0.0;
            for input_index in 0..input.len() {
                sum += self.weights[output_index][input_index] * input[input_index];
            }
            sum /= input.len() as f64;
            output[output_index] = (self.activation)(sum).0;
        }
        output
    }
    //back prop formula
    //compute loss with respective to every weight using chain rule-- this is the blackest magic
    //part
    //I don't wanna compromise with runtime garbage, maybe hand specifying the cross 
}

//weights for each layer will be, input_nodes*output_nodes
// struct Network<const  >{
//     // layers: ,
// }

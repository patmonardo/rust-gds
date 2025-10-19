//! Debug script to understand the backward pass issue

use rust_gds::ml::core::computation_context::ComputationContext;
use rust_gds::ml::core::functions::{Constant, ElementSum};
use rust_gds::ml::core::tensor::{Matrix, Scalar, Vector};
use rust_gds::ml::core::variable::Variable;

fn main() {
    println!("=== Debug Backward Pass ===");
    
    // Create a simple computation graph: ElementSum(Constant(matrix))
    let matrix_data = vec![1.0, 2.0, 3.0, 4.0];
    let constant = Constant::matrix(matrix_data, 2, 2);
    let element_sum = ElementSum::new(Box::new(constant));
    
    let ctx = ComputationContext::new();
    
    // Forward pass
    println!("Running forward pass...");
    let result = element_sum.apply(&ctx);
    println!("Forward pass result: {:?}", result.data());
    
    // Backward pass
    println!("Running backward pass...");
    ctx.backward(&element_sum);
    
    // Check gradients
    println!("Checking gradients...");
    if let Some(gradient) = ctx.gradient(&element_sum) {
        println!("ElementSum gradient: {:?}", gradient.data());
    } else {
        println!("ElementSum gradient: NOT FOUND");
    }
    
    if let Some(parent_gradient) = ctx.gradient(element_sum.parents()[0].as_ref()) {
        println!("Parent gradient: {:?}", parent_gradient.data());
    } else {
        println!("Parent gradient: NOT FOUND");
    }
}

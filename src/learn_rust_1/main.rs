mod learn_01_data_types_examples;
mod learn_02_control_flow;
mod learn_03_function_closures;
mod learn_04_data_structures;
mod learn_05_ownership_examples;
mod learn_06_struct_examples;
mod learn_07_enum_examples;
mod learn_08_basic_option_examples;
mod learn_09_result_examples;
mod learn_10_option_result_combinations_examples;
mod learn_11_traits_examples;
mod learn_12_concurrency_examples;

#[tokio::main]
 async fn main() {
    learn_01_data_types_examples::demo();
    learn_02_control_flow::demo();
    learn_03_function_closures::demo();
    learn_04_data_structures::demo();
    learn_05_ownership_examples::demo();
    learn_06_struct_examples::demo();
    learn_07_enum_examples::demo();
    learn_08_basic_option_examples::demo();
    learn_09_result_examples::demo();
    learn_10_option_result_combinations_examples::demo();
    learn_11_traits_examples::demo();
    learn_12_concurrency_examples::demo().await;
    println!("\n\n\n")
}

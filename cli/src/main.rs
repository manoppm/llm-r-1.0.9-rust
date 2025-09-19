use std::env;
use llm_core::LlmEngine;

fn print_usage() {
    eprintln!("Usage: llm_cli <path_model> <input_text>");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        print_usage();
        std::process::exit(1);
    }
    let model_path = &args[1];
    let input_text = &args[2];

    let mut engine = LlmEngine::new();
    engine.load_model(model_path).expect("load_model failed");
    let output = engine.run_pipeline(input_text).expect("run_pipeline failed");
    println!("{}", output);
}

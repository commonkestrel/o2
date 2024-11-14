use std::path::Path;

use inkwell::{attributes::AttributeLoc, context::Context, targets::{FileType, InitializationConfig, Target, TargetMachine, TargetMachineOptions, TargetTriple}};

const DEFAULT_TARGET: &str = env!("TARGET");

fn main() {
    println!("{DEFAULT_TARGET}");

    let triple = TargetTriple::create(DEFAULT_TARGET);
    Target::initialize_all(&InitializationConfig::default());
    let target = Target::from_triple(&triple).unwrap();

    let machine = target.create_target_machine_from_options(
        &triple, 
        TargetMachineOptions::default()
    ).unwrap();

    let context = Context::create();
    let module = context.create_module("main");
    let builder = context.create_builder();

    let ty = context.i32_type();
    let blank_sig = ty.fn_type(&[], false);
    let main = module.add_function("main", blank_sig, None);
    let basic_block = context.append_basic_block(main, "entry");

    let return_ty = context.i32_type();
    let return_value = return_ty.const_zero();

    builder.position_at_end(basic_block);
    builder.build_return(Some(&return_value));

    machine.write_to_file(&module, FileType::Object, Path::new("./o2.o"));
}

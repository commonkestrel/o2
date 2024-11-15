use std::path::Path;

use clap::Parser;
use inkwell::{
    attributes::AttributeLoc,
    context::Context,
    targets::{
        FileType, InitializationConfig, Target, TargetMachine, TargetMachineOptions, TargetTriple,
    },
    GlobalVisibility,
};

mod diagnostic;
mod span;
mod syntax {
    mod ascii;
    mod lex;
}
mod symbol_table;

const DEFAULT_TARGET: &str = env!("TARGET");

#[derive(Debug, Parser)]
struct Args {
    #[clap(short, long, default_value = DEFAULT_TARGET)]
    target: String,
}

fn main() {
    let args = Args::parse();

    let triple = TargetTriple::create(&args.target);
    Target::initialize_all(&InitializationConfig::default());
    let target =
        Target::from_triple(&triple).expect(&format!("invalid target triple: {}", args.target));

    let machine = target
        .create_target_machine_from_options(&triple, TargetMachineOptions::default())
        .unwrap();

    let context = Context::create();
    let module = context.create_module("main");
    let builder = context.create_builder();

    let void_ty = context.void_type();
    let blank_sig = void_ty.fn_type(&[], false);
    let main = module.add_function("_start", blank_sig, None);
    main.as_global_value()
        .set_visibility(GlobalVisibility::Default);
    let basic_block = context.append_basic_block(main, "entry");

    let exit_fn = context.i64_type().fn_type(
        &[context.i64_type().into(), context.i64_type().into()],
        false,
    );
    let asm = context.create_inline_asm(
        exit_fn,
        "syscall".to_string(),
        "=r,{rax},{rdi}".to_string(),
        true,
        false,
        None,
        false,
    );
    let params = &[
        context.i64_type().const_int(60, false).into(),
        context.i64_type().const_zero().into(),
    ];

    builder.position_at_end(basic_block);
    builder
        .build_indirect_call(exit_fn, asm, params, "exit")
        .unwrap();

    builder.build_return(None);

    machine
        .write_to_file(&module, FileType::Object, &Path::new("o2_test.o"))
        .unwrap();
}

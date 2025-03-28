use std::path::PathBuf;

use build_helper::exit;

use crate::BootstrapCommand;
use crate::builder::Builder;
use crate::core::build_steps::llvm::Llvm;
use crate::core::builder::Cargo;
use crate::core::config::TargetSelection;

pub(crate) fn instrument_coverage(builder: &Builder<'_>, cargo: &mut Cargo) {
    if !builder.config.profiler {
        eprintln!();
        eprintln!("Error: the profiler needs to be enabled to gather coverage.");
        eprintln!("Please set `build.profiler` to `true` in your bootstrap configuration.");
        exit!(1);
    }

    cargo.rustflag("-Cinstrument-coverage");
}

pub(crate) struct GatherCoverage<'a> {
    builder: &'a Builder<'a>,
    llvm_bin_dir: PathBuf,
    profraw_dir: PathBuf,
    profdata_file: PathBuf,
}

impl<'a> GatherCoverage<'a> {
    pub(crate) fn new(
        builder: &'a Builder<'a>,
        cargo: &mut Cargo,
        target: TargetSelection,
        name: &str,
    ) -> Self {
        builder.ensure(Llvm { target });
        let llvm_bin_dir = builder.llvm_out(target).join("bin");

        let profraw_dir = builder.tempdir().join(format!("ferrocene-profraw-{name}"));
        let profdata_file = builder
            .out
            .join(target.triple)
            .join("ferrocene")
            .join("coverage")
            .join(format!("{name}.profdata"));

        if profraw_dir.exists() {
            builder.remove_dir(&profraw_dir);
        }
        if profdata_file.exists() {
            builder.remove(&profdata_file);
        }

        builder.create_dir(&profraw_dir);
        builder.create_dir(profdata_file.parent().unwrap());

        let profraw_file_template = profraw_dir.join("%m_%p.profraw");
        cargo.env("LLVM_PROFILE_FILE", profraw_file_template);

        instrument_coverage(builder, cargo);

        Self { builder, llvm_bin_dir, profraw_dir, profdata_file }
    }

    pub(crate) fn post_process(self) {
        if self.builder.config.dry_run() {
            return;
        }

        let mut cmd = BootstrapCommand::new(self.llvm_bin_dir.join("llvm-profdata"));
        cmd.arg("merge").arg("--sparse").arg("-o").arg(self.profdata_file).arg(self.profraw_dir);
        cmd.fail_fast().run(self.builder);
    }
}

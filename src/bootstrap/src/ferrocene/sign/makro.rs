macro_rules! documents {
    ($($name:ident),*$(,)?) => {
        $(
            #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub(crate) struct $name {
                target: TargetSelection,
            }

            impl Step for $name {
                type Output = ();
                const DEFAULT: bool = false;
                const ONLY_HOSTS: bool = true;

                fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
                    crate::ferrocene::doc::$name::should_run(run)
                }

                fn make_run(run: RunConfig<'_>) {
                    run.builder.ensure(Self { target: run.target });
                }

                fn run(self, builder: &Builder<'_>) {
                    builder.ensure(SignDocument {
                        target: self.target,
                        document: crate::ferrocene::doc::$name {
                            mode: SphinxMode::Html,
                            target: self.target,
                            // Ensure there are no leftover artifacts from a previous incremental
                            // build when generating the signature.
                            fresh_build: true,
                        },
                    });
                }
            }
        )*

        pub(super) fn for_each_signable_document(
            builder: &Builder<'_>,
            target: TargetSelection,
            condition: impl Fn(&Path) -> bool,
            f: impl Fn(&Path, &Path),
        ) {
            $({
                let source_dir = builder.src.join(crate::ferrocene::doc::$name::SOURCE);
                if condition(&source_dir) {
                    let output_dir = builder.ensure(crate::ferrocene::doc::$name {
                        mode: SphinxMode::Html,
                        target,
                        fresh_build: false,
                    });
                    f(&source_dir, &output_dir);
                }
            })*
        }
    };
}

pub(super) use documents;

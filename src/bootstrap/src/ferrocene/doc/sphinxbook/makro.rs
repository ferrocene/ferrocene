/// Generate a [Step](crate::builder::Step) and [WithSource](super::WithSource)
/// implementation for every document and a struct
/// [AllSphinxDocuments](super::AllSphinxDocuments) which builds all documents
/// for the path `ferrocene/doc/`.
macro_rules! sphinx_books {
    ($({
        ty: $ty:ident,
        name: $name:expr,
        src: $src:expr,
        dest: $dest:expr,
        $(inject_all_other_document_ids: $inject_all_other_document_ids:expr,)?
        $(require_test_outcomes: $require_test_outcomes:expr,)?
        $(require_relnotes: $require_relnotes:expr,)?
    },)*) => {
        $(
            #[derive(Debug, PartialEq, Eq, Hash, Clone)]
            pub(crate) struct $ty {
                pub(crate) mode: SphinxMode,
                pub(crate) target: TargetSelection,
                pub(crate) fresh_build: bool,
            }

            impl Step for $ty {
                type Output = PathBuf;
                const DEFAULT: bool = true;

                fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
                    let builder = run.builder;
                    run.path($src).default_condition(builder.config.docs)
                }

                fn make_run(run: RunConfig<'_>) {
                    run.builder.ensure(Self {
                        mode: SphinxMode::Html,
                        target: run.target,
                        fresh_build: false,
                    });
                }

                fn run(self, builder: &Builder<'_>) -> Self::Output {
                    let signature_dir = builder.src
                        .join($src)
                        .join("signature");
                    let signature = if signature_dir.join("config.toml").is_file() {
                        if signature_dir.join("signature.toml").is_file() {
                            SignatureStatus::Present
                        } else {
                            SignatureStatus::Missing
                        }
                    } else {
                        SignatureStatus::NotNeeded
                    };

                    #[allow(unused_mut, unused_assignments)]
                    let mut inject_all_other_document_ids = false;
                    $(inject_all_other_document_ids = $inject_all_other_document_ids;)*

                    #[allow(unused_mut, unused_assignments)]
                    let mut require_test_outcomes = false;
                    $(require_test_outcomes = $require_test_outcomes;)*

                    #[allow(unused_mut, unused_assignments)]
                    let mut require_relnotes = false;
                    $(require_relnotes = $require_relnotes;)*

                    builder.ensure(SphinxBook {
                        mode: self.mode,
                        target: self.target,
                        name: $name.into(),
                        src: $src.into(),
                        dest: $dest.into(),
                        fresh_build: self.fresh_build,
                        signature,
                        inject_all_other_document_ids,
                        require_test_outcomes,
                        require_relnotes,
                        parent: Some(self),
                    })
                }
            }

            impl WithSource for $ty {
                const SOURCE: &'static str = $src;
            }
        )*

        #[derive(Debug, PartialEq, Eq, Hash, Clone)]
        pub(crate) struct AllSphinxDocuments {
            pub(crate) target: TargetSelection,
        }

        impl Step for AllSphinxDocuments {
            type Output = ();
            const DEFAULT: bool = false;

            fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
                run.path("ferrocene/doc")
            }

            fn make_run(run: RunConfig<'_>) {
                run.builder.ensure(AllSphinxDocuments {
                    target: run.target,
                });
            }

            fn run(self, builder: &Builder<'_>) {
                $(
                    builder.ensure($ty {
                        mode: SphinxMode::Html,
                        target: self.target,
                        fresh_build: false,
                    });
                )*

                // Also regenerate the index file, so that the "Ferrocene documentation" link in
                // the breadcrumbs doesn't break.
                builder.ensure(Index { target: self.target });
            }
        }

        fn intersphinx_gather_steps(target: TargetSelection) -> Vec<SphinxBook> {
            let mut steps = Vec::new();
            $({
                #[allow(unused_mut, unused_assignments)]
                let mut require_test_outcomes = false;
                $(require_test_outcomes = $require_test_outcomes;)*

                #[allow(unused_mut, unused_assignments)]
                let mut require_relnotes = false;
                $(require_relnotes = $require_relnotes;)*

                steps.push(SphinxBook {
                    mode: SphinxMode::OnlyObjectsInv,
                    target,
                    name: $name.into(),
                    src: $src.into(),
                    dest: $dest.into(),
                    fresh_build: false,
                    signature: SignatureStatus::NotNeeded,
                    inject_all_other_document_ids: false,
                    require_test_outcomes,
                    require_relnotes,
                    parent: None,
                });
            })*
            steps
        }

        fn gather_other_document_ids(builder: &Builder<'_>, target: TargetSelection) -> HashMap<&'static str, PathBuf> {
            let mut document_ids = HashMap::new();

            $(
                #[allow(unused_mut, unused_assignments)]
                let mut inject_all_other_document_ids = false;
                $(inject_all_other_document_ids = $inject_all_other_document_ids;)*

                // Avoid recursive builds.
                if !inject_all_other_document_ids {
                    document_ids.insert(
                        $name,
                        builder.ensure($ty {
                            mode: SphinxMode::Html,
                            target,
                            fresh_build: false,
                        }).join("document-id.txt"),
                    );
                }
            )*

            document_ids
        }

        pub(crate) fn ensure_all_xml_doctrees(
            builder: &Builder<'_>,
            target: TargetSelection,
        ) -> HashMap<&'static str, PathBuf> {
            let mut paths = HashMap::new();
            $(paths.insert(
                $name,
                builder.ensure($ty {
                    mode: SphinxMode::XmlDoctrees,
                    target,
                    fresh_build: false,
                })
            );)*
            paths
        }
    };
}

pub(super) use sphinx_books;

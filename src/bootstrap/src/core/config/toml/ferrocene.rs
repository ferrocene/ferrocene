//! Ferrocene specific configuration

use serde::{Deserialize, Deserializer};

use crate::core::config::Merge;
use crate::core::config::toml::ReplaceOpt;
use crate::{HashSet, PathBuf, define_config, exit};

define_config! {
    struct Ferrocene {
        channel: Option<String> = "channel",
        aws_profile: Option<String> = "aws-profile",
        traceability_matrix_mode: Option<String> = "traceability-matrix-mode",
        test_outcomes: Option<String> = "test-outcomes",
        test_outcomes_dir: Option<PathBuf> = "test-outcomes-dir",
        coverage_outcomes: Option<String> = "coverage-outcomes",
        coverage_outcomes_dir: Option<PathBuf> = "coverage-outcomes-dir",
        oxidos_src: Option<String> = "oxidos-src",
        tarball_signing_kms_key_arn: Option<String> = "tarball-signing-kms-key-arn",
        document_signatures: Option<String> = "document-signatures",
        document_signatures_s3_bucket: Option<String> = "document-signatures-s3-bucket",
        document_signatures_tarball: Option<PathBuf> = "document-signatures-tarball",
        compiler_technical_report_url: Option<String> = "compiler-technical-report-url",
        core_technical_report_url: Option<String> = "core-technical-report-url",
        secret_sauce_dir: Option<PathBuf> = "secret-sauce-dir",
        generate_coverage_report_after_test: Option<bool> = "generate-coverage-report-after-tests",
    }
}

use super::newtypes::{PayloadBytes, SignatureBytes};
use super::PublicKey;
use crate::keys::newtypes::PublicKeyBytes;
use crate::keys::{KeyAlgorithm, KeyPair, KeyRole};
use crate::sha256::hash_sha256;
use crate::Error;
use aws_sdk_kms::types::Blob;
use aws_sdk_kms::model::{KeySpec, MessageType, SigningAlgorithmSpec};
use aws_sdk_kms::Client;
use tokio::runtime::Handle;

/// Pair of public and private keys stored in [AWS KMS](https://aws.amazon.com/kms/).
///
/// The private key is exclusively stored inside of AWS KMS, and this struct makes network calls to
/// kMS for every signature request. The public key is downloaded locally when the struct is
/// instantiated, and signatures are verified without making network calls.
pub struct AwsKmsKeyPair {
    handle: Handle,
    kms: Client,
    key_id: String,
    public_key: PublicKey,
}

impl AwsKmsKeyPair {
    /// Load an AWS KMS asymmetric key. The key must exist, and must use one of the algorithms
    /// supported by criticaltrust.
    pub fn new(
        key_id: &str,
        tokio_handle: Handle,
        kms_client: Client,
        role: KeyRole,
    ) -> Result<Self, Error> {
        let public_key_response =
            tokio_handle.block_on(kms_client.get_public_key().key_id(key_id).send())?;

        let public_key = match public_key_response.key_spec() {
            Some(KeySpec::EccNistP256)
                if public_key_response
                    .signing_algorithms()
                    .unwrap_or(&[])
                    .contains(&SigningAlgorithmSpec::EcdsaSha256) =>
            {
                PublicKey {
                    role,
                    algorithm: KeyAlgorithm::EcdsaP256Sha256Asn1SpkiDer,
                    expiry: None,
                    public: PublicKeyBytes::owned(
                        public_key_response
                            .public_key()
                            .unwrap()
                            .clone()
                            .into_inner(),
                    ),
                }
            }
            _ => return Err(Error::UnsupportedKey),
        };

        Ok(Self {
            handle: tokio_handle,
            kms: kms_client,
            key_id: key_id.into(),
            public_key,
        })
    }
}

impl KeyPair for AwsKmsKeyPair {
    fn public(&self) -> &PublicKey {
        &self.public_key
    }

    fn sign(&self, data: &PayloadBytes<'_>) -> Result<SignatureBytes<'static>, Error> {
        let (digest, algorithm) = match self.public_key.algorithm {
            KeyAlgorithm::EcdsaP256Sha256Asn1SpkiDer => (
                hash_sha256(data.as_bytes()),
                SigningAlgorithmSpec::EcdsaSha256,
            ),
            KeyAlgorithm::Unknown => return Err(Error::UnsupportedKey),
        };

        let response = self.handle.block_on(
            self.kms
                .sign()
                .key_id(&self.key_id)
                .message(Blob::new(digest))
                .message_type(MessageType::Digest)
                .signing_algorithm(algorithm)
                .send(),
        )?;

        Ok(SignatureBytes::owned(
            response.signature().unwrap().clone().into_inner(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aws_sdk_kms::config::Credentials;
    use aws_sdk_kms::types::KeyUsageType;
    use rand_core::{OsRng, RngCore};
    use std::process::Command;
    use std::sync::Once;
    use tokio::runtime::Runtime;

    // We want to have tests for all of criticaltrust, which makes testing the integration with AWS
    // KMS quite tricky. To make it work, the tests for this module spawn a Docker container for
    // "localstack", a local replica of AWS services meant for testing.

    #[test]
    fn test_roundtrip() {
        let localstack = Localstack::init();
        let key = localstack.create_key(KeySpec::EccNistP256);

        let keypair = AwsKmsKeyPair::new(
            &key,
            localstack.runtime.handle().clone(),
            localstack.client.clone(),
            KeyRole::Root,
        )
        .expect("failed to create key pair");

        let payload = PayloadBytes::borrowed(b"Hello world");
        let signature = keypair.sign(&payload).expect("failed to sign");
        keypair
            .public()
            .verify(KeyRole::Root, &payload, &signature)
            .expect("failed to verify");
    }

    #[test]
    fn test_key_pair_with_unsupported_algorithm() {
        let localstack = Localstack::init();
        let key = localstack.create_key(KeySpec::Rsa2048);

        let keypair = AwsKmsKeyPair::new(
            &key,
            localstack.runtime.handle().clone(),
            localstack.client.clone(),
            KeyRole::Root,
        );
        assert!(matches!(keypair, Err(Error::UnsupportedKey)));
    }

    struct Localstack {
        runtime: Runtime,
        client: Client,
        container_name: String,
    }

    impl Localstack {
        fn init() -> Self {
            let image = pull_localstack_docker_image();
            let container_name = format!("criticaltrust-localstack-{}", OsRng.next_u64());

            run(Command::new("docker")
                .arg("create")
                .args(["--name", &container_name])
                .args(["-p", "4566"])
                .arg(image));
            run(Command::new("docker").args(["start", &container_name]));

            // localstack is bound to a random port to prevent conflicts between concurrent tests.
            // We thus need to fetch the actual port number Docker assigned.
            let ports = run(Command::new("docker").args(["port", &container_name, "4566/tcp"]));
            let port = std::str::from_utf8(&ports)
                .expect("non-utf-8 output of docker port")
                .split('\n')
                .next()
                .expect("empty output of docker port")
                .rsplit_once(':')
                .expect("invalid output of docker port")
                .1;

            let runtime = Runtime::new().expect("failed to create tokio runtime");
            let aws_config = runtime.block_on(
                aws_config::from_env()
                    // localstack doesn't validate IAM credentials, so we can configure a dummy
                    // secret key and region.
                    .credentials_provider(Credentials::new(
                        "aws_access_key_id",
                        "aws_secret_access_key",
                        None,
                        None,
                        "hardcoded",
                    ))
                    .region("us-east-1")
                    .load(),
            );

            let kms_config = aws_sdk_kms::config::Builder::from(&aws_config)
                .endpoint_url(format!("http://localhost:{port}"))
                .build();
            let client = aws_sdk_kms::Client::from_conf(kms_config);

            Self {
                runtime,
                client,
                container_name,
            }
        }

        fn create_key(&self, spec: KeySpec) -> String {
            self.runtime
                .block_on(
                    self.client
                        .create_key()
                        .key_usage(KeyUsageType::SignVerify)
                        .key_spec(spec)
                        .send(),
                )
                .expect("failed to create kms key")
                .key_metadata()
                .unwrap()
                .key_id()
                .unwrap()
                .into()
        }
    }

    impl Drop for Localstack {
        fn drop(&mut self) {
            run(Command::new("docker").args(["stop", &self.container_name, "-t", "10"]));
            run(Command::new("docker").args(["rm", &self.container_name]));
        }
    }

    fn pull_localstack_docker_image() -> &'static str {
        const IMAGE: &str = "localstack/localstack:2.2.0";
        static ONCE: Once = Once::new();

        ONCE.call_once(|| {
            run(Command::new("docker").args(["pull", IMAGE]));
        });

        IMAGE
    }

    fn run(command: &mut Command) -> Vec<u8> {
        let repr = format!("{command:?}");
        eprintln!("running {repr}");
        match command.output() {
            Ok(output) if output.status.success() => {
                eprintln!("finished running {repr}");
                output.stdout
            }
            Ok(output) => {
                eprintln!("failed to run command: exited with {}", output.status);
                eprintln!("command: {repr}");
                eprintln!(
                    "\nstdout:\n=====\n{}\n=====",
                    String::from_utf8_lossy(&output.stdout)
                );
                eprintln!(
                    "\nstderr:\n=====\n{}\n=====",
                    String::from_utf8_lossy(&output.stderr)
                );
                panic!();
            }
            Err(err) => panic!("command failed to start ({err}: {repr})"),
        }
    }
}

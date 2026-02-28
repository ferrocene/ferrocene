# Ferrocene Specific

This directory contains Ferrocene specific documentation and code.

## SBOM Generation

The Ferrocene SBOM is generated using [syft](https://github.com/anchore/syft).
To generate the SBOM locally, make sure syft is installed:

```sh
curl -sSfL https://get.anchore.io/syft | sudo sh -s -- -b /usr/local/bin
```

Check the Docker files in `ferrocene/ci/docker-images` if you want to install the exact version used in CI.
For other ways to install syft, see https://oss.anchore.com/docs/installation/syft/.

Once syft is installed, run the Ferrocene specific SBOM bootstrap command:

```
./x.py dist ferrocene-sbom
```

This will generate the SBOM under `build/ferrocene-sbom` and create a tarball at `build/dist`.

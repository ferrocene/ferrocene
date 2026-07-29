# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

musl-cross-target-version=v0.9.11
bootstrap-gcc-version=11.5.0
gcc-version=16.1.0
coreutils-version=9.11
binutils-version=2.47
openssl-version=3.5.7
python-version=3.12.3
gdb-version=12.1
cmake-version=4.4.0
arm-gnu-toolchain=15.3.rel1
git-version=2.55.0
commit != git rev-parse HEAD | cut -c 1-8
host-arch != uname -m

# if you want the bootstrap gcc from a previous commit set this
ifdef BOOTSTRAP_GCC_FROM
bootstrap-gcc-from = ${BOOTSTRAP_GCC_FROM}
build-bootstrap-gcc = "false"
else
bootstrap-gcc-from = $(commit)
build-bootstrap-gcc = "true"
endif
# if you want the gcc build from a previous commit set this
ifdef GCC_FROM
gcc-from = ${GCC_FROM}
build-gcc = "false"
else
gcc-from = $(commit)
build-gcc = "true"
endif

prepare: base-image
	mkdir -p output

base-image:
	docker build --tag centos7-base --file base.dockerfile .

download-arm-eabi-none-toolchain:
	mkdir -p build/arm-eabi-none
	aws s3 cp s3://ferrocene-ci-artifacts/manual/arm-gnu-toolchain/arm-gnu-toolchain-$(arm-gnu-toolchain)-$(host-arch)-arm-none-eabi.tar.xz build/arm-eabi-none/arm-gnu-toolchain.tar.xz

repack-arm-eabi-none: download-arm-eabi-none-toolchain
	docker build --tag arm-eabi-none:$(arm-gnu-toolchain) --file repack-arm-toolchain.dockerfile build/arm-eabi-none
	{ \
		id=$$(docker create arm-eabi-none:$(arm-gnu-toolchain)) ; \
		docker cp $$id://repacked/arm-gnu-toolchain-repacked.tar.xz  \
			output/arm-gnu-toolchain-$(arm-gnu-toolchain)-$(host-arch)-arm-none-eabi-repacked-$(commit).tar.xz ; \
		docker rm $$id ; \
	}

upload-arm-eabi-none:
	aws s3 cp --no-overwrite output/arm-gnu-toolchain-$(arm-gnu-toolchain)-$(host-arch)-arm-none-eabi-repacked-$(commit).tar.xz \
		s3://ferrocene-ci-mirrors/manual/arm/arm-gnu-toolchain-$(arm-gnu-toolchain)-$(host-arch)-arm-none-eabi-repacked-$(commit).tar.xz

download-arm-aarch64-none-toolchain:
	mkdir -p build/arm-aarch64-none
	aws s3 cp s3://ferrocene-ci-artifacts/manual/arm-gnu-toolchain/arm-gnu-toolchain-$(arm-gnu-toolchain)-$(host-arch)-aarch64-none-elf.tar.xz build/arm-aarch64-none/arm-gnu-toolchain.tar.xz

repack-arm-aarch64-none: download-arm-aarch64-none-toolchain
	docker build --tag arm-aarch64-none:$(arm-gnu-toolchain) --file repack-arm-toolchain.dockerfile build/arm-aarch64-none
	{ \
		id=$$(docker create arm-aarch64-none:$(arm-gnu-toolchain)) ; \
		docker cp $$id://repacked/arm-gnu-toolchain-repacked.tar.xz  \
			output/arm-gnu-toolchain-$(arm-gnu-toolchain)-$(host-arch)-aarch64-none-elf-repacked-$(commit).tar.xz ; \
		docker rm $$id ; \
	}

upload-arm-aarch64-none:
	aws s3 cp --no-overwrite output/arm-gnu-toolchain-$(arm-gnu-toolchain)-$(host-arch)-aarch64-none-elf-repacked-$(commit).tar.xz \
		s3://ferrocene-ci-mirrors/manual/arm/arm-gnu-toolchain-$(arm-gnu-toolchain)-$(host-arch)-aarch64-none-elf-repacked-$(commit).tar.xz


# we need a gcc 11 as intermediary step as it's the newest you can build with the gcc in centos 7
ifeq ($(build-bootstrap-gcc),"true")
download-bootstrap-gcc-src:
	mkdir -p build/bootstrap-gcc
	aws s3 cp s3://ferrocene-ci-artifacts/manual/gcc/gcc-$(bootstrap-gcc-version)-with-dependencies.tar.xz build/bootstrap-gcc/bootstrap-gcc-with-dependencies.tar.xz

bootstrap-gcc: prepare
	docker build --tag bootstrap-gcc:$(bootstrap-gcc-version) --file bootstrap-gcc.dockerfile build/bootstrap-gcc
	{ \
		id=$$(docker create bootstrap-gcc:$(bootstrap-gcc-version)) ; \
		docker cp $$id://gcc-build/gcc.tar.xz \
			output/gcc-$(bootstrap-gcc-version)-$(host-arch)-$(commit).tar.xz ; \
		docker rm $$id ; \
	}

upload-bootstrap-gcc:
	aws s3 cp --no-overwrite output/output/gcc-$(bootstrap-gcc-version)-$(host-arch)-$(commit).tar.xz \
		s3://ferrocene-ci-mirrors/manual/gcc/gcc-$(bootstrap-gcc-version)-$(host-arch)-$(commit).tar.xz
else

download-bootstrap-gcc-src:

bootstrap-gcc: prepare
	aws s3 cp s3://ferrocene-ci-mirrors/manual/gcc/gcc-$(bootstrap-gcc-version)-$(host-arch)-$(bootstrap-gcc-from).tar.xz output/

upload-bootstrap-gcc:

endif

ifeq ($(build-gcc),"true")

download-gcc-src:
	mkdir -p build/gcc
	aws s3 cp s3://ferrocene-ci-artifacts/manual/gcc/gcc-$(gcc-version)-with-dependencies.tar.xz build/gcc/gcc-with-dependencies.tar.xz
	
gcc: bootstrap-gcc
	cp output/gcc-$(bootstrap-gcc-version)-$(host-arch)-$(bootstrap-gcc-from).tar.xz build/gcc/bootstrap-gcc.tar.xz
	docker build --tag gcc:$(gcc-version) --file gcc.dockerfile build/gcc/
	{ \
		id=$$(docker create gcc:$(gcc-version)) ; \
		docker cp $$id://gcc-build/gcc.tar.xz \
			output/gcc-$(gcc-version)-$(host-arch)-$(commit).tar.xz ; \
		docker rm $$id ; \
	}

upload-gcc:
	aws s3 cp --no-overwrite output/gcc-$(gcc-version)-$(host-arch)-$(commit).tar.xz \
		s3://ferrocene-ci-mirrors/manual/gcc/gcc-$(gcc-version)-$(host-arch)-$(commit).tar.xz

else

download-gcc-src:

gcc:
	aws s3 cp s3://ferrocene-ci-mirrors/manual/gcc/gcc-$(gcc-version)-$(host-arch)-$(gcc-from).tar.xz output/

upload-gcc:
endif

download-utils-src:
	mkdir -p build/utils
	aws s3 cp s3://ferrocene-ci-artifacts/manual/binutils/binutils-$(binutils-version).tar.xz build/utils/binutils-src.tar.xz
	aws s3 cp s3://ferrocene-ci-artifacts/manual/coreutils/coreutils-$(coreutils-version).tar.xz build/utils/coreutils-src.tar.xz
	
utils: gcc
	cp output/gcc-$(gcc-version)-$(host-arch)-$(gcc-from).tar.xz build/utils/gcc.tar.xz
	docker build --tag utils:$(binutils-version)-$(coreutils-version) --file utils.dockerfile build/utils
	{ \
		id=$$(docker create utils:$(binutils-version)-$(coreutils-version)) ; \
		docker cp $$id://build/binutils-binaries.tar.xz \
			output/binutils-$(binutils-version)-$(host-arch)-$(commit).tar.xz ; \
		docker cp $$id://build/coreutils-binaries.tar.xz \
			output/coreutils-$(coreutils-version)-$(host-arch)-$(commit).tar.xz ; \
		docker rm $$id ; \
	}

upload-utils:
	aws s3 cp --no-overwrite output/output/binutils-$(binutils-version)-$(host-arch)-$(commit).tar.xz \
		s3://ferrocene-ci-mirrors/manual/binutils/binutils-$(binutils-version)-$(host-arch)-$(commit).tar.xz
	aws s3 cp --no-overwrite output/output/coreutils-$(coreutils-version)-$(host-arch)-$(commit).tar.xz \
		s3://ferrocene-ci-mirrors/manual/coreutils/coreutils-$(coreutils-version)-$(host-arch)-$(commit).tar.xz

download-openssl-src:
	mkdir -p build/openssl
	aws s3 cp s3://ferrocene-ci-artifacts/manual/openssl/openssl-$(openssl-version).tar.gz build/openssl/openssl.tar.gz

openssl: gcc utils
	cp output/gcc-$(gcc-version)-$(host-arch)-$(gcc-from).tar.xz build/openssl/gcc.tar.xz
	cp output/binutils-$(binutils-version)-$(host-arch)-$(commit).tar.xz build/openssl/binutils.tar.xz
	docker build --tag openssl:$(openssl-version) --file openssl.dockerfile build/openssl
	{ \
		id=$$(docker create openssl:$(openssl-version)) ; \
		docker cp $$id://openssl-build/openssl-binaries.tar.xz \
			output/openssl-$(openssl-version)-$(host-arch)-$(commit).tar.xz ; \
		docker rm $$id ; \
	}

upload-openssl:
	aws s3 cp --no-overwrite output/output/openssl-$(openssl-version)-$(host-arch)-$(commit).tar.xz \
		s3://ferrocene-ci-mirrors/manual/openssl/openssl-$(openssl-version)-$(host-arch)-$(commit).tar.xz


builder-image: builder.dockerfile gcc openssl utils
	mkdir -p build/builder
	cp output/gcc-$(gcc-version)-$(host-arch)-$(gcc-from).tar.xz build/builder/gcc.tar.xz
	cp output/binutils-$(binutils-version)-$(host-arch)-$(commit).tar.xz build/builder/binutils.tar.xz
	cp output/openssl-$(openssl-version)-$(host-arch)-$(commit).tar.xz build/builder/openssl.tar.xz
	docker build --tag centos7-builder --file builder.dockerfile build/builder

builder-python-image: builder-image builder-python.dockerfile
	mkdir -p build/builder-python
	cp output/python-$(python-version)-$(host-arch)-$(commit).tar.xz build/builder-python/python.tar.xz
	docker build --tag centos7-python-builder --build-arg=PYTHON_VERSION=$(python-version)  --file builder-python.dockerfile build/builder-python

download-musl-cross-src:
	mkdir -p build/musl-cross
	aws s3 cp s3://ferrocene-ci-artifacts/manual/musl-cross-make/musl-cross-make-$(musl-cross-target-version)-with-source.tar.xz build/musl-cross/musl-cross-make.tar.xz
	
musl-cross: musl.dockerfile builder-image prepare
	docker build -t musl-tarballs:$(musl-cross-target-version) --target tarballs --build-arg TARGETVERSION=$(musl-cross-target-version) -f musl.dockerfile build/musl-cross
	{ \
		set -xe ; \
		id=$$(docker create musl-tarballs:$(musl-cross-target-version) ) ; \
		docker cp $$id:/musl-cross-make/musl-cross-make-aarch64-$(musl-cross-target-version).tar.xz \
			output/musl-cross-make-$(host-arch)-to-aarch64-$(musl-cross-target-version)-$(commit).tar.xz ; \
		docker cp $$id:/musl-cross-make/musl-cross-make-x86_64-$(musl-cross-target-version).tar.xz \
			output/musl-cross-make-$(host-arch)-to-x86_64-$(musl-cross-target-version)-$(commit).tar.xz ; \
		docker rm $$id ; \
	}

upload-musl-cross:
	aws s3 cp --no-overwrite output/musl-cross-make-$(host-arch)-to-aarch64-$(musl-cross-target-version)-$(commit).tar.xz \
		s3://ferrocene-ci-mirrors/manual/musl/musl-cross-make-$(host-arch)-to-aarch64-$(musl-cross-target-version)-$(commit).tar.xz
	aws s3 cp --no-overwrite output/musl-cross-make-$(host-arch)-to-x86_64-$(musl-cross-target-version)-$(commit).tar.xz \
		s3://ferrocene-ci-mirrors/manual/musl/musl-cross-make-$(host-arch)-to-x86_64-$(musl-cross-target-version)-$(commit).tar.xz

download-python-src:
	mkdir -p build/python
	aws s3 cp s3://ferrocene-ci-artifacts/manual/python/Python-$(python-version).tar.xz build/python/python-source.tar.xz

python: python.dockerfile builder-image prepare
	docker build -t python:v$(python-version) --build-arg=PYTHON_VERSION=$(python-version) -f python.dockerfile build/python
	{ \
		set -xe ; \
		id=$$(docker create python:v$(python-version)) ; \
		docker cp $$id:/python/python-binaries.tar.xz \
			output/python-$(python-version)-$(host-arch)-$(commit).tar.xz ; \
		docker rm $$id ; \
	}

upload-python:
	aws s3 cp --no-overwrite output/python-$(python-version)-$(host-arch)-$(commit).tar.xz \
		s3://ferrocene-ci-mirrors/manual/python/python-$(python-version)-$(host-arch)-$(commit).tar.xz

download-gdb-src:
	mkdir -p build/gdb
	aws s3 cp s3://ferrocene-ci-artifacts/manual/gdb/gdb_$(gdb-version).orig.tar.xz build/gdb/gdb-source.tar.xz

gdb: gdb.dockerfile builder-python-image prepare
	docker build -t gdb:v$(gdb-version)  --build-arg=PYTHON_VERSION=$(python-version)  -f gdb.dockerfile build/gdb
	{ \
		set -xe ; \
		id=$$(docker create gdb:v$(gdb-version)) ; \
		docker cp $$id:/gdb/gdb-binaries.tar.xz \
			output/gdb-$(gdb-version)-$(host-arch)-$(commit).tar.xz ; \
		docker rm $$id ; \
	}

upload-gdb:
	aws s3 cp --no-overwrite output/gdb-$(gdb-version)-$(host-arch)-$(commit).tar.xz \
		s3://ferrocene-ci-mirrors/manual/gdb/gdb-$(gdb-version)-$(host-arch)-$(commit).tar.xz

download-git-src:
	mkdir -p build/git
	aws s3 cp s3://ferrocene-ci-artifacts/manual/git/git-$(git-version).tar.xz build/git/git-source.tar.xz

git: git.dockerfile builder-python-image prepare
	docker build -t git:v$(git-version) -f git.dockerfile build/git
	{ \
		set -xe ; \
		id=$$(docker create git:v$(git-version)) ; \
		docker cp $$id:/git/git-binaries.tar.xz \
			output/git-$(git-version)-$(host-arch)-$(commit).tar.xz ; \
		docker rm $$id ; \
	}

upload-git:
	aws s3 cp --no-overwrite output/git-$(git-version)-$(host-arch)-$(commit).tar.xz \
		s3://ferrocene-ci-mirrors/manual/git/git-$(git-version)-$(host-arch)-$(commit).tar.xz

download-all-src: download-musl-cross-src download-python-src download-gdb-src download-git-src download-openssl-src download-bootstrap-gcc-src download-gcc-src download-utils-src download-arm-aarch64-none-toolchain download-arm-eabi-none-toolchain
build-all: musl-cross python gdb git utils openssl bootstrap-gcc gcc repack-arm-aarch64-none repack-arm-eabi-none
upload-all: upload-musl-cross upload-python upload-gdb upload-git upload-utils upload-openssl upload-bootstrap-gcc upload-gcc upload-arm-aarch64-none upload-arm-eabi-none

update-hashes:
	mkdir -p hashes
	aws s3 cp s3://ferrocene-ci-mirrors/manual/gcc/gcc-$(gcc-version)-aarch64-$(gcc-from).tar.xz hashes/
	aws s3 cp s3://ferrocene-ci-mirrors/manual/coreutils/coreutils-$(coreutils-version)-aarch64-$(commit).tar.xz hashes/
	aws s3 cp s3://ferrocene-ci-mirrors/manual/binutils/binutils-$(binutils-version)-aarch64-$(commit).tar.xz hashes/
	aws s3 cp s3://ferrocene-ci-mirrors/manual/musl/musl-cross-make-aarch64-to-aarch64-$(musl-cross-target-version)-$(commit).tar.xz hashes
	aws s3 cp s3://ferrocene-ci-mirrors/manual/musl/musl-cross-make-aarch64-to-x86_64-$(musl-cross-target-version)-$(commit).tar.xz hashes
	aws s3 cp s3://ferrocene-ci-mirrors/manual/python/python-$(python-version)-aarch64-$(commit).tar.xz hashes/
	aws s3 cp s3://ferrocene-ci-mirrors/manual/gdb/gdb-$(gdb-version)-aarch64-$(commit).tar.xz hashes/
	aws s3 cp s3://ferrocene-ci-mirrors/manual/cmake/cmake-$(cmake-version)-linux-aarch64.tar.gz hashes/
	aws s3 cp s3://ferrocene-ci-mirrors/manual/openssl/openssl-$(openssl-version)-aarch64-$(commit).tar.xz hashes/
	aws s3 cp s3://ferrocene-ci-mirrors/manual/arm/arm-gnu-toolchain-$(arm-gnu-toolchain)-aarch64-arm-none-eabi-repacked-$(commit).tar.xz hashes/
	aws s3 cp s3://ferrocene-ci-mirrors/manual/arm/arm-gnu-toolchain-$(arm-gnu-toolchain)-aarch64-aarch64-none-elf-repacked-$(commit).tar.xz hashes/

	aws s3 cp s3://ferrocene-ci-mirrors/manual/gcc/gcc-$(gcc-version)-x86_64-$(gcc-from).tar.xz hashes/
	aws s3 cp s3://ferrocene-ci-mirrors/manual/coreutils/coreutils-$(coreutils-version)-x86_64-$(commit).tar.xz hashes/
	aws s3 cp s3://ferrocene-ci-mirrors/manual/binutils/binutils-$(binutils-version)-x86_64-$(commit).tar.xz hashes/
	aws s3 cp s3://ferrocene-ci-mirrors/manual/musl/musl-cross-make-x86_64-to-aarch64-$(musl-cross-target-version)-$(commit).tar.xz hashes
	aws s3 cp s3://ferrocene-ci-mirrors/manual/musl/musl-cross-make-x86_64-to-x86_64-$(musl-cross-target-version)-$(commit).tar.xz hashes
	aws s3 cp s3://ferrocene-ci-mirrors/manual/python/python-$(python-version)-x86_64-$(commit).tar.xz hashes/
	aws s3 cp s3://ferrocene-ci-mirrors/manual/gdb/gdb-$(gdb-version)-x86_64-$(commit).tar.xz hashes/
	# cmake is taken from the binary distribution and just manually uploaded, so it's never rebuilt and carries no commit identifier
	aws s3 cp s3://ferrocene-ci-mirrors/manual/cmake/cmake-$(cmake-version)-linux-x86_64.tar.gz hashes/
	aws s3 cp s3://ferrocene-ci-mirrors/manual/openssl/openssl-$(openssl-version)-x86_64-$(commit).tar.xz hashes/
	aws s3 cp s3://ferrocene-ci-mirrors/manual/arm/arm-gnu-toolchain-$(arm-gnu-toolchain)-x86_64-arm-none-eabi-repacked-$(commit).tar.xz hashes/
	aws s3 cp s3://ferrocene-ci-mirrors/manual/arm/arm-gnu-toolchain-$(arm-gnu-toolchain)-x86_64-aarch64-none-elf-repacked-$(commit).tar.xz hashes/

	cd hashes; sha256sum *tar.xz *tar.gz > ../hashes.txt

clean-build:
	rm -rf build/

clean-output:
	rm -rf output/

clean-docker:
	docker image rm centos7-base
	docker image rm centos7-builder
	docker image rm centos7-python-builder
	docker image rm gcc:$(gcc-version)
	docker image rm bootstrap-gcc:$(bootstrap-gcc-version)
	docker image rm musl-tarballs:$(musl-cross-target-version)
	docker image rm cmake:v$(cmake-version)
	docker image rm gdb:v$(gdb-version)
	docker image rm python:v$(python-version)

clean-all: clean-build clean-output
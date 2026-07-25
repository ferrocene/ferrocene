# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

musl-cross-target-version=v0.9.11
gcc-11-version=11.5.0
gcc-version=16.1.0
python-version=3.12.3
gdb-version=12.1
cmake-version=3.21.1
commit != git rev-parse HEAD | cut -c 1-8
host-arch != uname -m

prepare: base-image
	mkdir -p output

base-image:
	docker build --tag centos7-base --file base.dockerfile .

# we need a gcc 11 as intermediary step as it's the newest you can build with the gcc in centos 7
.ONESHELL:
gcc-11: prepare
	set -xe
	mkdir -p build/gcc-11
	cd build/gcc-11
	aws s3 cp s3://ferrocene-ci-artifacts/manual/gcc/gcc-$(gcc-11-version)-with-dependencies.tar.xz gcc-11-with-dependencies.tar.xz
	docker build --tag gcc-11:$(gcc-11-version) --file ../../gcc-11.dockerfile .
	{ \
		id=$$(docker create gcc-11:$(gcc-11-version)) ; \
		docker cp $$id://gcc-build/gcc.tar.xz \
			../../output/gcc-$(gcc-11-version)-$(host-arch)-$(commit).tar.xz ; \
		docker rm $$id ; \
	}

upload-gcc-11: gcc-11
	aws s3 cp --no-overwrite output/output/gcc-$(gcc-11-version)-$(host-arch)-$(commit).tar.xz \
		s3://ferrocene-ci-mirrors/manual/gcc/output/gcc-$(gcc-11-version)-$(host-arch)-$(commit).tar.xz

.ONESHELL:
gcc: gcc-11
	set -xe
	mkdir -p build/gcc
	cd build/gcc
	cp ../../output/gcc-$(gcc-11-version)-$(host-arch)-$(commit).tar.xz gcc-11.tar.xz
	aws s3 cp s3://ferrocene-ci-artifacts/manual/gcc/gcc-$(gcc-version)-with-dependencies.tar.xz gcc-with-dependencies.tar.xz
	docker build --tag gcc:$(gcc-version) --file ../../gcc.dockerfile .
	{ \
		id=$$(docker create gcc:$(gcc-version)) ; \
		docker cp $$id://gcc-build/gcc.tar.xz \
			../../output/gcc-$(gcc-version)-$(host-arch)-$(commit).tar.xz ; \
		docker rm $$id ; \
	}

upload-gcc: gcc
	aws s3 cp --no-overwrite output/output/gcc-$(gcc-version)-$(host-arch)-$(commit).tar.xz \
		s3://ferrocene-ci-mirrors/manual/gcc/output/gcc-$(gcc-version)-$(host-arch)-$(commit).tar.xz

.ONESHELL:
builder-image: gcc
	set -xe
	mkdir -p build/builder
	cd build/builder
	cp ../../output/gcc-$(gcc-version)-$(host-arch)-$(commit).tar.xz gcc.tar.xz
	docker build --tag centos7-builder --file builder.dockerfile .

.ONESHELL:
musl-cross: musl.dockerfile builder-image prepare
	mkdir -p build/musl-cross
	cd build/musl-cross
	aws s3 cp s3://ferrocene-ci-artifacts/manual/musl-cross-make/musl-cross-make-$(musl-cross-target-version)-with-source.tar.xz musl-cross-make.tar.xz
	docker build -t musl-tarballs:$(musl-cross-target-version) --target tarballs --build-arg TARGETVERSION=$(musl-cross-target-version) -f ../../musl.dockerfile .
	{ \
		set -xe ; \
		id=$$(docker create musl-tarballs:$(musl-cross-target-version) ) ; \
		docker cp $$id:/musl-cross-make/musl-cross-make-aarch64-$(musl-cross-target-version).tar.xz \
			../../output/musl-cross-make-$(host-arch)-to-aarch64-$(musl-cross-target-version)-$(commit).tar.xz ; \
		docker cp $$id:/musl-cross-make/musl-cross-make-x86_64-$(musl-cross-target-version).tar.xz \
			../../output/musl-cross-make-$(host-arch)-to-x86_64-$(musl-cross-target-version)-$(commit).tar.xz ; \
		docker rm $$id ; \
	}

upload-musl-cross: musl-cross
	aws s3 cp --no-overwrite output/musl-cross-make-$(host-arch)-to-aarch64-$(musl-cross-target-version)-$(commit).tar.xz \
		s3://ferrocene-ci-mirrors/manual/musl/musl-cross-make-$(host-arch)-to-aarch64-$(musl-cross-target-version)-$(commit).tar.xz
	aws s3 cp --no-overwrite output/musl-cross-make-$(host-arch)-to-x86_64-$(musl-cross-target-version)-$(commit).tar.xz \
		s3://ferrocene-ci-mirrors/manual/musl/musl-cross-make-$(host-arch)-to-x86_64-$(musl-cross-target-version)-$(commit).tar.xz

.ONESHELL:
python: python.dockerfile builder-image prepare
	mkdir -p build/python
	cd build/python
	aws s3 cp s3://ferrocene-ci-artifacts/manual/python/Python-$(python-version).tar.xz python-source.tar.xz
	docker build -t python:v$(python-version) -f ../../python.dockerfile .
	{ \
		set -xe ; \
		id=$$(docker create python:v$(python-version)) ; \
		docker cp $$id:/python/python-binaries.tar.xz \
			../../output/python-$(python-version)-$(host-arch)-$(commit).tar.xz ; \
		docker rm $$id ; \
	}

upload-python: python
	aws s3 cp --no-overwrite output/python-$(python-version)-$(host-arch)-$(commit).tar.xz \
		s3://ferrocene-ci-mirrors/manual/python/python-$(python-version)-$(host-arch)-$(commit).tar.xz

.ONESHELL:
gdb: gdb.dockerfile builder python-image prepare
	mkdir -p build/gdb
	cd build/gdb
	cp ../../output/python-$(python-version)-$(host-arch)-$(commit).tar.xz python.tar.xz
	aws s3 cp s3://ferrocene-ci-artifacts/manual/gdb/gdb_$(gdb-version).orig.tar.xz gdb-source.tar.xz
	docker build -t gdb:v$(gdb-version) -f ../../gdb.dockerfile .
	{ \
		set -xe ; \
		id=$$(docker create gdb:v$(gdb-version)) ; \
		docker cp $$id:/gdb/gdb-binaries.tar.xz \
			../../output/gdb-$(gdb-version)-$(host-arch)-$(commit).tar.xz ; \
		docker rm $$id ; \
	}

upload-gdb: gdb
	aws s3 cp --no-overwrite output/gdb-$(gdb-version)-$(host-arch)-$(commit).tar.xz \
		s3://ferrocene-ci-mirrors/manual/gdb/gdb-$(gdb-version)-$(host-arch)-$(commit).tar.xz

.ONESHELL:
cmake: cmake.dockerfile builder-image prepare
	mkdir -p build/cmake
	cd build/cmake
	aws s3 cp s3://ferrocene-ci-artifacts/manual/cmake/cmake-$(cmake-version).tar.gz cmake-source.tar.gz
	docker build -t cmake:v$(cmake-version) -f ../../cmake.dockerfile .
	{ \
		set -xe ; \
		id=$$(docker create cmake:v$(cmake-version)) ; \
		docker cp $$id:/cmake/cmake-binaries.tar.xz \
			../../output/cmake-$(cmake-version)-$(host-arch)-$(commit).tar.xz ; \
		docker rm $$id ; \
	}

upload-cmake: cmake
	aws s3 cp --no-overwrite output/cmake-$(cmake-version)-$(host-arch)-$(commit).tar.xz \
		s3://ferrocene-ci-mirrors/manual/cmake/cmake-$(cmake-version)-$(host-arch)-$(commit).tar.xz

make-all: musl-cross python gdb cmake gcc-11 gcc
upload-all: upload-musl-cross upload-python upload-gdb upload-cmake gcc-11 gcc

.ONESHELL:
update-hashes:
	mkdir -p hashes
	aws s3 cp s3://ferrocene-ci-mirrors/manual/gcc/output/gcc-$(gcc-11-version)-$(host-arch)-$(commit).tar.xz hashes/
	aws s3 cp s3://ferrocene-ci-mirrors/manual/gcc/output/gcc-$(gcc-version)-$(host-arch)-$(commit).tar.xz hashes/
	aws s3 cp s3://ferrocene-ci-mirrors/manual/musl/musl-cross-make-aarch64-to-aarch64-$(musl-cross-target-version)-$(commit).tar.xz hashes
	aws s3 cp s3://ferrocene-ci-mirrors/manual/musl/musl-cross-make-aarch64-to-x86_64-$(musl-cross-target-version)-$(commit).tar.xz hashes
	aws s3 cp s3://ferrocene-ci-mirrors/manual/python/python-$(python-version)-aarch64-$(commit).tar.xz hashes/
	aws s3 cp s3://ferrocene-ci-mirrors/manual/gdb/gdb-$(gdb-version)-aarch64-$(commit).tar.xz hashes/
	aws s3 cp s3://ferrocene-ci-mirrors/manual/cmake/cmake-$(cmake-version)-aarch64-$(commit).tar.xz hashes/
	aws s3 cp s3://ferrocene-ci-mirrors/manual/musl/musl-cross-make-x86_64-to-aarch64-$(musl-cross-target-version)-$(commit).tar.xz hashes
	aws s3 cp s3://ferrocene-ci-mirrors/manual/musl/musl-cross-make-x86_64-to-x86_64-$(musl-cross-target-version)-$(commit).tar.xz hashes
	aws s3 cp s3://ferrocene-ci-mirrors/manual/python/python-$(python-version)-x86_64-$(commit).tar.xz hashes/
	aws s3 cp s3://ferrocene-ci-mirrors/manual/gdb/gdb-$(gdb-version)-x86_64-$(commit).tar.xz hashes/
	aws s3 cp s3://ferrocene-ci-mirrors/manual/cmake/cmake-$(cmake-version)-x86_64-$(commit).tar.xz hashes/
	cd hashes; sha256sum *tar.xz > ../hashes.txt

clean-build:
	rm -rf build/

clean-output:
	rm -rf output/

clean-docker:
	docker image rm centos7-base
	docker image rm centos7-builder
	docker image rm gcc:$(gcc-version)
	docker image rm gcc-11:$(gcc-11-version)
	docker image rm musl-tarballs:$(musl-cross-target-version)
	docker image rm cmake:v$(cmake-version)
	docker image rm gdb:v$(gdb-version)
	docker image rm python:v$(python-version)

clean-all: clean-build clean-output clean-docker
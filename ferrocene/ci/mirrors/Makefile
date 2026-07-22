# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

musl-cross-target-version=v0.9.11
python-version=3.12.3
gdb-version=12.1
cmake-version=3.21.1

prepare:
	mkdir output

.ONESHELL:
musl-cross: musl.dockerfile prepare
	mkdir -p build/musl-cross
	cd build/musl-cross
	aws s3 cp s3://ferrocene-ci-artifacts/manual/musl-cross-make/musl-cross-make-$(musl-cross-target-version)-with-source.tar.xz musl-cross-make.tar.xz
	tar xf musl-cross-make.tar.xz
	docker build -t musl-tarballs --target tarballs --build-arg TARGETVERSION=$(musl-cross-target-version) -f ../../musl.dockerfile .
	{ \
		set -xe ; \
		id=$$(docker create musl-tarballs) ; \
		docker cp $$id:/musl-cross-make/musl-cross-make-aarch64-$(musl-cross-target-version).tar.xz \
			../../output/musl-cross-make-$$(uname -p)-aarch64-$(musl-cross-target-version).tar.xz ; \
		docker cp $$id:/musl-cross-make/musl-cross-make-x86_64-$(musl-cross-target-version).tar.xz \
			../../output/musl-cross-make-$$(uname -p)-x86_64-$(musl-cross-target-version).tar.xz ; \
		docker rm $$id ; \
	}

upload-musl-cross: musl-cross
	aws s3 cp output/musl-cross-make-$$(uname -p)-aarch64-$(musl-cross-target-version).tar.xz s3://ferrocene-ci-artifacts/manual/musl-cross-make/binaries/musl-cross-make-$$(uname -p)-aarch64-$(musl-cross-target-version).tar.xz
	aws s3 cp output/musl-cross-make-$$(uname -p)-x86_64-$(musl-cross-target-version).tar.xz s3://ferrocene-ci-artifacts/manual/musl-cross-make/binaries/musl-cross-make-$$(uname -p)-aarcx86_64h64-$(musl-cross-target-version).tar.xz

.ONESHELL:
python: python.dockerfile prepare
	mkdir -p build/python
	cd build/python
	aws s3 cp s3://ferrocene-ci-artifacts/manual/python/Python-$(python-version).tar.xz python-source.tar.xz
	docker build -t python:v$(python-version) -f ../../python.dockerfile .
	{ \
		set -xe ; \
		id=$$(docker create python:v$(python-version)) ; \
		docker cp $$id:/python/python-binaries.tar.xz \
			../../output/python-$$(uname -p)-$(python-version).tar.xz ; \
		docker rm $$id ; \
	}

upload-python: python
        aws s3 cp output/python-$$(uname -p)-$(python-version).tar.xz s3://ferrocene-ci-artifacts/manual/python/binaries/python-$$(uname -p)-$(python-version).tar.xz

.ONESHELL:
gdb: gdb.dockerfile prepare
	mkdir -p build/gdb
	cd build/gdb
	aws s3 cp s3://ferrocene-ci-artifacts/manual/gdb/gdb_$(gdb-version).orig.tar.xz gdb-source.tar.xz
	docker build -t gdb:v$(gdb-version) -f ../../gdb.dockerfile .
	{ \
		set -xe ; \
		id=$$(docker create gdb:v$(gdb-version)) ; \
		docker cp $$id:/gdb/gdb-binaries.tar.xz \
			../../output/gdb-$$(uname -p)-$(gdb-version).tar.xz ; \
		docker rm $$id ; \
	}

upload-gdb: gdb
	aws s3 cp output/gdb-$$(uname -p)-$(gdb-version).tar.xz s3://ferrocene-ci-artifacts/manual/gdb/binaries/gdb-$$(uname -p)-$(gdb-version).tar.xz

.ONESHELL:
cmake: cmake.dockerfile prepare
	mkdir -p build/cmake
	cd build/cmake
	aws s3 cp s3://ferrocene-ci-artifacts/manual/cmake/cmake-$(cmake-version).tar.gz cmake-source.tar.gz
	docker build -t cmake:v$(cmake-version) -f ../../cmake.dockerfile .
	{ \
		set -xe ; \
		id=$$(docker create cmake:v$(cmake-version)) ; \
		docker cp $$id:/cmake/cmake-binaries.tar.xz \
			../../output/cmake-$$(uname -p)-$(cmake-version).tar.xz ; \
		docker rm $$id ; \
	}

upload-cmake: cmake
	aws s3 cp output/cmake-$$(uname -p)-$(cmake-version).tar.xz s3://ferrocene-ci-artifacts/manual/cmake/binaries/cmake-$$(uname -p)-$(cmake-version).tar.xz

make-all: musl-cross python gdb cmake
upload-all: upload-musl-cross upload-python upload-gdb upload-cmake

clean-build:
	rm -rf build/
#   docker image rm musl-tarballs
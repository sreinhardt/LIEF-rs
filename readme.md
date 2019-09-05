### LIEF-RS
#### Spenser Reinhardt

Extending [LIEF]() to Rust!


##### Building LIEF


* Linux
	```bash
	make_release=false
	python_libs=false

	git clone https://github.com/lief-project/LIEF.git
	cd LIEF
	mkdir build
	cd build

	if $python_libs; then
		DPYTHON="-DLIEF_PYTHON_API=on -DPYTHON_VERSION=3.6"
	else
		DPYTHON="-DLIEF_PYTHON_API=off"
	fi

	if $make_release; then
		DRELEASE="Release"
	else
		DRELEASE="Debug"
	fi

	cmake ${DPYTHON} -DCMAKE_BUILD_TYPE=${DRELEASE} ..
	cmake --build . --target LIB_LIEF --config ${DRELEASE}
	if $python_libs; then
		cmake --build . --target pyLIEF --config ${DRELEASE}
	fi
	```
* Windows
	```batch
	@echo off
	@setlocal EnableExtensions EnableDelayedExpansion

	set "python_libs=false"
	set "use_crt=false"
	set "make_release=dbg"

	git clone https://github.com/lief-project/LIEF.git
	cd LIEF
	mkdir build
	cd build

	if "!python_libs!" == "true" (
		set "DPYTHON=-DLIEF_PYTHON_API=on -DPYTHON_VERSION=3.6"
	) else (
		set "DPYTHON=-DLIEF_PYTHON_API=off"
	)

	if "!make_release!" == "dbg" (
		set "DRELEASE=Debug"
		set "DCRT=MTd"
	) else (
		set "DRELEASE=Release"
		set "DCRT=MT"
	)

	if "!use_crt!" == "true" (
		cmake %DPYTHON% -DCMAKE_BUILD_TYPE=!DRELEASE! -DLIEF_USE_CRT_RELEASE=!DCRT! ..
		cmake --build . --target LIB_LIEF --config
	) else (
		cmake !DPYTHON! -DCMAKE_BUILD_TYPE=!DRELEASE! ..
		cmake --build . --target LIB_LIEF --config !DRELEASE!
	)

	if "!python_libs!" == "true" (
		cmake --build . --target pyLIEF --config Release
	)
	```

#### Building LIEF-RS

* Linux
	```bash
	LIEF_LIB_PATH="../LIEF/build"              # has libLIEF.a
	LIEF_INC_PATH="../LIEF/api/c/include/LIEF" # has LIEF.h
	```

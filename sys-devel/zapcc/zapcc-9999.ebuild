# Copyright 1999-2019 Gentoo Authors
# Distributed under the terms of the GNU General Public License v2

EAPI=6

: ${CMAKE_MAKEFILE_GENERATOR:=ninja}
# (needed due to CMAKE_BUILD_TYPE != Gentoo)
CMAKE_MIN_VERSION=3.7.0-r1
PYTHON_COMPAT=( python2_7 )

inherit cmake-utils eapi7-ver flag-o-matic git-r3 \
	multiprocessing pax-utils python-any-r1 toolchain-funcs

DESCRIPTION="ZapCC caching c++ compiler"
HOMEPAGE="https://www.zapcc.com/"
SRC_URI=""
EGIT_REPO_URI="https://github.com/yrnkrn/zapcc.git"
LICENSE="UoI-NCSA"

SLOT="9"
KEYWORDS=""

DEPEND="${PYTHON_DEPS}"
REQUIRED_USE="${PYTHON_REQUIRED_USE}"

CMAKE_BUILD_TYPE=Release

src_compile() {
	cmake-utils_src_compile
}

src_install() {
	cmake-uitls_src_install
}

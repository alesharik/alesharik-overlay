# Copyright 1999-2019 Gentoo Authors
# Distributed under the terms of the GNU General Public License v2
# $Header: $

EAPI=5

inherit bzr eutils flag-o-matic

DESCRIPTION="Utility to replace a desktop background with a movie, screensaver, etc."
KEYWORDS="~amd64"
LICENSE="HPND"
HOMEPAGE="http://tech.shantanugoel.com/projects/linux/shantz-xwinwrap"
EBZR_REPO_URI="lp:xwinwrap"

SLOT="0"

IUSE=""

DEPEND=">=x11-libs/libX11-1.0.3
	>=media-libs/mesa-6.5
	dev-vcs/bzr"

RDEPEND="${DEPEND}"

src_prepare() {
	cp "$FILESDIR"/Makefile ./
}

src_install() {
	dobin xwinwrap
}

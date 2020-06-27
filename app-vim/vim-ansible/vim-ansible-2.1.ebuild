# Copyright 1999-2020 Gentoo Authors
# Distributed under the terms of the GNU General Public License v2

EAPI=7

inherit vim-plugin

if [[ ${PV} == 9999 ]]; then
	inherit git-r3
	EGIT_REPO_URI="https://github.com/pearofducks/ansible-vim.git"
else
	SRC_URI="https://github.com/pearofducks/ansible-vim/archive/${PV}.tar.gz -> ${P}.tar.gz"
	KEYWORDS="~amd64 ~x86 ~ppc-macos"
fi

DESCRIPTION="vim plugin: ansible support for vim"
HOMEPAGE="https://github.com/pearofducks/ansible-vim/"
LICENSE="MIT"

src_prepare() {
	rm -r UltiSnips || die
	default
}

# Copyright 1999-2019 Gentoo Authors
# Distributed under the terms of the GNU General Public License v2

EAPI=7

DESCRIPTION="Java profiles which does not suffer from Safepoint bias problem"
HOMEPAGE="https://github.com/jvm-profiling-tools/async-profiler"
SRC_URI="https://github.com/jvm-profiling-tools/async-profiler/archive/v1.5.tar.gz"

LICENSE="Apache-2.0"
SLOT="0"
KEYWORDS="~amd64 ~x86"
IUSE=""

DEPEND=">=virtual/jdk-1.8:*"
RDEPEND="${DEPEND} >=sys-devel/gcc-5.0.0"
BDEPEND=""

pkg_preinst() {
	dodir /usr/bin
	dosym "${D}/profiler.sh" /usr/bin/async-profiler
	grep -qxF '# AsyncProfiler required options' /etc/sysctl.conf || echo "# AsyncProfiler required options" >> /etc/sysctl.conf
	grep -qxF 'perk_event_paranoid = 1' /etc/sysctl.conf || echo "perk_event_paranoid = 1" >> /etc/sysctl.conf
	grep -qxF 'kptr_restrict = 0' /etc/sysctl.conf || echo "kptr_restrict = 0" >> /etc/sysctl.conf
	sysctl -p
}

bin_PROGRAMS = daemonizer
noinst_PROGRAMS = testd

daemonizer_CFLAGS  = -Wall $(AM_CLFAGS)
daemonizer_SOURCES = daemonizer.c daemonizer.h

testd_CFLAGS  = -Wall $(AM_CLFAGS)
testd_SOURCES = testd.c

man_MANS = daemonizer.1
EXTRA_DIST = $(man_MANS) daemonizer.spec DEVELOPMENT LICENSE COPYRIGHT

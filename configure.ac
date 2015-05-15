#                                               -*- Autoconf -*-
# Process this file with autoconf to produce a configure script.

AC_PREREQ([2.69])
AC_INIT([caesiumclt], [1.9.9], [caesium@saerasoft.com])
AC_CONFIG_SRCDIR([src/])
AC_CONFIG_AUX_DIR(tools)
AM_INIT_AUTOMAKE(-Wall -Werror foreign)

# Checks for programs.
AC_PROG_CC

AC_CONFIG_FILES([
 Makefile
 src/Makefile
])

AC_ARG_WITH(libjpeg, [  --libjpeg=DIR	  libjpeg is installed in ],
	[if test $withval != yes; then
	   	jpeginfo_cv_libjpeg=$withval
	fi
	if test -d "${jpeginfo_cv_libjpeg}/lib"; then
		LDFLAGS="-L${jpeginfo_cv_libjpeg}/lib $LDFLAGS"
		CPPFLAGS="$CPPFLAGS -I${jpeginfo_cv_libjpeg}/include"	
	else
		LDFLAGS="-L${jpeginfo_cv_libjpeg} $LDFLAGS"
		CPPFLAGS="$CPPFLAGS -I${jpeginfo_cv_libjpeg}"	
	fi])

# Checks for libraries.
AC_CHECK_LIB(jpeg, ,[
echo "Cannot find libjpeg."
exit 1
])

# Checks for header files.
AC_CHECK_HEADERS([limits.h stdlib.h string.h getopt.h stdio.h sys/stats.h dirent.h])
AC_CHECK_HEADERS(jpeglib.h,,[
echo "Cannot find jpeglib.h."
exit 1
])

# Checks for typedefs, structures, and compiler characteristics.
AC_TYPE_OFF_T

# Checks for library functions.
AC_FUNC_MALLOC
AC_CHECK_FUNCS([mkdir strrchr strtol])

AC_OUTPUT

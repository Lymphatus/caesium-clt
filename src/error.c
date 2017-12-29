#include <stdio.h>
#include <caesium.h>
#include <stdlib.h>

#include "error.h"

void display_error(error_level level, int code)
{
	char *error_level = ((level) ? "[WARNING]" : "[ERROR]");
	fprintf(stderr, "%s %d: %s\n",
			error_level,
			code,
			get_error_message(code));
	if (level == ERROR) {
		exit(-code);
	}
}

const char *get_error_message(int code)
{
	switch (code) {
		//Generic errors
		case 1:
			return "Invalid quality value. Must be between [0-100].";
		case 2:
			return "Unrecognized option.";
		case 3:
			return "Empty input folder.";
		case 4:
			return "Cannot keep folder structure providing multiple input files.";
		case 5:
			return "Cannot create output folder.";
		case 6:
			return "Cannot check if is a directory.";
		case 7:
			return "Cannot calculate file size";
		case 8:
			return "Input folder provided. Skipping all other inputs.";
		case 9:
			return "Input files provided. Cannot mix them with a folder.";
		case 10:
			return "-R has no effects on files.";
		case 11:
			return "-S has no effect without -R.";
		case 12:
			return "Cannot set output folder inside the input one";

		default:
			return "Unrecognized error.";
	}
}
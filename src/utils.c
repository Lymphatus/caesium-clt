#include <stdio.h>
#include <stdlib.h>
#include <errno.h>
#include "utils.h"

void print_help()
{
	fprintf(stdout,
			"CaesiumCLT - Caesium Command Line Tools\n\n"
					"Usage: caesiumclt [OPTIONS] INPUT...\n"
					"Compress your pictures up to 90%% without visible quality loss.\n\n"

					"Options:\n"
					"\t-q, --quality\t\t\tset output file quality between [1-100], JPEG only\n"
					"\t-e, --exif\t\t\t\tkeeps EXIF info during compression\n"
					"\t-o, --output\t\t\toutput folder\n"
					"\t-l, --lossless\t\t\tuse lossless optimization\n"
					"\t-R, --recursive\t\t\tif input is a folder, scan subfolders too\n"
					//TODO Remove this warning
					"\t-S, --keep-structure\tkeep the folder structure [Not active yet]\n"
					"\t-h, --help\t\t\t\tdisplay this help and exit\n"
					"\t-v, --version\t\t\toutput version information and exit\n\n");
	exit(EXIT_SUCCESS);
}
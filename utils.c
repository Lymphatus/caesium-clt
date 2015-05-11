#include <stdlib.h>
#include <limits.h>
#include <stdio.h>
#include <errno.h>
#include <setjmp.h>
#include <stdio.h>
#include <jpeglib.h>
#include <string.h>

#include "utils.h"

cclt_compress_parameters initialize_compression_parameters() {
	cclt_compress_parameters par;
	
	par.quality = 0;
	par.width = 0;
	par.height = 0;
	par.smoothing_factor = 50;
	par.scaling_factor = 100;
	//par.color_space = NULL; TODO Must set?
	//par.dct_method = NULL; TODO Must set?
	par.output_folder = NULL;
	par.exif_copy = 0;
	par.lossless = 0;
	par.input_files_count = 0;
	//par.input_files = (char**) malloc (55 * sizeof(char));
	return par;
}

int string_to_int(char* in_string) {
	int value = 0;
	char* endptr;
	errno = 0; //Error checking

	value = strtol(in_string, &endptr, 0); //Convert the string
	
	//Check errors
	if ((errno == ERANGE && (value == LONG_MAX || value == LONG_MIN))
            || (errno != 0 && value == 0)) {
        perror("strtol");
        exit(-8);
    }

   if (endptr == in_string) {
        fprintf(stderr, "Parse error: No digits were found for -q option. Aborting.\n");
        exit(-7);
    }
	
	return value;
}

void print_help() {
	fprintf(stdout,
		"Usage: cclt [OPTION] INPUT...\n"
		"Compress your pictures up to 90% without visible quality loss.\n\n"

		"Options:\n"
			"\t-q\tset output file quality between [1-100], ignored for non-JPEGs\n"
			"\t-e\tkeeps EXIF info during compression\n"
			"\t-o\tcompress to custom folder\n"
			"\t-l\tuse lossless optimization\n"
			"\t-s\tscale to value, expressed as percentage (e.g. 20%)\n"
			"\t-R\tif input is a folder, scan subfolders too\n"
			"\t-h\tdisplay this help and exit\n"
			"\t-v\toutput version information and exit\n");
	exit(0);
}

void print_progress(int current, int max, char* message) {
	fprintf(stdout, "\e[?25l");
	fprintf(stdout, "\r%s[%d\%]", message, current * 100 / max);
	if (current == max) {
		fprintf(stdout, "\e[?25h\n");
	}
}

char* get_filename_with_extension(char* full_path) {
	char* dest;
		
	dest = strrchr(full_path, '/') + 1;
	
	printf("%s\n", dest);
	
	return dest;
}

#ifndef CCTL_UTILS
#define CCTL_UTILS

#include <jpeglib.h>

typedef struct cclt_compress_parameters {
	int quality;
	int width;
	int height;
	int smoothing_factor;
	int scaling_factor;
	char* output_folder;
	J_COLOR_SPACE color_space;
	J_DCT_METHOD dct_method;
	int exif_copy;
	int lossless;
	char** input_files;
	int input_files_count;
} cclt_compress_parameters;

cclt_compress_parameters initialize_compression_parameters();

int string_to_int(char* in_string);
void print_help();
void print_progress(int current, int max, char* message);

#endif

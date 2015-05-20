#ifndef CCLT_UTILS
#define CCLT_UTILS

#include <jpeglib.h>
#include <turbojpeg.h>
#include <sys/types.h>

#define APP_VERSION "1.9.9 BETA"
#define BUILD 20150519

typedef struct cclt_compress_parameters {
	int quality;
	int width;
	int height;
	int scaling_factor;
	char* output_folder;
	int color_space;
	int dct_method;
	int exif_copy;
	int lossless;
	char** input_files;
	int input_files_count;
	enum TJSAMP subsample;
} cclt_compress_parameters;

cclt_compress_parameters initialize_compression_parameters();

int string_to_int(char* in_string);
void print_help();
void print_progress(int current, int max, char* message);
char* get_filename_with_extension(char* full_path);
void jcopy_markers_execute (j_decompress_ptr srcinfo, j_compress_ptr dstinfo);
int mkpath(const char *pathname, mode_t mode);
cclt_compress_parameters parse_arguments(int argc, char* argv[]);
void cclt_compress_routine(char* input, char* output,cclt_compress_parameters* pars);

#endif

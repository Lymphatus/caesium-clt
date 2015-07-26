#ifndef CCLT_UTILS
#define CCLT_UTILS

#include <jpeglib.h>
#include <turbojpeg.h>
#include <sys/types.h>

#define APP_VERSION "1.9.9 BETA"
#define BUILD 20150723

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
	int recursive;
} cclt_compress_parameters;

enum image_type {
	JPEG,
	PNG,
	GIF,
	UNKN,
};

int string_to_int(char* in_string);
void print_help();
void print_progress(int current, int max, char* message);
int mkpath(const char *pathname, mode_t mode);
enum image_type detect_image_type(char* path);

#endif

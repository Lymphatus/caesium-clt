#ifndef CCLT_UTILS
#define CCLT_UTILS

#include <jpeglib.h>
#include <turbojpeg.h>
#include <sys/types.h>

#define APP_VERSION "0.9.1-beta"
#define BUILD 20160116

typedef struct cclt_compress_parameters {
	int quality;
	int width;
	int height;
	char* output_folder;
	int color_space;
	int dct_method;
	int exif_copy;
	int lossless;
	char** input_files;
	int input_files_count;
	enum TJSAMP subsample;
	int recursive;
	int structure;
} cclt_compress_parameters;

enum image_type {
	JPEG,
	PNG,
	UNKN,
};

int string_to_int(char* in_string);
void print_help();
int mkpath(const char *pathname, mode_t mode);
enum image_type detect_image_type(char* path);
int is_directory(const char *file_path);
int scan_folder(char** fileList, char* basedir, int recur);
char* get_human_size(long size);

#endif

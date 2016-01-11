#ifndef CCLT_UTILS
#define CCLT_UTILS

#include <jpeglib.h>
#include <turbojpeg.h>
#include <sys/types.h>

#define APP_VERSION "0.9.1-beta"
#define BUILD 20150921

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
void print_progress(int current, int max, char* message);
int mkpath(const char *pathname, mode_t mode);
enum image_type detect_image_type(char* path);
int isDirectory(const char *file_path);
char** scan_folder(char* basedir, int* n, int recur);
char* get_human_size(long size);

#endif

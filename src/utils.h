#ifndef CCLT_UTILS
#define CCLT_UTILS

#include <jpeglib.h>
#include <turbojpeg.h>
#include <sys/types.h>
#include <stdbool.h>
#include "ccltypes.h"

#define APP_VERSION "0.9.1"
#define BUILD 20160922

int string_to_int(char* in_string);
void print_help();
int mkpath(const char* pathname, mode_t mode);
enum image_type detect_image_type(char* path);
int is_directory(const char* file_path);
void scan_folder(cclt_parameters* parameters, char* basedir, int recur);
char* get_human_size(long size);

#endif

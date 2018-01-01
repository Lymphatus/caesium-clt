//
// Created by Matteo Paonessa on 15/12/16.
//

#ifndef CAESIUM_CLT_UTILS_H
#define CAESIUM_CLT_UTILS_H

#include "helper.h"

void print_help();

bool is_directory(const char *path);

int scan_folder(const char *directory, cclt_options *options, bool recursive);

char *get_filename(char * full_path);

off_t get_file_size(const char *path);

char* get_human_size(off_t size);

int mkpath(const char *pathname);

bool file_exists(const char* file_path);

int strndx(const char* string, const char search);

double parse_scale_factor(const char* factor_string);

#ifdef _WIN32
char *str_replace(char *orig, char *rep, char *with);
char *strsep(char **stringp, const char *delim);
#endif


#endif //CAESIUM_CLT_UTILS_H

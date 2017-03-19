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

char *str_replace(char *orig, char *rep, char *with);

#ifdef _WIN32
char *strsep(char **stringp, const char *delim);
#endif


#endif //CAESIUM_CLT_UTILS_H

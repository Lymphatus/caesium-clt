//
// Created by Matteo Paonessa on 15/12/16.
//

#ifndef CAESIUM_CLT_UTILS_H
#define CAESIUM_CLT_UTILS_H

#include "helper.h"

void print_help();

bool is_directory(const char *path);

int scan_folder(const char *directory, cclt_options *options, bool recursive);

int mkpath(const char *pathname, mode_t mode);

char *get_filename(char * full_path);

off_t get_file_size(const char *path);

char* get_human_size(off_t size);


#endif //CAESIUM_CLT_UTILS_H

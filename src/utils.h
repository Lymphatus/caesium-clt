//
// Created by Matteo Paonessa on 15/12/16.
//

#ifndef CAESIUM_CLT_UTILS_H
#define CAESIUM_CLT_UTILS_H

#include "helper.h"

#define APP_VERSION_STRING "0.10.0"
#define APP_VERSION_NUMBER 0100
#define BUILD 20161215

void print_help();

int is_directory(const char *path);

int scan_folder(const char *directory, cclt_options *options, bool recursive);

int mkpath(const char *pathname, mode_t mode);

char *get_filename(char * full_path);


#endif //CAESIUM_CLT_UTILS_H

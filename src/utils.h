/*
 *
 * Copyright 2019 Matteo Paonessa
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */

#ifndef CAESIUM_CLT_UTILS_H
#define CAESIUM_CLT_UTILS_H

#include "helper.h"

void print_help();

bool is_directory(const char *path);

void scan_folder(const char *directory, cclt_options *options, bool recursive);

char *get_filename(char * full_path);

off_t get_file_size(const char *path);

char* get_human_size(off_t size);

int mkpath(const char *pathname);

bool file_exists(const char* file_path);

int strndx(const char* string, char search);

overwrite_policy parse_overwrite_policy(const char* overwrite_string);

void print_to_console(FILE* buffer, int verbose, const char* format, ...);

int parse_png_quality(int quality);

#ifdef _WIN32
char *str_replace(char *orig, char *rep, char *with);
char *strsep(char **stringp, const char *delim);
#endif


#endif //CAESIUM_CLT_UTILS_H

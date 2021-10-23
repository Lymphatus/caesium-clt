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

#ifndef CAESIUM_CLT_HELPER_H
#define CAESIUM_CLT_HELPER_H

#ifdef _WIN32
#define MAX_PATH_SIZE _MAX_PATH
#else
#include <limits.h>
#include <stdbool.h>

#define MAX_PATH_SIZE PATH_MAX
#endif

typedef enum overwrite_policy {
	none,
	prompt,
	bigger,
	all
} overwrite_policy;

typedef struct C_CSParameters {
    bool keep_metadata;
    unsigned int jpeg_quality;
    unsigned int png_level;
    bool png_force_zopfli;
    unsigned int gif_quality;
    unsigned int webp_quality;
    bool optimize;
} C_CSParameters;

extern bool c_compress(const char *i, const char *o, struct C_CSParameters params);

typedef struct cclt_options
{
	char **input_files;
	char input_folder[MAX_PATH_SIZE];
	char output_folder[MAX_PATH_SIZE];
	bool recursive;
	bool keep_structure;
	int files_count;
	off_t input_total_size;
	off_t output_total_size;
	bool dry_run;
	overwrite_policy overwrite;
} cclt_options;



cclt_options parse_arguments(char *argv[], C_CSParameters *options);

int start_compression(cclt_options *options, struct C_CSParameters parameters);

#endif //CAESIUM_CLT_HELPER_H

/*
 *
 * Copyright 2018 Matteo Paonessa
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

#include <caesium.h>

#ifdef _WIN32
#define MAX_PATH_SIZE _MAX_PATH
#else
#include <limits.h>
#define MAX_PATH_SIZE PATH_MAX
#endif

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
} cclt_options;

cclt_options parse_arguments(char *argv[], cs_image_pars *options);

int start_compression(cclt_options *options, cs_image_pars *parameters);

#endif //CAESIUM_CLT_HELPER_H

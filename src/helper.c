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

#include <stdlib.h>
#include <stdio.h>
#include <string.h>

#ifdef _WIN32
#include <direct.h>
#endif

#include "helper.h"
#include "vendor/optparse.h"
#include "utils.h"
#include "config.h"
#include "error.h"

cclt_options parse_arguments(char **argv, cs_image_pars *options)
{
	struct optparse opts;
	//Initialize application options
	cclt_options parameters = {NULL, "", "", false, false, 0, 0, 0, false};

	//Parse command line args
	optparse_init(&opts, argv);
	struct optparse_long longopts[] = {
			{"quality",        'q', OPTPARSE_REQUIRED},
			{"exif",           'e', OPTPARSE_NONE},
			{"output",         'o', OPTPARSE_REQUIRED},
			{"scale",		   's', OPTPARSE_REQUIRED},
			{"recursive",      'R', OPTPARSE_NONE},
			{"keep-structure", 'S', OPTPARSE_NONE},
			{"dry-run",        'd', OPTPARSE_NONE},
			{"version",        'v', OPTPARSE_NONE},
			{"help",           'h', OPTPARSE_NONE},
			{0}
	};

	int option;
	while ((option = optparse_long(&opts, longopts, NULL)) != -1) {
		switch (option) {
			case 'q':
				options->jpeg.quality = (int) strtol(opts.optarg, (char **) NULL, 10);
				if (options->jpeg.quality < 0 || options->jpeg.quality > 100) {
					display_error(ERROR, 1);
				}
				break;
			case 'e':
				options->jpeg.exif_copy = true;
			case 'o':
				if (opts.optarg[0] == '~') {
					snprintf(parameters.output_folder, strlen(opts.optarg) + 1, "%s", opts.optarg);
				} else {
#ifdef _WIN32
					_fullpath(parameters.output_folder, opts.optarg, MAX_PATH);
#else
					realpath(opts.optarg, parameters.output_folder);
#endif
				}
				int pathlen = strlen(parameters.output_folder);
				if (parameters.output_folder[pathlen - 1] != '/' &&
					parameters.output_folder[pathlen - 1] != '\\') {
					// append the extra slash/backslash
#ifdef _WIN32
					snprintf(parameters.output_folder+pathlen, 2, "\\");
#else
					snprintf(parameters.output_folder+pathlen, 2, "/");
#endif
				}
				break;
			case 's':
				options->jpeg.scale_factor = options->png.scale_factor = parse_scale_factor(opts.optarg);
				break;
			case 'R':
				parameters.recursive = true;
				break;
			case 'S':
				parameters.keep_structure = true;
				break;
			case 'd':
				parameters.dry_run = true;
				break;
			case 'v':
				fprintf(stdout, "%d.%d.%d\n", VERSION_MAJOR, VERSION_MINOR, VERSION_PATCH);
				exit(EXIT_SUCCESS);
			case 'h':
				print_help();
				break;
			case '?':
			default:
				fprintf(stderr, "%s: %s\n", argv[0], opts.errmsg);
				display_error(ERROR, 2);
		}
	}
	//Remaining arguments
	char *arg;
	bool files_flag = false, folders_flag = false;
	char resolved_path[MAX_PATH_SIZE];

	fprintf(stdout, "%s\n", "Collecting files...");

	while ((arg = optparse_arg(&opts))) {
		if (folders_flag) {
			display_error(WARNING, 8);
			break;
		}

		//Check if it's a directory and add its content
		if (arg[0] == '~' && is_directory(arg)) {
			if (arg[strlen(arg) - 1] == '/' || arg[strlen(arg) - 1] == '\\') {
				snprintf(resolved_path, strlen(arg), "%s", arg);
			} else {
#ifdef _WIN32
				snprintf(resolved_path, strlen(arg) + 1, "%s\\", arg);
#else
				snprintf(resolved_path, strlen(arg) + 1, "%s/", arg);
#endif
			}
		} else {
#ifdef _WIN32
			_fullpath(resolved_path, arg, MAX_PATH);
#else
			realpath(arg, resolved_path);
#endif
		}

		if (is_directory(resolved_path)) {
			if (!files_flag) {
				folders_flag = true;
				size_t len = strlen(resolved_path);
				if (resolved_path[len - 1] != '/' && resolved_path[strlen(resolved_path) - 1] != '\\') {
#ifdef _WIN32
					resolved_path[len] = '\\';
#else
					resolved_path[len] = '/';
#endif
					resolved_path[len + 1] = '\0';
				}

				snprintf(parameters.input_folder, strlen(resolved_path) + 1, "%s", resolved_path);
				int count = 0;
				count = scan_folder(resolved_path, &parameters, parameters.recursive);
				if (count == 0) {
					display_error(WARNING, 3);
				}
			} else {
				display_error(WARNING, 9);
			}
		} else {
			files_flag = true;
			parameters.input_files = realloc(parameters.input_files, (parameters.files_count + 1) * sizeof(char *));
			parameters.input_files[parameters.files_count] = malloc((strlen(arg) + 1) * sizeof(char));
			snprintf(parameters.input_files[parameters.files_count], strlen(arg) + 1, "%s", arg);
			parameters.files_count++;
		}
	}

	//Check if the output folder is a subfolder of the input to avoid infinite loops
	//but just if the -R option is set
	//However, if the folders are the same, we can let it go as it will overwrite the files
	if (folders_flag) {
		if (strstr(parameters.output_folder, parameters.input_folder) != NULL
			&& strcmp(parameters.output_folder, parameters.input_folder) != 0
			&& parameters.recursive) {
			display_error(ERROR, 12);
		}
	}

	//-R and -S set warnings
	if (parameters.recursive && !folders_flag) {
		display_error(WARNING, 10);
		parameters.recursive = false;
	}
	if (!parameters.recursive && parameters.keep_structure) {
		display_error(WARNING, 11);
		parameters.keep_structure = false;
	}
	//If there're files and folders, we cannot keep the structure
	if (parameters.keep_structure && (!folders_flag && parameters.files_count > 1)) {
		display_error(WARNING, 4);
		parameters.keep_structure = false;
	}
	return parameters;
}

int start_compression(cclt_options *options, cs_image_pars *parameters)
{
	int compressed_files = 0;
	off_t input_file_size = 0;
	off_t output_file_size = 0;
	//Create the output folder if does not exists
	if (mkpath(options->output_folder) == -1) {
		display_error(ERROR, 5);
	}

	for (int i = 0; i < options->files_count; i++) {
		char *filename = get_filename(options->input_files[i]);
		char *output_full_path = NULL;
		char *original_output_full_path = NULL;
		bool overwriting = false;
		off_t file_size = 0;
		//If we don't need to keep the structure, we put all the files in one folder by just the filename
		if (!options->keep_structure) {
			output_full_path = malloc((strlen(filename) + strlen(options->output_folder) + 1) * sizeof(char));
			snprintf(output_full_path, (strlen(filename) + strlen(options->output_folder) + 1), "%s%s",
					 options->output_folder, filename);
		} else {
			/*
			 * Otherwise, we need to compute the whole directory structure
			 * We are sure we have a folder only as input, so that's the root
			 * Just compute the subfolders without the filename, make them and append the filename
			 * A piece of cake <3
			*/

			size_t index = strspn(options->input_folder, options->input_files[i]);
			size_t size = strlen(options->input_files[i]) - index - strlen(filename);
			char output_full_folder[strlen(options->output_folder) + size + 1];

			snprintf(output_full_folder, strlen(options->output_folder) + size + 1, "%s%s",
					 options->output_folder, &options->input_files[i][index]);
			output_full_path = malloc((strlen(output_full_folder) + strlen(filename) + 1) * sizeof(char));
			snprintf(output_full_path, strlen(output_full_folder) + strlen(filename) + 1, "%s%s",
					 output_full_folder,
					 filename);

			mkpath(output_full_folder);
		}

		fprintf(stdout, "(%d/%d) %s -> %s\n",
				i + 1,
				options->files_count,
				filename,
				output_full_path);

		//If the file already exist, create a temporary file
		if (file_exists(output_full_path)) {
			original_output_full_path = strdup(output_full_path);
			output_full_path = realloc(output_full_path, (strlen(output_full_path) + 4) * sizeof(char));
			snprintf(output_full_path, (strlen(original_output_full_path) + 4), "%s.cs", original_output_full_path);
			overwriting = true;
		}

		file_size = get_file_size(options->input_files[i]);
		if (file_size == 0) {
			//We could not open the file
			continue;
		}
		input_file_size = file_size;
		options->input_total_size += input_file_size;

		//Prevent compression if running in dry mode
		if (!options->dry_run) {
			if (cs_compress(options->input_files[i], output_full_path, parameters)) {
				compressed_files++;
				output_file_size = get_file_size(output_full_path);
				options->output_total_size += output_file_size;

				char *human_input_size = get_human_size(input_file_size);
				char *human_output_size = get_human_size(output_file_size);

				fprintf(stdout, "%s -> %s [%.2f%%]\n",
						human_input_size,
						human_output_size,
						((float) output_file_size - input_file_size) * 100 / input_file_size);
			} else {
				options->input_total_size -= get_file_size(options->input_files[i]);
			}
		}

		//Rename if we were overwriting
		if (overwriting && !options->dry_run) {
			rename(output_full_path, original_output_full_path);
		}

		free(original_output_full_path);
		free(output_full_path);
	}

	return compressed_files;
}



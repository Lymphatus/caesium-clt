#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include "helper.h"
#include "optparse.h"
#include "utils.h"
#include "config.h"
#include "error.h"

cclt_options parse_arguments(char **argv, cs_image_pars *options)
{
	struct optparse opts;
	//Initialize application options
	cclt_options parameters = {NULL, NULL, false, false, 0, 0, 0};

	//Parse command line args
	optparse_init(&opts, argv);
	struct optparse_long longopts[] = {
			{"quality",        'q', OPTPARSE_REQUIRED},
			{"exif",           'e', OPTPARSE_NONE},
			{"output",         'o', OPTPARSE_REQUIRED},
			{"recursive",      'R', OPTPARSE_NONE},
			{"keep-structure", 'S', OPTPARSE_NONE},
			{"version",        'v', OPTPARSE_NONE},
			{"help",           'h', OPTPARSE_NONE},
			{0}
	};
	int option;

	while ((option = optparse_long(&opts, longopts, NULL)) != -1) {
		switch (option) {
			case 'q':
				options->jpeg.quality = atoi(opts.optarg);
				if (options->jpeg.quality < 0 || options->jpeg.quality > 100) {
					display_error(ERROR, 1);
				}
				break;
			case 'e':
				options->jpeg.exif_copy = true;
				break;
			case 'o':
				if (opts.optarg[strlen(opts.optarg) - 1] == '/' || opts.optarg[strlen(opts.optarg) - 1] == '\\') {
					parameters.output_folder = malloc((strlen(opts.optarg) + 1) * sizeof(char));
					snprintf(parameters.output_folder, strlen(opts.optarg) + 1, "%s", opts.optarg);
				} else {
					parameters.output_folder = malloc((strlen(opts.optarg) + 2) * sizeof(char));
#ifdef _WIN32
					snprintf(parameters.output_folder, strlen(opts.optarg) + 2, "%s\\", opts.optarg);
#else
					snprintf(parameters.output_folder, strlen(opts.optarg) + 2, "%s/", opts.optarg);
#endif
				}
				break;
			case 'R':
				parameters.recursive = true;
				break;
			case 'S':
				parameters.keep_structure = true;
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
	while ((arg = optparse_arg(&opts))) {
		if (folders_flag) {
			display_error(WARNING, 8);
			continue;
		}
		//Check if it's a directory and add its content
		if (is_directory(arg)) {
			if (!files_flag) {
				folders_flag = true;
				parameters.input_folder = strdup(arg);
				int count = 0;
				count = scan_folder(arg, &parameters, parameters.recursive);
				if (count == 0) {
					display_error(WARNING, 3);
				}
			} else {
				display_error(WARNING, 9);
			}
		} else {
			files_flag = true;
			parameters.input_folder = NULL;
			parameters.input_files = realloc(parameters.input_files, (parameters.files_count + 1) * sizeof(char *));
			parameters.input_files[parameters.files_count] = malloc((strlen(arg) + 1) * sizeof(char));
			snprintf(parameters.input_files[parameters.files_count], strlen(arg) + 1, "%s", arg);
			parameters.files_count++;
		}
	}

	//Check if the output folder is a subfolder of the input to avoid infinite loops
	if (folders_flag) {
		if (strstr(parameters.output_folder, parameters.input_folder) != NULL) {
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
		char *output_full_path;
		//If we don't need to keep the structure, we put all the files in one folder by just the filename
		if (!options->keep_structure) {
			output_full_path = malloc((strlen(filename) + strlen(options->output_folder) + 1) * sizeof(char));
			snprintf(output_full_path, (strlen(filename) + strlen(options->output_folder) + 1), "%s%s",
					 options->output_folder, filename);
		} else {
			/*
			 * Otherwise, we nee to compute the whole directory structure
			 * We are sure we have a folder only as input, so that's the root
			 * Just compute the subfolders without the filename, make them and append the filename
			 * A piece of cake <3
			*/

			size_t index = strspn(options->input_folder, options->input_files[i]) + 1;
			size_t size = strlen(options->input_files[i]) - index - strlen(filename);
			char output_full_folder[strlen(options->output_folder) + size + 1];
			snprintf(output_full_folder, strlen(options->output_folder) + size + 1, "%s%s", options->output_folder, &options->input_files[i][index]);
			output_full_path = malloc((strlen(output_full_folder) + strlen(filename) + 1) * sizeof(char));
			snprintf(output_full_path, strlen(output_full_folder) + strlen(filename) + 1, "%s%s", output_full_folder, filename);
			mkpath(output_full_folder);
		}

		fprintf(stdout, "(%d/%d) %s -> %s\n",
				i + 1,
				options->files_count,
				filename,
				output_full_path);

		input_file_size = get_file_size(options->input_files[i]);
		options->input_total_size += input_file_size;
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

		free(output_full_path);
	}

	return compressed_files;
}



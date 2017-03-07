#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include "helper.h"
#include "optparse.h"
#include "utils.h"
#include "config.h"

cclt_options parse_arguments(char **argv, cs_image_pars *options)
{
	struct optparse opts;
	//Initialize application options
	cclt_options result = {NULL, NULL, false, false, 0, 0, 0};

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
					//TODO Trigger a error
					exit(EXIT_FAILURE);
				}
				break;
			case 'e':
				options->jpeg.exif_copy = true;
				break;
			case 'o':
				if (opts.optarg[strlen(opts.optarg) - 1] == '/') {
					result.output_folder = malloc((strlen(opts.optarg) + 1) * sizeof(char));
					snprintf(result.output_folder, strlen(opts.optarg) + 1, "%s", opts.optarg);
				} else {
					result.output_folder = malloc((strlen(opts.optarg) + 2) * sizeof(char));
					snprintf(result.output_folder, strlen(opts.optarg) + 2, "%s/", opts.optarg);
				}
				break;
			case 'R':
				result.recursive = true;
				break;
			case 'S':
				result.keep_structure = true;
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
				exit(EXIT_FAILURE);
		}
	}

	//Remaining arguments
	char *arg;
	bool files_flag = false, folders_flag = false;
	while ((arg = optparse_arg(&opts))) {
		//Check if it's a directory and add its content
		if (is_directory(arg)) {
			int count = 0;
			count = scan_folder(arg, &result, result.recursive);
			if (count == 0) {
				//TODO Trigger a warning
			}

		} else {
			result.input_files = realloc(result.input_files, (result.files_count + 1) * sizeof(char *));
			result.input_files[result.files_count] = malloc((strlen(arg) + 1) * sizeof(char));
			snprintf(result.input_files[result.files_count], strlen(arg) + 1, "%s", arg);
			result.files_count++;
		}
	}

	//If there're files and folders, we cannot keep the structure
	//TODO Trigger a warning
	result.keep_structure = !(files_flag && folders_flag);

	return result;
}

int start_compression(cclt_options *options, cs_image_pars *parameters)
{
	int compressed_files = 0;
	off_t input_file_size = 0;
	off_t output_file_size = 0;
	//TODO Support folder structure

	//Create the output folder if does not exists
	if (mkpath(options->output_folder, 0777) == -1) {
		//TODO Error
		exit(EXIT_FAILURE);
	}

	for (int i = 0; i < options->files_count; i++) {
		char *filename = get_filename(options->input_files[i]);
		char *output_full_path = malloc((strlen(filename) + strlen(options->output_folder) + 1) * sizeof(char));
		snprintf(output_full_path, (strlen(filename) + strlen(options->output_folder) + 1), "%s%s", options->output_folder, filename);

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

			fprintf(stdout, "%s -> %s [%.2f%%]\n",
					get_human_size(input_file_size),
					get_human_size(output_file_size),
					((float) output_file_size - input_file_size) * 100 / input_file_size);
		} else {
			options->input_total_size -= get_file_size(options->input_files[i]);
		}

		free(output_full_path);
	}

	return compressed_files;
}



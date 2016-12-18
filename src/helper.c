#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include "helper.h"
#include "optparse.h"
#include "utils.h"

void initialize_jpeg_parameters(cs_image_pars *options)
{
	options->jpeg.quality = 0;
	options->jpeg.exif_copy = false;
	options->jpeg.dct_method = 2048;
	options->jpeg.width = 0;
	options->jpeg.height = 0;
}

void initialize_png_parameters(cs_image_pars *par)
{
	par->png.iterations = 10;
	par->png.iterations_large = 5;
	par->png.block_split_strategy = 4;
	par->png.lossy_8 = true;
	par->png.transparent = true;
	par->png.auto_filter_strategy = 1;
}

cs_image_pars initialize_parameters()
{
	cs_image_pars options;

	initialize_jpeg_parameters(&options);
	initialize_png_parameters(&options);

	return options;
}

cclt_options parse_arguments(char **argv, cs_image_pars *options)
{
	struct optparse opts;
	//Initialize application options
	cclt_options result = {NULL, NULL, false, false, 0};

	//Parse command line args
	optparse_init(&opts, argv);
	struct optparse_long longopts[] = {
			{"quality", 'q', OPTPARSE_REQUIRED},
			{"exif", 'e', OPTPARSE_NONE},
			{"output", 'o', OPTPARSE_REQUIRED},
			{"recursive", 'R', OPTPARSE_NONE},
			{"keep-structure", 'S', OPTPARSE_NONE},
			{"version", 'v', OPTPARSE_NONE},
			{"help", 'h', OPTPARSE_NONE},
			{0}
	};
	int option;

	while ((option = optparse_long(&opts, longopts, NULL)) != -1) {
		switch (option) {
			case 'q':
				options->jpeg.quality = atoi(opts.optarg);
				break;
			case 'e':
				options->jpeg.exif_copy = true;
				break;
			case 'o':
				result.output_folder = malloc((strlen(opts.optarg) + 1) * sizeof(char));
				strncpy(result.output_folder, opts.optarg, strlen(opts.optarg) + 1);
				break;
			case 'R':
				result.recursive =  true;
				break;
			case 'S':
				result.keep_structure = true;
				break;
			case 'v':
				fprintf(stdout, "%s-%d\n", APP_VERSION_STRING, BUILD);
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
		//TODO Check if there's a folder and change behaviour accordingly
		//Check if it's a directory and add its content
		if (is_directory(arg)) {
			//NOTE Scanning a folder with this function does not check if we are actually getting images
			//The actual check is performed by the library
			int count = 0;
			count = scan_folder(arg, &result, result.recursive);
			if (count == 0) {
				//TODO Trigger a warning
			}

		} else {
			result.input_files = realloc(result.input_files, (result.files_count + 1) * sizeof(char*));
			result.input_files[result.files_count] = malloc((strlen(arg) + 1) * sizeof(char));
			//TODO Replace with strdup for alloc
			strncpy(result.input_files[result.files_count], arg, strlen(opts.optarg) + 1);
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
	//TODO Support folder structure
	int status = 0;

	//Create the output folder if does not exists
	if (mkpath(options->output_folder, 0777) == -1) {
		//TODO Error
		exit(EXIT_FAILURE);
	}

	for (int i = 0; i < options->files_count; i++) {
		//TODO remove unnecessary "/"s
		char *filename = get_filename(options->input_files[i]);
		char *output_full_path = malloc((strlen(filename) + strlen(options->output_folder) + 2) * sizeof(char));
		strncpy(output_full_path, options->output_folder, strlen(options->output_folder));
		strcat(output_full_path, "/");
		strcat(output_full_path, filename);


		fprintf(stdout,
			"Compressing %s in %s\n", filename, output_full_path);
		cs_compress(options->input_files[i], output_full_path, parameters);

	}

	return status;
}



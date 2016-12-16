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

		} else {
			result.input_files = realloc(result.input_files, (result.files_count + 1) * sizeof(char*));
			result.input_files[result.files_count] = malloc((strlen(arg) + 1) * sizeof(char));
			strncpy(result.input_files[result.files_count], arg, strlen(opts.optarg) + 1);
			result.files_count++;
		}
		//If there're files and folders, we cannot keep the structure
		//TODO Trigger a warning
		result.keep_structure = !(files_flag && folders_flag);
	}

	return result;
}



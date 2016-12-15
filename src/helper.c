#include <stdlib.h>
#include <stdio.h>
#include "helper.h"
#include "optparse.h"
#include "utils.h"

void initialize_jpeg_parameters(cs_image_pars *options)
{
	options->jpeg.quality = 65;
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

cs_image_pars parse_arguments(int argc, char **argv)
{
	struct optparse options;
	//Initialize the default parameters
	cs_image_pars result = initialize_parameters();

	//Parse command line args
	optparse_init(&options, argv);
	struct optparse_long longopts[] = {
			{"quality", 'q', OPTPARSE_REQUIRED},
			{"exif", 'e', OPTPARSE_NONE},
			{"output", 'o', OPTPARSE_REQUIRED},
			{"lossless", 'l', OPTPARSE_NONE},
			{"recursive", 'R', OPTPARSE_NONE},
			{"keep-structure", 'S', OPTPARSE_NONE},
			{"version", 'v', OPTPARSE_NONE},
			{"help", 'h', OPTPARSE_NONE},
			{0}
	};
	int option;

	while ((option = optparse_long(&options, longopts, NULL)) != -1) {
		switch (option) {
			case 'q':
				result.jpeg.quality = atoi(options.optarg);
				break;
			case 'e':
				result.jpeg.exif_copy = true;
				break;
			case 'o':
				//TODO General args
				break;
			case 'l':
				result.jpeg.quality = 0;
				break;
			case 'R':
				//TODO General args
				break;
			case 'S':
				//TODO General args
				break;
			case 'v':
				fprintf(stdout,
						"%s-%d\n", APP_VERSION_STRING, BUILD);
				exit(EXIT_SUCCESS);
			case 'h':
				print_help();
				break;
			case '?':
			default:
				fprintf(stderr, "%s: %s\n", argv[0], options.errmsg);
				exit(EXIT_FAILURE);
		}
	}

	/* Print remaining arguments. */
	char *arg;
	while ((arg = optparse_arg(&options)))
		printf("%s\n", arg);
	return result;
}



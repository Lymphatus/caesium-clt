#include <setjmp.h>
#include <stdio.h>
#include <jpeglib.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h>
#include <turbojpeg.h>
#include <getopt.h>
#include <ctype.h>

#include "utils.h"
#include "jpeg.h"
#include "png.h"


cclt_compress_parameters initialize_compression_parameters() {
	cclt_compress_parameters par;
	
	par.quality = 0;
	par.width = 0;
	par.height = 0;
	par.scaling_factor = 100;
	par.color_space = TJPF_RGB;
	par.dct_method = TJFLAG_FASTDCT;
	par.output_folder = NULL;
	par.exif_copy = 0;
	par.lossless = 0;
	par.input_files_count = 0;
	par.recursive = 0;
	par.input_files = NULL;
	par.structure = 0;

	return par;
}

cclt_compress_parameters parse_arguments(int argc, char* argv[]) {
	
	//Initialize default params
	cclt_compress_parameters parameters = initialize_compression_parameters();
	int c;

	while (optind < argc) {
		if ((c = getopt (argc, argv, "q:velo:s:hR")) != -1) {
			switch (c) {
				case 'v':
					printf("CCLT - Caesium Command Line Tools - Version %s (Build: %d)\n", APP_VERSION, BUILD);
					exit(0);
					break;
				case '?':
					//TODO if -o not specified or empty, use current. Useful?
					if (optopt == 'q' || optopt == 'o' || optopt == 's') {
						fprintf (stderr, "Option -%c requires an argument.\n", optopt);
						//Arguments without values
						exit(-1);
					}
					else if (isprint(optopt))  {
						fprintf (stderr, "Unknown option `-%c'.\n", optopt);
					}
					else {
						fprintf (stderr, "Unknown option character `\\x%x'.\n", optopt);
					}
					break;
				case ':':
					fprintf(stderr, "Parameter expected.\n");
					break;
				case 'q':
					parameters.quality = string_to_int(optarg);
					break;
				case 'e':
					parameters.exif_copy = 1;
					break;
				case 'l':
					parameters.lossless = 1;
					break;
				case 'o':
					parameters.output_folder = optarg;
					break;
				case 's':
					parameters.scaling_factor = string_to_int(optarg);
					break;
				case 'h':
					print_help();
					break;
				case 'R':
					parameters.recursive = 1;
					break;
				case 'S':
					parameters.structure = 1;
					break;
				default:
					abort();
			}
		} else {
			int i = 0;
			if (isDirectory(argv[optind])) {
				//TODO Works but I'd like to pass the list and return the number of files instead
				parameters.input_files = scan_folder(argv[optind], &parameters.input_files_count, parameters.recursive);
				return parameters;
			}
			parameters.input_files = (char**) malloc ((argc - optind) * sizeof (char*));
			while (optind < argc) {
				parameters.input_files[i] = (char*) malloc (strlen(argv[optind]) * sizeof(char)); //TODO Necessary??
				parameters.input_files[i] = argv[optind];
				parameters.input_files_count = i + 1;
				optind++;
				i++;
			}
		}
	}

	return parameters;
}

int cclt_compress_routine(char* input, char* output, cclt_compress_parameters* pars) {
	enum image_type type = detect_image_type(input);
	if (type == JPEG && pars->lossless == 0) {
		cclt_jpeg_compress(output, cclt_jpeg_decompress(input, pars), pars);
		cclt_jpeg_optimize(output, output, pars->exif_copy, input);
	} else if (type == JPEG && pars->lossless != 0) {
		cclt_jpeg_optimize(input, output, pars->exif_copy, input);
	} else if (type == PNG) {
		cclt_png_optimize(input, output);
	} else if (type == GIF) {
		printf("GIF detected. Not implemented yet.\n");
	} else {
		printf("Unknown file type.\n");
		return -1;
	}
	return 0;
}
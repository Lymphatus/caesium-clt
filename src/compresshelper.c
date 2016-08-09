#include <setjmp.h>
#include <stdio.h>
#include <jpeglib.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h>
#include <turbojpeg.h>
#include <ctype.h>
#include <getopt.h>
#include <dirent.h>
#include <errno.h>
#include <sys/stat.h>
#include <libgen.h>
#include <stdbool.h>

#include "utils.h"
#include "jpeg.h"
#include "png.h"

void initialize_jpeg_parameters(cclt_parameters* par) {
	par->jpeg.quality = 0;
	par->jpeg.width = 0;
	par->jpeg.height = 0;
	par->jpeg.color_space = TJCS_RGB;
	par->jpeg.dct_method = TJFLAG_FASTDCT;
	par->jpeg.exif_copy = false;
	par->jpeg.lossless = false;
}

void initialize_png_parameters(cclt_parameters* par) {
	par->png.iterations = 10;
	par->png.iterations_large = 5;
	par->png.block_split_strategy = 4;
	par->png.lossy_8 = true;
	par->png.transparent = true;
	par->png.auto_filter_strategy = 1;
}

cclt_parameters initialize_compression_parameters() {
	cclt_parameters par;

	initialize_jpeg_parameters(&par);
	initialize_png_parameters(&par);

	par.output_folder = NULL;
	par.input_files_count = 0;
	par.recursive = 0;
	par.input_files = NULL;
	par.structure = false;

	return par;
}

void validate_parameters(cclt_parameters* pars) {
	//Either -l or -q must be set but not together
	if (!((pars->jpeg.lossless) ^ (pars->jpeg.quality > 0))) {
		//Both or none are set
		if (pars->jpeg.lossless && pars->jpeg.quality > 0) {
			fprintf(stderr, "[ERROR] -l option can't be used with -q. Either use one or the other.\n");
			exit(-1);
		} else if (!pars->jpeg.lossless && pars->jpeg.quality <= 0) {
			fprintf(stderr, "[ERROR] Either -l or -q must be set.\n");
			exit(-2);
		}
	} else {
		//One of them is set
		//If -q is set check it is within the 1-100 range
		if (!(pars->jpeg.quality >= 1 && pars->jpeg.quality <= 100) && pars->jpeg.lossless == 0) {
			fprintf(stderr, "[ERROR] Quality must be within a [1-100] range.\n");
			exit(-3);
		}
	}

	//Check if you set the input files
	if (pars->input_files_count == 0) {
		fprintf(stderr, "[ERROR] No input files.\n");
		exit(-9);
	}

	//Check if the output folder is set
	if (pars->output_folder == NULL) {
		fprintf(stderr, "[ERROR] No -o option pointing to the destination folder. Aborting.\n");
		exit(-4);
	}
}

cclt_parameters parse_arguments(int argc, char* argv[]) {

	//Initialize default params
	cclt_parameters parameters = initialize_compression_parameters();
	int c;

	while (optind < argc) {
		if ((c = getopt (argc, argv, "q:velo:s:hR")) != -1) {
			switch (c) {
				case 'v':
					printf("%s-%d\n", APP_VERSION, BUILD);
					exit(0);
					break;
				case '?':
					if (optopt == 'q' || optopt == 'o' || optopt == 's') {
						fprintf (stderr, "[ERROR] Option -%c requires an argument.\n", optopt);
						//Arguments without values
						exit(-1);
					}
					else if (isprint(optopt))  {
						fprintf (stderr, "[ERROR] Unknown option `-%c'.\n", optopt);
					}
					else {
						fprintf (stderr, "[ERROR] Unknown option character `\\x%x'.\n", optopt);
					}
					break;
				case ':':
					fprintf(stderr, "[ERROR] Parameter expected.\n");
					break;
				case 'q':
					parameters.jpeg.quality = string_to_int(optarg);
					break;
				case 'e':
					parameters.jpeg.exif_copy = true;
					break;
				case 'l':
					parameters.jpeg.lossless = true;
					break;
				case 'o':
					parameters.output_folder = optarg;
					break;
				case 'h':
					print_help();
					break;
				case 'R':
					parameters.recursive = true;
					break;
				case 'S':
					parameters.structure = true;
					break;
				default:
					abort();
			}
		} else {
			int i = 0;
			parameters.input_files = (char**) malloc ((argc - optind) * sizeof (char*));
			while (optind < argc) {
				if (is_directory(argv[optind])) {
					if (i != 0) {
						//TODO This error appears also if there a value to the -l parameter
						printf("[ERROR] Found folder along with input files. Aborting.\n");
						exit(-20);
					} else if (i == 0 && argc - optind > 1) {
						printf("[WARNING] Folder found, skipping all other inputs.\n");
					}
					scan_folder(&parameters, argv[optind], parameters.recursive);
					return parameters;
				} else {
					parameters.input_files[i] = (char*) malloc (strlen(argv[optind]) * sizeof(char)); //TODO Necessary??
					parameters.input_files[i] = argv[optind];
					i++;
					parameters.input_files_count = i;
					optind++;
				}
			}
		}
	}

	//Check if all parameters are poperly set
	validate_parameters(&parameters);

	return parameters;
}

int cclt_compress_routine(char* input, char* output, cclt_parameters* pars) {
	//Detect which image type are we compressing
	enum image_type type = detect_image_type(input);
	char* exif_orig = (char*) malloc(strlen(input) * sizeof(char));
	strcpy(exif_orig, input);

	if (type == JPEG) {
		//Lossy processing just uses the compression method before optimizing
		if (!pars->jpeg.lossless) {
			cclt_jpeg_compress(output, cclt_jpeg_decompress(input, &pars->jpeg), &pars->jpeg);
			//TODO Check memory leaks
			//If we are using lossy compression, the input file is the output of
			//the previous function
			//Exif must be copied from the original thou
			input = output;
		}
		//Optimize
		cclt_jpeg_optimize(input, output, pars->jpeg.exif_copy, exif_orig);
	} else if (type == PNG) {
		cclt_png_optimize(input, output, &pars->png);
	} else {
		printf("[WARNING] Unknown file type.\n");
		return -1;
	}
	return 0;
}

void cclt_start(cclt_parameters* pars, off_t* i_t_size, off_t* o_t_size) {

	struct stat st_buf;
	int i = 0;

	//Creates the output folder (which will always be needed)
	if (mkpath(pars->output_folder, S_IRWXU | S_IRWXG | S_IROTH | S_IXOTH) == -1) {
		if (errno != EEXIST) {
			fprintf(stderr, "[ERROR] Failed to create output directory.\n");
			exit(-5);
		}
	}

	while (i < pars->input_files_count) {

		off_t i_size, o_size;
		int status; //Pointer for stat() call

		char* output_filename = (char*) malloc ((strlen(pars->output_folder) + 1) * sizeof(char));

		strcpy(output_filename, pars->output_folder);

		//Append / if was not entered by user
		if (output_filename[strlen(pars->output_folder) - 1] != '/') {
			strcat(output_filename, "/");
		}

		output_filename = realloc(output_filename, (strlen(output_filename) + strlen(basename(pars->input_files[i]))) * sizeof(char));
		output_filename = strcat(output_filename, basename(pars->input_files[i]));

		//Get input stats
		status = stat(pars->input_files[i], &st_buf);
		if (status != 0) {
			fprintf(stderr, "[ERROR] Failed to get input file stats.\n");
			exit(-11);
		}

	    //Check if we ran into a folder
	    //TODO Check symlinks too
		if (is_directory(pars->input_files[i])) {
	    	//Folder found, but we don't need it here
			i++;
			continue;
		}

		//Get input file size
		i_size = st_buf.st_size;
		*(i_t_size) += i_size;

		//TODO Do we want a more verbose output?
		fprintf(stdout, "(%d/%d) %s -> %s\n",
						i + 1,
						pars->input_files_count,
						pars->input_files[i],
						output_filename);

		int routine = cclt_compress_routine(pars->input_files[i], output_filename, pars);
		if (routine == -1) {
			i++;
			continue;
		}

		//Get output stats
		status = stat(output_filename, &st_buf);
		if (status != 0) {
		//TODO This is not critical, but still something to be tracked
			fprintf(stderr, "[ERROR] Failed to get output file stats.\n");
			exit(-12);
		}
		o_size = st_buf.st_size;
		*(o_t_size) += o_size;

		fprintf(stdout, "%s -> %s [%.2f%%]\n",
						get_human_size(i_size),
						get_human_size(o_size),
						((float) o_size - i_size) * 100 / i_size);

		i++;
	}

}

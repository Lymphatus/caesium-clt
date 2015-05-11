#include <stdlib.h>
#include <stdio.h>
#include <getopt.h>
#include <ctype.h>
#include <errno.h>
#include <string.h>
#include <sys/stat.h>

#include "lossless.h"
#include "compress.h"
#include "utils.h"

#define VERSION "1.9.9 BETA"
#define BUILD 20150509

/* PARAMETERS:
	-q quality
	-e exif
	-o output folder
	-v version
	-l lossless
	-s scale
	-h help
	-R recursive
*/

cclt_compress_parameters parse_arguments(int argc, char* argv[]) {
	
	//Initialize default params
	cclt_compress_parameters parameters = initialize_compression_parameters();
	int c;

	while (optind < argc) {
		if ((c = getopt (argc, argv, "q:velo:s:hR")) != -1) {
			switch (c) {
				case 'v':
					printf("CCLT - Caesium Command Line Tool - Version %s (Build: %d)\n", VERSION, BUILD);
					exit(0);
					break;
				case '?':
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
				default:
					abort();
			}
		} else {
			int i = 0;
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

int main (int argc, char *argv[]) {
	errno = 0;
	//Parse arguments
	cclt_compress_parameters pars = parse_arguments(argc, argv);
	
	
	
	//Either -l or -q must be set but not together
	if ((pars.lossless == 1) ^ (pars.quality > 0) == 0) {
		//Both or none are set
		if (pars.lossless == 1 && pars.quality != -1) {
			fprintf(stderr, "-l option can't be used with -q. Either use one or the other. Aborting.\n");
			exit(-1);
		} else if (pars.lossless == 0 && pars.quality == -1) {
			fprintf(stderr, "Either -l or -q must be set. Aborting.\n");
			exit(-2);
		}
	} else {
		//One of them is set
		//If -q is set check it is within the 1-100 range	
		if (!(pars.quality >= 1 && pars.quality <= 100) && pars.lossless == 0) {
			fprintf(stderr, "Quality must be within a [1-100] range. Aborting.\n");
			exit(-3);
		}
	}
	
	//Check if you set the input files
	if (pars.input_files_count == 0) {
		fprintf(stderr, "No input files. Aborting.\n");
		exit(-9);
	}
	
	//Check if there's a valid scaling factor
	if (pars.scaling_factor <= 0) {
		fprintf(stderr, "Scaling factor must be > 0. Aborting.\n");
		exit(-6);
	}
		
	//Check if the output folder exists, otherwise create it
	if (pars.output_folder == NULL) {
		fprintf(stderr, "No -o option pointing to the destination folder. Aborting.\n");
		exit(-4);
	} else {
		if (mkdir(pars.output_folder, S_IRWXU | S_IRWXG | S_IROTH | S_IXOTH) == -1) {
			if (errno != EEXIST) {
				perror("mkdir");
				exit(-5);
			}
		}
	}
	
	
		
	for (int i = 0; i < pars.input_files_count; i++) {	
		char* output_filename = pars.output_folder;
		char* i_tmp = (char*) malloc (strlen(pars.input_files[i]) * sizeof(char));
		
		strcpy(i_tmp, pars.input_files[i]);
		if (output_filename[strlen(pars.output_folder -1)] != '/') {
			strcat(pars.output_folder, "/");
		}
		
		output_filename = strcat(pars.output_folder, get_filename_with_extension(i_tmp));
		printf("%s\n", pars.input_files[i]);
		cclt_optimize(pars.input_files[i], output_filename);
	}
		
	exit(0);
}

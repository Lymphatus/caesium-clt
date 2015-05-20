#include <sys/stat.h>
#include <sys/types.h>
#include <stdlib.h>
#include <limits.h>
#include <stdio.h>
#include <errno.h>
#include <setjmp.h>
#include <stdio.h>
#include <jpeglib.h>
#include <turbojpeg.h>
#include <string.h>
#include <getopt.h>
#include <ctype.h>
#include <linux/limits.h>


#include "utils.h"
#include "compress.h"
#include "lossless.h"

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

	return par;
}

int string_to_int(char* in_string) {
	long value = 0;
	char* endptr;
	errno = 0; //Error checking

	value = strtol(in_string, &endptr, 0); //Convert the string
	
	//Check errors
	if ((errno == ERANGE) || (errno != 0 && value == 0)) {
        perror("strtol");
        exit(-8);
    }

   if (endptr == in_string) {
        fprintf(stderr, "Parse error: No digits were found for -q option. Aborting.\n");
        exit(-7);
    }
	
	return value;
}

void print_help() {
	fprintf(stdout,
		"CCLT - Caesium Command Line Tools\n\n"
		"Usage: cclt [OPTION] INPUT...\n"
		"Compress your pictures up to 90%% without visible quality loss.\n\n"

		"Options:\n"
			"\t-q\tset output file quality between [1-100], ignored for non-JPEGs\n"
			"\t-e\tkeeps EXIF info during compression\n"
			"\t-o\tcompress to custom folder\n"
			"\t-l\tuse lossless optimization\n"
			"\t-s\tscale to value, expressed as percentage (e.g. 20%%)\n"
			"\t-R\tif input is a folder, scan subfolders too\n"
			"\t-h\tdisplay this help and exit\n"
			"\t-v\toutput version information and exit\n\n");
	exit(0);
}

void print_progress(int current, int max, char* message) {
	fprintf(stdout, "\e[?25l");
	fprintf(stdout, "\r%s[%d%%]", message, current * 100 / max);
	if (current == max) {
		fprintf(stdout, "\e[?25h\n");
	}
}

//TODO Recheck
int mkpath(const char *pathname, mode_t mode) {

	//Need include in Linux, not on OSX
	char parent[PATH_MAX], *p;
	/* make a parent directory path */
	strncpy(parent, pathname, sizeof(parent));
	parent[sizeof(parent) - 1] = '\0';
	for(p = parent + strlen(parent); *p != '/' && p != parent; p--);
	*p = '\0';
	/* try make parent directory */
	if(p != parent && mkpath(parent, mode) != 0)
	return -1;
	/* make this one if parent has been made */
	if(mkdir(pathname, mode) == 0)
	return 0;
	/* if it already exists that is fine */
	if(errno == EEXIST)
	return 0;
	return -1;
}

char* get_filename_with_extension(char* full_path) {
	char* dest;
		
	dest = strrchr(full_path, '/') + 1;	

	return dest;
}

void jcopy_markers_execute (j_decompress_ptr srcinfo, j_compress_ptr dstinfo) {
  jpeg_saved_marker_ptr marker;

  for (marker = srcinfo->marker_list; marker != NULL; marker = marker->next) {
    if (dstinfo->write_JFIF_header &&
        marker->marker == JPEG_APP0 &&
        marker->data_length >= 5 &&
        GETJOCTET(marker->data[0]) == 0x4A &&
        GETJOCTET(marker->data[1]) == 0x46 &&
        GETJOCTET(marker->data[2]) == 0x49 &&
        GETJOCTET(marker->data[3]) == 0x46 &&
        GETJOCTET(marker->data[4]) == 0)
      continue;                 
    if (dstinfo->write_Adobe_marker &&
        marker->marker == JPEG_APP0+14 &&
        marker->data_length >= 5 &&
        GETJOCTET(marker->data[0]) == 0x41 &&
        GETJOCTET(marker->data[1]) == 0x64 &&
        GETJOCTET(marker->data[2]) == 0x6F &&
        GETJOCTET(marker->data[3]) == 0x62 &&
        GETJOCTET(marker->data[4]) == 0x65)
      continue;                 
    jpeg_write_marker(dstinfo, marker->marker,
                      marker->data, marker->data_length);
  }
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

void cclt_compress_routine(char* input, char* output, cclt_compress_parameters* pars) {
	cclt_compress(output, cclt_decompress(input, pars), pars);
	cclt_optimize(output, output, pars->exif_copy, input);
}



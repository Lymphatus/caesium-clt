#include <stdlib.h>
#include <stdio.h>
#include <getopt.h>
#include <ctype.h>

#include "lossless.h"
#include "compress.h"

#define VERSION "1.9.9"
#define BUILD 20150508

/* PARAMETERS:
	-q quality
	-e exif
	-o output file
	-v version
	-l lossless
	-s 
	-h help
	-R recursive
	
*/

cclt_compress_parameters parse_arguments(int argc, char* argv[]) {
	cclt_compress_parameters parameters;
	int c;
	char *qvalue = NULL;
	char *evalue = NULL;
	char *ovalue = NULL;
	char *svalue = NULL;

	while ((c = getopt (argc, argv, "q:ve:lo:s:hR")) != -1) {
		switch (c) {
			case 'v':
				printf("CCLT - Caesium Command Line Tool - Version %s (Build: %d)\n", VERSION, BUILD);
				break;
			case '?':
				if (optopt == 'q' || optopt == 'e' || optopt == 'o' || optopt == 's') {
          			//fprintf (stderr, "Option -%c requires an argument.\n", optopt);
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
			case 'q':
				qvalue = optarg;
				break;
			case 'e':
				evalue = optarg;
				break;
			default:
        		abort();
		}
	}
	
	return parameters;
}

int main (int argc, char *argv[]) {
	
	cclt_compress_parameters parameters;

	//Check if there's at least one argument
	if (argc <= 1) {
		printf("CCLT requires at least one argument. Aborting.\n");
		return -1;
	}
	
	parameters = parse_arguments(argc, argv);

	return 0;
}

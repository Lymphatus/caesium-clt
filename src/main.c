#include <stdlib.h>
#include <stdio.h>
#include <ctype.h>
#include <errno.h>
#include <string.h>
#include <dirent.h>
#include <libgen.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <time.h>

#include "jpeg.h"
#include "compresshelper.h"
#include "utils.h"

/* PARAMETERS:
-q quality v
-e exif v
-o output folder v
-v version v
-l lossless v
-h help v
-R recursive v
-S keep folder structure
*/



//TODO If the output is INSIDE the folder we are passing as input, ignore it or we're gonna go in a infinite loop

int main (int argc, char *argv[]) {
	errno = 0;
	off_t i_t_size = 0, o_t_size = 0;

	//Parse arguments
	cclt_parameters pars = parse_arguments(argc, argv);

	//Either -l or -q must be set but not together
	if (!((pars.jpeg.lossless == 1) ^ (pars.jpeg.quality > 0))) {
		//Both or none are set
		if (pars.jpeg.lossless == 1 && pars.jpeg.quality > 0) {
			fprintf(stderr, "-l option can't be used with -q. Either use one or the other. Aborting.\n");
			exit(-1);
		} else if (pars.jpeg.lossless == 0 && pars.jpeg.quality <= 0) {
			fprintf(stderr, "Either -l or -q must be set. Aborting.\n");
			print_help();
			exit(-2);
		}
	} else {
		//One of them is set
		//If -q is set check it is within the 1-100 range
		if (!(pars.jpeg.quality >= 1 && pars.jpeg.quality <= 100) && pars.jpeg.lossless == 0) {
			fprintf(stderr, "Quality must be within a [1-100] range. Aborting.\n");
			exit(-3);
		}
	}

	//Check if you set the input files
	if (pars.input_files_count == 0) {
		fprintf(stderr, "No input files. Aborting.\n");
		exit(-9);
	}

	//Check if the output folder exists, otherwise create it
	if (pars.output_folder == NULL) {
		fprintf(stderr, "No -o option pointing to the destination folder. Aborting.\n");
		exit(-4);
	} else {
		if (mkpath(pars.output_folder, S_IRWXU | S_IRWXG | S_IROTH | S_IXOTH) == -1) {
			if (errno != EEXIST) {
				exit(-5);
			}
		}
	}

	//Start a timer
	clock_t start = clock(), diff;
	//We need the file list right here
	cclt_start(&pars, &i_t_size, &o_t_size);
	diff = clock() - start;

	fprintf(stdout, "-------------------------------\nCompression completed in %lum%lus\n%s -> %s [%.2f%% | %s]\n",
					diff / CLOCKS_PER_SEC / 60,
					diff / CLOCKS_PER_SEC % 60,
					get_human_size((long) i_t_size),
					get_human_size((long) o_t_size),
					((float) o_t_size - i_t_size) * 100 / i_t_size,
					get_human_size(((long) o_t_size - i_t_size)));

	return 0;
}

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
//TODO Trigger a warning if you are overwriting files

int main (int argc, char *argv[]) {
	errno = 0;
	off_t i_t_size = 0, o_t_size = 0;

	//Parse arguments
	cclt_parameters pars = parse_arguments(argc, argv);

	//Start a timer
	clock_t start = clock(), diff;
	//We need the file list right here
	cclt_start(&pars, &i_t_size, &o_t_size);

	diff = clock() - start;
	long msec = diff * 1000 / CLOCKS_PER_SEC;

	fprintf(stdout, "-------------------------------\nCompression completed in %lum%lus%lums\n%s -> %s [%.2f%% | %s]\n",
		msec / 1000 / 60,
		msec / 1000 % 60,
		msec % 1000,
		get_human_size((long) i_t_size),
		get_human_size((long) o_t_size),
		((float) o_t_size - i_t_size) * 100 / i_t_size,
		get_human_size(((long) o_t_size - i_t_size)));

	return 0;
}

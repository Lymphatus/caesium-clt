#include <stdlib.h>
#include <time.h>
#include <stdio.h>
#include <errno.h>
#include "utils.h"


int main(int argc, char* argv[]) {
	errno = 0;
	long execution_ms = 0;
	//TODO Parse arguments

	//Start a timer before calling the compression
	clock_t start = clock(), diff;

	//TODO Compress here
	//start(input_files, output_folder, options);

	//Get the difference
	diff = clock() - start;
	execution_ms = diff * 1000 / CLOCKS_PER_SEC;

	//Output the compression results
	fprintf(stdout,
			"Performed in %lum%lus%lums\n",
			execution_ms / 1000 / 60, execution_ms / 1000 % 60, execution_ms % 1000);
}
#include <stdlib.h>
#include <time.h>
#include <stdio.h>
#include <errno.h>
#include <caesium.h>
#include "utils.h"
#include "helper.h"


int main(int argc, char* argv[]) {
	errno = 0;
	long execution_ms = 0;
	cs_image_pars compress_options;
	cclt_options options;

	//Initialize the default parameters
	compress_options = initialize_parameters();
	options = parse_arguments(argv, &compress_options);
	//Start a timer before calling the compression
	clock_t start = clock(), diff;

	//TODO Compress here
	//start(input_files, output_folder, &options);
	printf("%s\n", options.output_folder);

	for (int i = 0; i < options.files_count; i++) {
		printf("%s\n", options.input_files[i]);
	}

	scan_folder("/Users/lymphatus/Documents/Progetti/C/caesium-clt/samples");

	//Cleanup the two memory allocated objects
	free(options.output_folder);
	free(options.input_files);

	//Get the difference
	diff = clock() - start;
	execution_ms = diff * 1000 / CLOCKS_PER_SEC;

	//Output the compression results
	fprintf(stdout,
			"Performed in %lum%lus%lums\n",
			execution_ms / 1000 / 60, execution_ms / 1000 % 60, execution_ms % 1000);

	exit(EXIT_SUCCESS);
}
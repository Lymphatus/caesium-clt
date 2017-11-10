#include <stdlib.h>
#include <time.h>
#include <stdio.h>
#include <caesium.h>
#include "utils.h"

#include <unistd.h>


int main(int argc, char *argv[])
{
	//Exit if less than 2 arguments
	if (argc < 2) {
		print_help();
		exit(EXIT_FAILURE);
	}

	long compression_time = 0;
	cs_image_pars compress_options;
	cclt_options options;

	//Initialize the default parameters
	compress_options = initialize_parameters();
	//Set them according to command line parameters
	options = parse_arguments(argv, &compress_options);

	//Start a timer before calling the compression
	clock_t start = clock(), diff;

	start_compression(&options, &compress_options);

	//Cleanup the two memory allocated objects
	free(options.input_files);

	//Get the difference
	diff = clock() - start;
	compression_time = diff * 1000 / CLOCKS_PER_SEC;


	//Output the compression results
	fprintf(stdout, "-------------------------------\n");
	compression_time / 1000 % 60 >= 1 ?
		fprintf(stdout, "Compression completed in %lum%lus\n",
				compression_time / 1000 / 60, compression_time / 1000 % 60) :
		fprintf(stdout, "Compression completed in %lum%lus%lums\n",
				compression_time / 1000 / 60, compression_time / 1000 % 60, compression_time % 1000);
	fprintf(stdout, "%s -> %s [%.2f%% | %s]\n",
			get_human_size(options.input_total_size), get_human_size(options.output_total_size),
			((float) options.output_total_size - options.input_total_size) * 100 / options.input_total_size,
			get_human_size((options.output_total_size - options.input_total_size)));


	exit(EXIT_SUCCESS);
}
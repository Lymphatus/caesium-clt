/*
 *
 * Copyright 2019 Matteo Paonessa
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */

#include <stdlib.h>
#include <time.h>
#include <stdio.h>
#include "utils.h"
#include "shared.h"

int main(int argc, char *argv[])
{
	//Exit if less than 2 arguments
	if (argc < 2) {
		print_help();
		exit(EXIT_FAILURE);
	}

	long compression_time = 0;
	cclt_options options;

	//Initialize the default parameters
    C_CSParameters compress_options = {
            false,
            80,
            80,
            false,
            20,
            60,
            false,
            0,
            0
    };
	//Set them according to command line parameters
	options = parse_arguments(argv, &compress_options);

	//Start a timer before calling the compression
	clock_t start = clock(), diff;

	start_compression(&options, compress_options);

	//Cleanup the memory allocated objects
	free(options.input_files);

	//Get the difference
	diff = clock() - start;
	compression_time = diff * 1000 / CLOCKS_PER_SEC;


	//Output the compression results
	print_to_console(stdout, verbose, "-------------------------------\n");
	compression_time / 1000 % 60 >= 1 ?
		print_to_console(stdout, verbose, "Compression completed in %lum%lus\n",
				compression_time / 1000 / 60, compression_time / 1000 % 60) :
		print_to_console(stdout, verbose, "Compression completed in %lum%lus%lums\n",
				compression_time / 1000 / 60, compression_time / 1000 % 60, compression_time % 1000);
	print_to_console(stdout, verbose, "%s -> %s [%.2f%% | %s]\n",
			get_human_size(options.input_total_size), get_human_size(options.output_total_size),
			((float) options.output_total_size - options.input_total_size) * 100 / options.input_total_size,
			get_human_size((options.output_total_size - options.input_total_size)));


	exit(EXIT_SUCCESS);
}
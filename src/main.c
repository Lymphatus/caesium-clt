#include <stdlib.h>
#include <stdio.h>
#include <getopt.h>
#include <ctype.h>
#include <errno.h>
#include <string.h>
#include <dirent.h> 
#include <sys/stat.h>
#include <sys/types.h>
#include <libexif/exif-data.h>

#include "lossless.h"
#include "compress.h"
#include "utils.h"

/* PARAMETERS:
	-q quality v
	-e exif v
	-o output folder v
	-v version v
	-l lossless v
	-s scale 
	-h help v
	-R recursive
*/

//TODO Use a general fuction to support folder separators

int main (int argc, char *argv[]) {
	struct stat st_buf;
	errno = 0;
	off_t i_t_size = 0, o_t_size = 0;

	//Parse arguments
	cclt_compress_parameters pars = parse_arguments(argc, argv);
	
	//Either -l or -q must be set but not together
	if (!((pars.lossless == 1) ^ (pars.quality > 0))) {
		//Both or none are set
		if (pars.lossless == 1 && pars.quality > 0) {
			fprintf(stderr, "-l option can't be used with -q. Either use one or the other. Aborting.\n");
			exit(-1);
		} else if (pars.lossless == 0 && pars.quality <= 0) {
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
		if (mkpath(pars.output_folder, S_IRWXU | S_IRWXG | S_IROTH | S_IXOTH) == -1) {
			if (errno != EEXIST) {
				exit(-5);
			}
		}
	}


	//This is the main loop. It iterates through all the input files provided.
	//It also extract the original filename to be saved in the new destination.
	//TODO Provide support for folder structure.	
	for (int i = 0; i < pars.input_files_count; i++) {
		off_t i_size, o_size;
		int status; //Pointer for stat() call
		char* output_filename = (char*) malloc ((strlen(pars.output_folder) + 2) * sizeof(char));
		char* i_tmp = (char*) malloc (strlen(pars.input_files[i]) * sizeof(char));
		
		strcpy(i_tmp, pars.input_files[i]);
		strcpy(output_filename, pars.output_folder);

		//Append / if was not entered by user
		if (output_filename[strlen(pars.output_folder - 1)] != '/') {
			strcat(output_filename, "/");
		}

		output_filename = realloc(output_filename, (strlen(output_filename) + strlen(get_filename_with_extension(i_tmp)) + 1) * sizeof(char));

		output_filename = strcat(output_filename, get_filename_with_extension(i_tmp));

		//TODO OVERALL progress update?
		//print_progress(i + 1, pars.input_files_count, "Progress: ");

		//Get input stats
		status = stat(pars.input_files[i], &st_buf);
	    if (status != 0) {
	        fprintf(stderr, "Failed to get input file stats. Aborting.\n");
			exit(-11);
	    }

	    //If the input is a folder, skip
	    if (S_ISDIR (st_buf.st_mode)) {
	    	//TODO If we find a folder, we need to get into it if -R is set
        	continue;
    	}

    	//Get input file size
    	i_size = st_buf.st_size;
    	i_t_size += i_size;

		//TODO Do we want a more verbose output?
		fprintf(stdout, "Compressing: %s -> %s\n", pars.input_files[i], output_filename);

		//Lossless optmization requested
		if (pars.lossless != 0) {
			cclt_optimize(pars.input_files[i], output_filename, pars.exif_copy, pars.input_files[i]);
		} else {
			//TODO Standard compression requested
			//unsigned char* buffer = cclt_decompress(pars.input_files[i], &pars);
			//cclt_compress(output_filename, buffer, &pars);
			cclt_compress_routine(pars.input_files[i], output_filename, &pars);
		}

		//Get output stats
		status = stat(output_filename, &st_buf);
	    if (status != 0) {
	    	//TODO This is not critical
	        fprintf(stderr, "Failed to get output file stats. Aborting.\n");
			exit(-12);
	    }
	    o_size = st_buf.st_size;
	    o_t_size += o_size;

	    fprintf(stdout, "%ld bytes -> %ld bytes [%.2f%%]\n",
			(long) i_size, (long) o_size, ((float) o_size - i_size) * 100 / i_size);

		//TODO Perform the required instructions
		//TODO Provide complete progress support
		//INPUT: pars.input_files[i] | OUTPUT: output_filename

		//Free allocated memory
		//TODO Causing segfaults
		//free(output_filename);
		//free(i_tmp);
	}

	fprintf(stdout, "Compression completed.\n%ld bytes -> %ld bytes [%.2f%%]\n",
		(long) i_t_size, (long) o_t_size, ((float) o_t_size - i_t_size) * 100 / i_t_size);
		
	exit(0);
}

#include <stdlib.h>
#include <stdio.h>
#include <getopt.h>
#include <ctype.h>
#include <errno.h>
#include <string.h>
#include <dirent.h> 
#include <sys/stat.h>
#include <sys/types.h>

#include "lossless.h"
#include "compress.h"
#include "utils.h"

/* PARAMETERS:
	-q quality v
	-e exif v
	-o output folder v
	-v version v
	-l lossless v
	-s scale v
	-h help v
	-R recursive
*/



//TODO Use a general fuction to support folder separators

void cclt_start(char** input_files, int n, char* output_folder, cclt_compress_parameters* pars, off_t* i_t_size, off_t* o_t_size) {
	struct stat st_buf;
	int i = 0, lossless = pars->lossless, exif = pars->exif_copy, recursive = pars->recursive;

	if (mkpath(output_folder, S_IRWXU | S_IRWXG | S_IROTH | S_IXOTH) == -1) {
		if (errno != EEXIST) {
			exit(-5);
		}
	}

	while (i < n) {
		off_t i_size, o_size;
		int status; //Pointer for stat() call
		char* output_filename = (char*) malloc ((strlen(output_folder) + 2) * sizeof(char));
		char* i_tmp = (char*) malloc (strlen(input_files[i]) * sizeof(char));
		
		strcpy(i_tmp, input_files[i]);
		strcpy(output_filename, output_folder);

		//Append / if was not entered by user
		if (output_filename[strlen(output_folder) - 1] != '/') {
			strcat(output_filename, "/");
		}

		output_filename = realloc(output_filename, (strlen(output_filename) + strlen(get_filename_with_extension(i_tmp)) + 1) * sizeof(char));
		output_filename = strcat(output_filename, get_filename_with_extension(i_tmp));

		//TODO OVERALL progress update?
		//print_progress(i + 1, pars.input_files_count, "Progress: ");

		//Get input stats
		status = stat(input_files[i], &st_buf);
	    if (status != 0) {
	        fprintf(stderr, "Failed to get input file stats. Aborting.\n");
			exit(-11);
	    }

	    //Check if we ran into a folder
	    //TODO Check symlinks too
	    if (S_ISDIR (st_buf.st_mode) && recursive == 0) {
	    	//Folder found, but we don't need it here
	    	i++;
        	continue;
    	} else if (S_ISDIR (st_buf.st_mode) && recursive != 0) {
    		//Folder found, we need to get into it
    		

    		/*
				1. Scan the entire folder input_files[i]
				2. Get a new array containing all the files and folders
				3. Set the output folder to be one step deeper
				3. Call cclt_start(new_list, new folder, same, same, same)
    		*/

			//TODO malloc?
			//char** new_files = (char**) malloc(256 * sizeof(char*));
			//new_files = scan_folder(input_files[i], 0);
    		//cclt_start(new_files, output_folder, pars, i_t_size, o_t_size);
    		//i++;
    		//TODO Remove this after this funcion is fully completed
    		//free(new_files);
    		continue;
    	}

    	//Get input file size
    	i_size = st_buf.st_size;
    	*(i_t_size) += i_size;

		//TODO Do we want a more verbose output?
		fprintf(stdout, "Compressing: %s -> %s\n", input_files[i], output_filename);

		if (lossless != 0) {
			cclt_optimize(input_files[i], output_filename, exif, input_files[i]);
		} else {
			cclt_compress_routine(input_files[i], output_filename, pars);
		}

		//Get output stats
		status = stat(output_filename, &st_buf);
	    if (status != 0) {
	    	//TODO This is not critical, but still something to be tracked
	        fprintf(stderr, "Failed to get output file stats. Aborting.\n");
			exit(-12);
	    }
	    o_size = st_buf.st_size;
	    *(o_t_size) += o_size;

	    fprintf(stdout, "%ld bytes -> %ld bytes [%.2f%%]\n",
			(long) i_size, (long) o_size, ((float) o_size - i_size) * 100 / i_size);

		//TODO Perform the required instructions
		//TODO Provide complete progress support
		//INPUT: pars.input_files[i] | OUTPUT: output_filename

		//Free allocated memory
		//TODO Causing segfaults
		//free(output_filename);
		//free(i_tmp);
		i++;
	}

}

int main (int argc, char *argv[]) {
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

	if ((pars.lossless == 1) && (pars.scaling_factor != 100)) {
		fprintf(stderr, "Lossless scaling is not supported. Use -q instead. Aborting.\n");
		exit(-13);
	}

	cclt_start(pars.input_files, pars.input_files_count, pars.output_folder, &pars, &i_t_size, &o_t_size);

	fprintf(stdout, "Compression completed.\n%ld bytes -> %ld bytes [%.2f%%]\n",
		(long) i_t_size, (long) o_t_size, ((float) o_t_size - i_t_size) * 100 / i_t_size);
		
	exit(0);
}

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



//TODO Use a general fuction to support folder separators
//TODO If inputs a folder AND files, send an error
//TODO If the output is INSIDE the folder we are passing as input, ignore it or we're gonna go in a infinite loop

void cclt_start(char** input_files, int n, char* output_folder, cclt_compress_parameters* pars, off_t* i_t_size, off_t* o_t_size) {

	struct stat st_buf;
	int i = 0;

	if (mkpath(output_folder, S_IRWXU | S_IRWXG | S_IROTH | S_IXOTH) == -1) {
		if (errno != EEXIST) {
			exit(-5);
		}
	}

	while (i < n) {

		off_t i_size, o_size;
		int status; //Pointer for stat() call

		char* output_filename = (char*) malloc ((strlen(output_folder) + 1) * sizeof(char));
		//CRITICAL SEGFUCKINGFAULT
		//TODO input_files[i] è NULL when we pass a folder with -R
		char* i_tmp = (char*) malloc (strlen(input_files[i]) * sizeof(char));


		strcpy(i_tmp, input_files[i]);
		strcpy(output_filename, output_folder);

		//Append / if was not entered by user
		if (output_filename[strlen(output_folder) - 1] != '/') {
			strcat(output_filename, "/");
		}

		//fprintf(stderr, "%s - %lu\n", output_filename, strlen(output_filename) + strlen(get_filename_with_extension(i_tmp)) + 1);

		output_filename = realloc(output_filename, (strlen(output_filename) + strlen(basename(i_tmp))) * sizeof(char));
		output_filename = strcat(output_filename, basename(i_tmp));

		//Get input stats
		status = stat(input_files[i], &st_buf);
		if (status != 0) {
			fprintf(stderr, "Failed to get input file stats. Aborting.\n");
			exit(-11);
		}

	    //Check if we ran into a folder
	    //TODO Check symlinks too
		if (is_directory(input_files[i])) {
	    	//Folder found, but we don't need it here
	    	printf("Folder found\n");
			i++;
			continue;
		}

		//Get input file size
		i_size = st_buf.st_size;
		*(i_t_size) += i_size;

		//TODO Do we want a more verbose output?
		fprintf(stdout, "Compressing: %s -> %s\n", input_files[i], output_filename);

		int routine = cclt_compress_routine(input_files[i], output_filename, pars);
		if (routine == -1) {
			i++;
			continue;
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

		fprintf(stdout, "%s -> %s [%.2f%%]\n",
			get_human_size(i_size), get_human_size(o_size), ((float) o_size - i_size) * 100 / i_size);

		//TODO Provide complete progress support
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
			print_help();
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
	cclt_start(pars.input_files, pars.input_files_count, pars.output_folder, &pars, &i_t_size, &o_t_size);
	/*for (int i = 0; i < pars.input_files_count; i++) {
		printf("FILE %d: %s\n", i, pars.input_files[i]);
	}*/
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

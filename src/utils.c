#include <stdio.h>
#include <stdlib.h>
#include <sys/stat.h>
#include <caesium.h>
#include <limits.h>
#include <math.h>
#include "utils.h"
#include "tinydir.h"
#include "error.h"


void print_help()
{
	fprintf(stdout,
			"CaesiumCLT - Caesium Command Line Tools\n\n"
					"Usage: caesiumclt [OPTIONS] INPUT...\n"
					"Compress your pictures up to 90%% without visible quality loss.\n\n"

					"Options:\n"
					"\t-q, --quality\t\tset output file quality between [0-100], 0 for optimization\n"
					"\t-e, --exif\t\tkeeps EXIF info during compression\n"
					"\t-o, --output\t\toutput folder\n"
					"\t-R, --recursive\t\tif input is a folder, scan subfolders too\n"
					//TODO Remove this warning
					"\t-S, --keep-structure\tkeep the folder structure [Not active yet], use with -R\n"
					"\t-h, --help\t\tdisplay this help and exit\n"
					"\t-v, --version\t\toutput version information and exit\n\n");
	exit(EXIT_SUCCESS);
}

bool is_directory(const char *path)
{
	tinydir_file file;

	if (tinydir_file_open(&file, path) == -1) {
		display_error(ERROR, 6);
		exit(EXIT_FAILURE);
	}

	return (bool) file.is_dir;
}

int scan_folder(const char *directory, cclt_options *options, bool recursive)
{
	int n = 0;
	tinydir_dir dir;
	tinydir_open(&dir, directory);

	while (dir.has_next) {
		tinydir_file file;
		tinydir_readfile(&dir, &file);

		if (file.is_dir) {
			if (strcmp(file.name, ".") != 0 && strcmp(file.name, "..") != 0 && recursive) {
				scan_folder(file.path, options, true);
			}
		} else {
			options->input_files = realloc(options->input_files, (options->files_count + 1) * sizeof(char *));
			options->input_files[options->files_count] = malloc((strlen(file.path) + 1) * sizeof(char));
			strncpy(options->input_files[options->files_count], file.path, strlen(file.path) + 1);
			options->files_count++;
			n++;
		}
		tinydir_next(&dir);
	}

	tinydir_close(&dir);
	return n;
}

//TODO Recheck
int mkpath(const char *pathname, mode_t mode)
{
	char parent[PATH_MAX], *p;
	/* make a parent directory path */
	strncpy(parent, pathname, sizeof(parent));
	parent[sizeof(parent) - 1] = '\0';
	for (p = parent + strlen(parent); *p != '/' && p != parent; p--);
	*p = '\0';
	/* try make parent directory */
	if (p != parent && mkpath(parent, mode) != 0) {
		return -1;
	}
	/* make this one if parent has been made */
	if (mkdir(pathname, mode) == 0) {
		return 0;
	}
	/* if it already exists that is fine */
	if (errno == EEXIST) {
		return 0;
	}
	return -1;
}

char *get_filename(char *full_path)
{
	char *token, *tofree;

	//Get just the filename
	tofree = strdup(full_path);
	//TODO change to strncpy
	strcpy(tofree, full_path);
	//TODO Windows?
	while ((token = strsep(&tofree, "/")) != NULL) {
		if (tofree == NULL) {
			break;
		}
	}

	free(tofree);

	return token;
}

off_t get_file_size(const char *path)
{
	tinydir_file file;

	if (tinydir_file_open(&file, path) == -1) {
		display_error(ERROR, 7);
		exit(EXIT_FAILURE);
	}

	return file._s.st_size;
}

char *get_human_size(off_t size)
{
	//We should not get more than TB images
	char *unit[5] = {"B", "KB", "MB", "GB", "TB"};
	//Index of the array containing the correct unit
	double order = floor(log2(labs(size)) / 10);
	//Alloc enough size for the final string
	char *final = (char *) malloc(((int) (floor(log10(labs(size))) + 4)) * sizeof(char));

	//If the order exceeds 4, something is fishy
	if (order > 4) {
		order = 4;
	}

	//Copy the formatted string into the buffer
	sprintf(final, "%.2f %s", size / (pow(1024, order)), unit[(int) order]);
	//And return it
	return final;
}


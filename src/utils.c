#include <stdio.h>
#include <stdlib.h>
#include <sys/stat.h>
#include <caesium.h>
#include <limits.h>
#include "utils.h"
#include "tinydir.h"


void print_help()
{
	fprintf(stdout,
			"CaesiumCLT - Caesium Command Line Tools\n\n"
					"Usage: caesiumclt [OPTIONS] INPUT...\n"
					"Compress your pictures up to 90%% without visible quality loss.\n\n"

					"Options:\n"
					"\t-q, --quality\t\t\tset output file quality between [0-100], 0 for optimization\n"
					"\t-e, --exif\t\t\t\tkeeps EXIF info during compression\n"
					"\t-o, --output\t\t\toutput folder\n"
					"\t-R, --recursive\t\t\tif input is a folder, scan subfolders too\n"
					//TODO Remove this warning
					"\t-S, --keep-structure\tkeep the folder structure [Not active yet], use with -R\n"
					"\t-h, --help\t\t\t\tdisplay this help and exit\n"
					"\t-v, --version\t\t\toutput version information and exit\n\n");
	exit(EXIT_SUCCESS);
}

int is_directory(const char *path)
{
	tinydir_dir dir;
	tinydir_file file;
	bool is_dir = false;

	tinydir_open(&dir, path);

	tinydir_readfile(&dir, &file);
	is_dir = (bool) file.is_dir;

	tinydir_close(&dir);

	return is_dir;
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
	//TODO Change on Windows
	while ((token = strsep(&tofree, "/")) != NULL) {
		if (tofree == NULL) {
			break;
		}
	}

	free(tofree);

	return token;
}


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

int is_directory(const char* path)
{
	struct stat sb;

	return ((stat(path, &sb) && S_ISDIR(sb.st_mode)));

}

int scan_folder(const char *directory)
{
	//TODO Not recursive at all
	int n = 0;
	tinydir_dir dir;
	tinydir_open(&dir, directory);

	while (dir.has_next)
	{
		tinydir_file file;
		tinydir_readfile(&dir, &file);

		if (file.is_dir) {
			if (strcmp(file.name, ".") != 0 && strcmp(file.name, "..") != 0) {
				printf("%s/", file.name);
				char real[PATH_MAX];
				realpath(file.name, real);
				scan_folder(real);
			}
		} else {
			printf("%s\n", file.name);
		}

		tinydir_next(&dir);
		n++;
	}

	tinydir_close(&dir);
	return 0;
}

/*
 * int n = 0;
	tinydir_dir dir;
	tinydir_open(&dir, directory);

	while (dir.has_next) {
		tinydir_file file;
		tinydir_readfile(&dir, &file);

		printf("%s", file.name);
		if (file.is_dir && (file.name != "." && file.name != "..")) {
			scan_folder(file.name);
			printf("/");
		} else {
			tinydir_next(&dir);
			n++;
		}
		printf("\n");
	}

	tinydir_close(&dir);
	return 0;
 */


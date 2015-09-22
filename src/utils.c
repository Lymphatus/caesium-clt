#include <sys/stat.h>
#include <sys/types.h>
#include <stdlib.h>
#include <limits.h>
#include <stdio.h>
#include <errno.h>
#include <setjmp.h>
#include <stdio.h>
#include <string.h>
#include <getopt.h>
#include <ctype.h>
#include <dirent.h>
#include <math.h>

#ifdef __linux
	#include <linux/limits.h>
#endif

#include "utils.h"

int string_to_int(char* in_string) {
	long value = 0;
	char* endptr;
	errno = 0; //Error checking

	value = strtol(in_string, &endptr, 0); //Convert the string
	
	//Check errors
	if ((errno == ERANGE) || (errno != 0 && value == 0)) {
        perror("strtol");
        exit(-8);
    }

   if (endptr == in_string) {
        fprintf(stderr, "Parse error: No digits were found for -q option. Aborting.\n");
        exit(-7);
    }
	
	return value;
}

void print_help() {
	fprintf(stdout,
		"CCLT - Caesium Command Line Tools\n\n"
		"Usage: caesiumclt [OPTIONs] INPUT...\n"
		"Compress your pictures up to 90%% without visible quality loss.\n\n"

		"Options:\n"
			"\t-q\tset output file quality between [1-100], ignored for non-JPEGs\n"
			"\t-e\tkeeps EXIF info during compression\n"
			"\t-o\tcompress to custom folder\n"
			"\t-l\tuse lossless optimization\n"
			"\t-s\tscale to value, expressed as percentage (e.g. 20%%) [Only 1/2^n allowed]\n"
			"\t-R\tif input is a folder, scan subfolders too\n"
			//TODO Remove this warning
			"\t-S\tkeep the folder structure [Not active yet]\n"
			"\t-h\tdisplay this help and exit\n"
			"\t-v\toutput version information and exit\n\n");
	exit(0);
}

void print_progress(int current, int max, char* message) {
	fprintf(stdout, "\e[?25l");
	fprintf(stdout, "\r%s[%d%%]", message, current * 100 / max);
	if (current == max) {
		fprintf(stdout, "\e[?25h\n");
	}
}

//TODO Recheck
int mkpath(const char *pathname, mode_t mode) {

	//Need include in Linux, not on OSX
	char parent[PATH_MAX], *p;
	/* make a parent directory path */
	strncpy(parent, pathname, sizeof(parent));
	parent[sizeof(parent) - 1] = '\0';
	for(p = parent + strlen(parent); *p != '/' && p != parent; p--);
	*p = '\0';
	/* try make parent directory */
	if(p != parent && mkpath(parent, mode) != 0) {
		return -1;
	}
	/* make this one if parent has been made */
	if(mkdir(pathname, mode) == 0) {
		return 0;
	}
	/* if it already exists that is fine */
	if (errno == EEXIST) {
		return 0;
	}
	return -1;
}

char** scan_folder(char* basedir, int* n, int recur) {
	DIR *dir;
	struct dirent *ent;
	char* entpath = NULL;
	struct stat s;
	int indexes = 0;
	int i = 0;
	char** fileList = NULL;

	char absolute_path[PATH_MAX];
	realpath(basedir, absolute_path);

	dir = opendir(absolute_path);
	
	if (dir != NULL) {		
		while ((ent = readdir(dir)) != NULL) {
			// Do not allow "." or ".."
			if (strcmp(ent->d_name, ".") == 0 || strcmp(ent->d_name, "..") == 0) {
				continue;
			}
			
			//TODO allocate for this entry
			//Basedir + filename + separator
			entpath = realloc(entpath, (strlen(absolute_path) + strlen(ent->d_name) + 1) * sizeof(char));
			strcpy(entpath, absolute_path);
			//Append separator
			strcat(entpath, "/");
			//Append the name
			strcat(entpath, ent->d_name);

			//Gets stats
			stat(entpath, &s);
			
			if (S_ISDIR(s.st_mode)) {			
				// Directory, walk it if recursive is set
				if (recur != 0) {
					fileList = scan_folder(entpath, n, recur);
				}
			} else {
				//File, add to the list
				//New entry in the array

				indexes++;
				//Alloc new room for the array
				fileList = realloc(fileList, indexes * sizeof(char*));
				fileList[i] = (char*) malloc(strlen(entpath) * sizeof(char));
				//Copy the file path in the array
				fileList[i] = strcpy(fileList[i], entpath);
				i++;
			}
		}
		closedir(dir);
	} else {
		fprintf(stderr, "Failed to open folder. Aborting.\n");
		exit(-19);
	}
	free(entpath);
	*n = i;
	return fileList;
}

enum image_type detect_image_type(char* path) {
	//Open the file
	FILE* fp;
	unsigned char* type_buffer = valloc(2);

	fp = fopen(path, "r");

	if (fp == NULL) {
		fprintf(stderr, "Cannot open input file for type detection. Aborting.\n");
		exit(-14);
	}
	//Read enough bytes
	if (fread(type_buffer, 1, 2, fp) < 2) {
		fprintf(stderr, "Cannot read file type. Aborting.\n");
		exit(-15);
	}
	//We don't need it anymore
	fclose(fp);

	//Check the bytes against the JPEG and PNG specs
	if (((int) type_buffer[0] == 0xFF) && ((int) type_buffer[1] == 0xD8)) {
		free(type_buffer);
		return JPEG;
	} else if (((int) type_buffer[0] == 0x89) && ((int) type_buffer[1] == 0x50)) {
		free(type_buffer);
		return PNG;
	} else {
		return UNKN;
	}
}

int isDirectory(const char *file_path) {
	struct stat s;
	stat(file_path, &s);
	return S_ISDIR(s.st_mode);
}

char* get_human_size(long size) {
	//We should not get more than TB images
	char* unit[5] = {"B", "KB", "MB", "GB", "TB"};
	//Index of the array containing the correct unit
	double order = floor(log2(labs(size)) / 10);
	//Alloc enough size for the final string
	char* final = (char*) malloc(((int) (floor(log10(labs(size))) + 4)) * sizeof(char));

	//If the order exceeds 4, something is fishy
	if (order > 4) {
		fprintf(stderr, "Do you really have such a huge file?\n");
		order = 4;
	}

	//Copy the formatted string into the buffer
	sprintf(final, "%.2f %s", size / (pow(1024, order)), unit[(int)order]);
	//And return it
	return final;
}
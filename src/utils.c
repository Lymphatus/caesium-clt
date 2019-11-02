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

#include <stdio.h>
#include <stdlib.h>
#include <sys/stat.h>
#include <caesium.h>
#include <limits.h>
#include <math.h>
#include <errno.h>
#include <stdarg.h>
#ifdef _WIN32
#include <stdint.h>
#endif

#include "utils.h"
#include "vendor/tinydir.h"
#include "error.h"

void print_help()
{
	print_to_console(stdout, 1,
			"CaesiumCLT - Caesium Command Line Tools\n\n"
					"Usage: caesiumclt [OPTIONS] INPUT...\n"
					"Command line image compressor.\n\n"

					"Options:\n"
					"\t-q, --quality\t\tset output file quality between [0-100], 0 for optimization\n"
					"\t-e, --exif\t\tkeeps EXIF info during compression\n"
					"\t-o, --output\t\toutput folder\n"
					"\t-s, --scale\t\t[EXPERIMENTAL] scale the image. Allowed formats are [.x, 0.x, n/d, xx%%].\n\t\t\t\tMust be > 0 and <= 1.0.\n"
					"\t-R, --recursive\t\tif input is a folder, scan subfolders too\n"
					"\t-S, --keep-structure\tkeep the folder structure, use with -R\n"
					"\t-O, --overwrite\t\tOverwrite policy: all, none, prompt, bigger. Default is bigger.\n"
					"\t-d, --dry-run\t\tdo not really compress files but just show output paths\n"
					"\t-Q, --quiet\t\tsuppress all output\n"
					"\t-h, --help\t\tdisplay this help and exit\n"
					"\t-v, --version\t\toutput version information and exit\n\n");
	exit(EXIT_SUCCESS);
}

bool is_directory(const char *path)
{
#ifdef _WIN32
	tinydir_dir dir;

	return tinydir_open(&dir, path) != -1;

#else
	tinydir_file file;

	if (tinydir_file_open(&file, path) == -1) {
		display_error(ERROR, 6);
	}

	return (bool) file.is_dir;
#endif
}

void scan_folder(const char *directory, cclt_options *options, bool recursive)
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
			snprintf(options->input_files[options->files_count],
					 strlen(file.path) + 1, "%s", file.path);
#ifdef _WIN32
			options->input_files[options->files_count] = str_replace(options->input_files[options->files_count], "/", "\\");
#endif
			options->files_count++;
		}
		tinydir_next(&dir);
	}

	tinydir_close(&dir);
}

int mkpath(const char *pathname)
{
	char parent[PATH_MAX], *p;
	/* make a parent directory path */
	strncpy(parent, pathname, sizeof(parent));
	parent[sizeof(parent) - 1] = '\0';
	for (p = parent + strlen(parent); *p != '/' && p != parent; p--);
	*p = '\0';
	/* try make parent directory */
	if (p != parent && mkpath(parent) != 0) {
		return -1;
	}
	/* make this one if parent has been made */
#ifdef _WIN32
	if (mkdir(pathname) == 0) {
		return 0;
	}
#else
	if (mkdir(pathname, 0755) == 0) {
		return 0;
	}
#endif
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
#ifdef _WIN32
	while ((token = strsep(&tofree, "\\")) != NULL) {
#else
	while ((token = strsep(&tofree, "/")) != NULL) {
#endif
		if (tofree == NULL) {
			break;
		}
	}

	free(tofree);

	return token;
}

off_t get_file_size(const char *path)
{
	FILE *f = fopen(path, "rb");
	if (f == NULL) {
		display_error(WARNING, 7);
		return 0;
	}
	fseek(f, 0, SEEK_END);
	off_t len = ftell(f);
	fclose(f);

	return len;
}

char *get_human_size(off_t size)
{
	if (size == 0) {
		return "0.00 B";
	}

	//We should not get more than TB images
	char *unit[5] = {"B", "KB", "MB", "GB", "TB"};
	//Index of the array containing the correct unit
	double order = floor(log2(labs(size)) / 10);
	//Alloc enough size for the final string
	char *final = (char *) malloc(((int) (floor(log10(labs(size))) + 5)) * sizeof(char));

	//If the order exceeds 4, something is fishy
	if (order > 4) {
		order = 4;
	}

	//Copy the formatted string into the buffer
	sprintf(final, "%.2f %s", size / (pow(1024, order)), unit[(int) order]);
	//And return it
	return final;
}

bool file_exists(const char *file_path)
{
	struct stat buffer;
	return (stat(file_path, &buffer) == 0);
}

int strndx(const char *string, const char search)
{
	char *pointer = strchr(string, search);
	if (pointer == NULL) {
		return -1;
	} else {
		return (int) (pointer - string);
	}
}

double parse_scale_factor(const char *factor_string)
{
	/*
	 * We're gonna accept scaling factor as such:
	 * a. 0.5 or .5 or 1
	 * b. 3/4
	 * c. 80%
	 * We're gonna test a. last since it's "standard"
	 */
	int index;
	bool parse_error = false;
	double scale_factor = 1.0;

	if ((index = strndx(factor_string, '/')) != -1
		&& strndx(factor_string, '%') == -1) {
		if (index == 0 || index == strlen(factor_string) - 1) {
			parse_error = true;
		} else {
			char *num_str = malloc((index + 1) * sizeof(char));
			char *den_str = malloc((strlen(factor_string) - index) * sizeof(char));
			snprintf(num_str, index + 1, "%s", factor_string);
			snprintf(den_str, strlen(factor_string) - index, "%s", factor_string + index + 1);
			long num = strtol(num_str, (char **) NULL, 10);
			long den = strtol(den_str, (char **) NULL, 10);
			if (num > 0 && den > 0) {
				scale_factor = (double) num / (double) den;
			} else {
				parse_error = true;
			}
		}
	} else if ((index = strndx(factor_string, '%')) != -1
			   && strndx(factor_string, '/') == -1) {
		if (index != strlen(factor_string) - 1) {
			parse_error = true;
		} else {
			char *num_str = malloc((index + 1) * sizeof(char));
			snprintf(num_str, index + 1, "%s", factor_string);
			long num = strtol(num_str, (char **) NULL, 10);
			scale_factor = (double) num / 100.0;
			free(num_str);
		}
	} else {
		// Maybe it's just a number!
		char *endstr;
		scale_factor = strtod(factor_string, &endstr);
		if (scale_factor == 0.0 || endstr[0] != '\0') {
			parse_error = true;
		}
	}

	if (parse_error) {
		display_error(WARNING, 14);
		exit(-14);
	}

	if (scale_factor <= 0 || scale_factor > 1.0) {
		display_error(WARNING, 13);
		return 1.0;
	}
	return scale_factor;
}

overwrite_policy parse_overwrite_policy(const char* overwrite_string)
{
	if (strcmp(overwrite_string, "none") == 0) {
		return none;
	} else if (strcmp(overwrite_string, "prompt") == 0) {
		return prompt;
	} else if (strcmp(overwrite_string, "bigger") == 0) {
		return bigger;
	} else if(strcmp(overwrite_string, "all") == 0) {
		return all;
	}
	display_error(WARNING, 15);
	return bigger;
}

void print_to_console(FILE* buffer, int verbose, const char* format, ...)
{
	if (!verbose) {
	    return;
	}

    va_list args;

    va_start(args, format);
    vfprintf(buffer, format, args);
    va_end(args);
}


#ifdef _WIN32
char *str_replace(char *orig, char *rep, char *with) {
	char *result; // the return string
	char *ins;    // the next insert point
	char *tmp;    // varies
	int len_rep;  // length of rep (the string to remove)
	int len_with; // length of with (the string to replace rep with)
	int len_front; // distance between rep and end of last rep
	int count;    // number of replacements

	if (!orig || !rep)
		return NULL;
	len_rep = strlen(rep);
	if (len_rep == 0)
		return NULL;
	if (!with)
		with = "";
	len_with = strlen(with);

	ins = orig;
	for (count = 0; tmp = strstr(ins, rep); ++count) {
		ins = tmp + len_rep;
	}

	tmp = result = malloc(strlen(orig) + (len_with - len_rep) * count + 1);

	if (!result)
		return NULL;

	while (count--) {
		ins = strstr(orig, rep);
		len_front = ins - orig;
		tmp = strncpy(tmp, orig, len_front) + len_front;
		tmp = strcpy(tmp, with) + len_with;
		orig += len_front + len_rep;
	}
	strcpy(tmp, orig);
	return result;
}

char *strsep (char **stringp, const char *delim)
{
	char *begin, *end;

	begin = *stringp;
	if (begin == NULL)
		return NULL;

	/* A frequent case is when the delimiter string contains only one
	   character.  Here we don't need to call the expensive `strpbrk'
	   function and instead work using `strchr'.  */
	if (delim[0] == '\0' || delim[1] == '\0')
	{
		char ch = delim[0];

		if (ch == '\0')
			end = NULL;
		else
		{
			if (*begin == ch)
				end = begin;
			else if (*begin == '\0')
				end = NULL;
			else
				end = strchr (begin + 1, ch);
		}
	}
	else
		/* Find the end of the token.  */
		end = strpbrk (begin, delim);

	if (end)
	{
		/* Terminate the token and set *STRINGP past NUL character.  */
		*end++ = '\0';
		*stringp = end;
	}
	else
		/* No more delimiters; this is the last token.  */
		*stringp = NULL;

	return begin;
}

#endif


#ifndef CAESIUM_CLT_HELPER_H
#define CAESIUM_CLT_HELPER_H

#include <caesium.h>

#ifdef _WIN32
#define MAX_PATH_SIZE _MAX_PATH
#else
#include <limits.h>
#define MAX_PATH_SIZE PATH_MAX
#endif

typedef struct cclt_options
{
	char **input_files;
	char input_folder[MAX_PATH_SIZE];
	char output_folder[MAX_PATH_SIZE];
	bool recursive;
	bool keep_structure;
	int files_count;
	off_t input_total_size;
	off_t output_total_size;
	bool dry_run;
	bool no_clobber;
} cclt_options;

cclt_options parse_arguments(char *argv[], cs_image_pars *options);

int start_compression(cclt_options *options, cs_image_pars *parameters);

#endif //CAESIUM_CLT_HELPER_H

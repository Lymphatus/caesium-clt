#ifndef CAESIUM_CLT_HELPER_H
#define CAESIUM_CLT_HELPER_H

#include <caesium.h>

typedef struct cclt_options
{
	char **input_files;
	char *input_folder;
	char *output_folder;
	bool recursive;
	bool keep_structure;
	int files_count;
	off_t input_total_size;
	off_t output_total_size;
} cclt_options;

cclt_options parse_arguments(char *argv[], cs_image_pars *options);

int start_compression(cclt_options *options, cs_image_pars *parameters);

#endif //CAESIUM_CLT_HELPER_H

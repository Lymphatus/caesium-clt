#ifndef CAESIUM_CLT_HELPER_H
#define CAESIUM_CLT_HELPER_H

#include <caesium.h>

typedef struct cclt_options {
	char **input_files;
	char *output_folder;
	bool recursive;
	bool keep_structure;
	int files_count;
} cclt_options;


void initialize_jpeg_parameters(cs_image_pars *options);

void initialize_png_parameters(cs_image_pars *options);

cs_image_pars initialize_parameters();

cclt_options parse_arguments(char *argv[], cs_image_pars *options);

#endif //CAESIUM_CLT_HELPER_H

#ifndef CAESIUM_CLT_HELPER_H
#define CAESIUM_CLT_HELPER_H

#include <caesium.h>

void initialize_jpeg_parameters(cs_image_pars *options);

void initialize_png_parameters(cs_image_pars *options);

cs_image_pars initialize_parameters();

cs_image_pars parse_arguments(int argc, char *argv[]);

#endif //CAESIUM_CLT_HELPER_H

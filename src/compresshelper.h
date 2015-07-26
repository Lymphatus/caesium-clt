#ifndef CCLT_COMPRESSHELPER
#define CCLT_COMPRESSHELPER

#include "utils.h"

cclt_compress_parameters initialize_compression_parameters();
cclt_compress_parameters parse_arguments(int argc, char* argv[]);
void cclt_compress_routine(char* input, char* output, cclt_compress_parameters* pars);

#endif
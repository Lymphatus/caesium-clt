#ifndef CCLT_COMPRESSHELPER
#define CCLT_COMPRESSHELPER

#include "utils.h"

cclt_parameters initialize_compression_parameters();
cclt_parameters parse_arguments(int argc, char* argv[]);
int cclt_compress_routine(char* input, char* output, cclt_parameters* pars); //Returns -1 if the file type is unknown

#endif

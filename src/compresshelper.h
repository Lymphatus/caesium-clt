#ifndef CCLT_COMPRESSHELPER
#define CCLT_COMPRESSHELPER

#include "utils.h"

cclt_parameters initialize_compression_parameters();
cclt_parameters parse_arguments(int argc, char* argv[]);
int cclt_compress_routine(char* input, char* output, cclt_parameters* pars); //Returns -1 if the file type is unknown
void cclt_start(cclt_parameters* pars, off_t* i_t_size, off_t* o_t_size);
//TODO Maybe it's better to return a int
void validate_parameters(cclt_parameters* pars);

#endif

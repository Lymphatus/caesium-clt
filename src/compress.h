#ifndef CCLT_COMPRESS
#define CCLT_COMPRESS

#include "utils.h"

void cclt_compress(char* output_file, unsigned char* image_buffer, cclt_compress_parameters* pars);
unsigned char* cclt_decompress(char* fileName, cclt_compress_parameters* pars);

#endif

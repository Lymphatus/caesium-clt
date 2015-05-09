#ifndef CCTL_COMPRESS
#define CCTL_COMPRESS

#include <jpeglib.h>

typedef struct cclt_compress_parameters {
	int quality;
	int width;
	int height;
	int smoothing_factor;
	J_COLOR_SPACE color_space;
	J_DCT_METHOD dct_method;
	int exif_copy;
} cclt_compress_parameters;

void cclt_compress(char* output_file, unsigned char* image_buffer);

#endif

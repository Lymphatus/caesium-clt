#ifndef CCLT_LOSSLESS
#define CCLT_LOSSLESS

int cclt_optimize(char* input_file, char* output_file, int exif_flag, char* exif_src);
struct jpeg_decompress_struct cclt_get_markers(char* input);

#endif

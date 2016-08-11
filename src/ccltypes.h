#ifndef CCLT_CCLTYPES
#define CCLT_CCLTYPES

typedef struct cclt_jpeg_parameters {
	int quality;
	int width;
	int height;
	int color_space;
	int dct_method;
	bool exif_copy;
	bool lossless;
	enum TJSAMP subsample;
} cclt_jpeg_parameters;

typedef struct cclt_png_parameters {
	int iterations;
	int iterations_large;
	int block_split_strategy;
	bool lossy_8;
	bool transparent;
	int auto_filter_strategy;
} cclt_png_parameters;

typedef struct cclt_parameters {
	cclt_jpeg_parameters jpeg;
	cclt_png_parameters png;

	char* output_folder;
	char** input_files;
	int input_files_count;
	bool recursive;
	bool structure;
} cclt_parameters;

enum image_type {
	JPEG,
	PNG,
	UNKN,
};

#endif

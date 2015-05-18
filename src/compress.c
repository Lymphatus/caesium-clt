#include <setjmp.h>
#include <stdio.h>
#include <jpeglib.h>
#include <turbojpeg.h>
#include <stdlib.h>

#include "compress.h"
#include "utils.h"

void cclt_compress(char* output_file, unsigned char* image_buffer, cclt_compress_parameters* pars)
{
	struct jpeg_compress_struct cinfo;

	struct jpeg_error_mgr jerr;
	FILE * outfile;
	JSAMPROW row_pointer[1];
	int row_stride;

	if ((outfile = fopen(output_file, "wb")) == NULL) {
		fprintf(stderr, "can't open %s\n", output_file);
		exit(1);
	}

	cinfo.err = jpeg_std_error(&jerr);

	jpeg_create_compress(&cinfo);
	jpeg_stdio_dest(&cinfo, outfile);

	cinfo.image_width = pars->width;
	cinfo.image_height = pars->height;
	cinfo.input_components = 3;
	cinfo.in_color_space = JCS_RGB;

	jpeg_set_defaults(&cinfo);

	cinfo.dct_method = pars->dct_method;
	cinfo.optimize_coding = TRUE;
	cinfo.smoothing_factor = pars->smoothing_factor;
	jpeg_set_quality(&cinfo, pars->quality, TRUE);
	jpeg_set_colorspace(&cinfo, JCS_RGB);



	jpeg_start_compress(&cinfo, TRUE);

	row_stride = pars->width * 3;


	while (cinfo.next_scanline < cinfo.image_height) {

	//printf("%d%%\r", cinfo.next_scanline * 100 / cinfo.image_height);

		row_pointer[0] = &image_buffer[cinfo.next_scanline * row_stride];
		(void) jpeg_write_scanlines(&cinfo, row_pointer, 1);
	}
	//printf("%d%%\n", cinfo.next_scanline * 100 / cinfo.image_height);


	jpeg_finish_compress(&cinfo);
	fclose(outfile);


	jpeg_destroy_compress(&cinfo);

}

unsigned char* cclt_decompress(char* fileName, cclt_compress_parameters* pars) {

    FILE *file = NULL;
    int res = 0;
    long int sourceJpegBufferSize = 0;
    unsigned char* sourceJpegBuffer = NULL;
    tjhandle tjDecompressHandle;
    int fileWidth = 0, fileHeight = 0, jpegSubsamp = 0;

    file = fopen(fileName, "rb");
    res = fseek(file, 0, SEEK_END);
    sourceJpegBufferSize = ftell(file);
    sourceJpegBuffer = tjAlloc(sourceJpegBufferSize);

    res = fseek(file, 0, SEEK_SET);
    res = fread(sourceJpegBuffer, (long)sourceJpegBufferSize, 1, file);
    tjDecompressHandle = tjInitDecompress();
    res = tjDecompressHeader2(tjDecompressHandle, sourceJpegBuffer, sourceJpegBufferSize, &fileWidth, &fileHeight, &jpegSubsamp);

    int destWidth = fileWidth;
    int destHeight = fileHeight;
    pars->color_space = jpegSubsamp;



    unsigned char* temp = tjAlloc(destHeight * destWidth * tjPixelSize[TJPF_RGB]);


    res = tjDecompress2(tjDecompressHandle,
                                 sourceJpegBuffer,
                                 sourceJpegBufferSize,
                                 temp,
                                 destWidth,
                                 0,
                                 destHeight,
                                 TJPF_RGB,
                                 TJFLAG_ACCURATEDCT);

    pars->width = destWidth;
    pars->height = destHeight;

    return temp;
}

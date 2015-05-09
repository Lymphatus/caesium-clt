#include <setjmp.h>
#include <stdio.h>
#include <jpeglib.h>
#include <stdlib.h>

#include "compress.h"

void cclt_compress(char* output_file, unsigned char* image_buffer)
{
  struct jpeg_compress_struct cinfo;

  struct jpeg_error_mgr jerr;
  FILE * outfile;               /* target file */
  JSAMPROW row_pointer[1];      /* pointer to JSAMPLE row[s] */
  int row_stride;               /* physical row width in image buffer */

  if ((outfile = fopen(output_file, "wb")) == NULL) {
    fprintf(stderr, "can't open %s\n", output_file);
    exit(1);
  }

  cinfo.err = jpeg_std_error(&jerr);

  jpeg_create_compress(&cinfo);
  jpeg_stdio_dest(&cinfo, outfile);

  cinfo.image_width = 80;
  cinfo.image_height = 80;
  cinfo.input_components = 3;
  cinfo.in_color_space = JCS_RGB;

  jpeg_set_defaults(&cinfo);

  cinfo.dct_method = JDCT_FLOAT;
  cinfo.optimize_coding = TRUE;
  cinfo.smoothing_factor = 50;
  jpeg_set_quality(&cinfo, 80, TRUE );
  jpeg_set_colorspace(&cinfo, JCS_RGB);

  

  jpeg_start_compress(&cinfo, TRUE);

	//TODO cambia
  row_stride = 80 * 3;//image_width * 3;


  while (cinfo.next_scanline < cinfo.image_height) {
  
  	printf("%d%\r", cinfo.next_scanline * 100 / cinfo.image_height);

    row_pointer[0] = &image_buffer[cinfo.next_scanline * row_stride];
    (void) jpeg_write_scanlines(&cinfo, row_pointer, 1);
  }
  printf("%d%\n", cinfo.next_scanline * 100 / cinfo.image_height);


  jpeg_finish_compress(&cinfo);
  fclose(outfile);


  jpeg_destroy_compress(&cinfo);

}

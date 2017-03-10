#ifndef CAESIUMCLT_ERROR_H
#define CAESIUMCLT_ERROR_H

void display_error(error_level level, int code);

const char *get_error_message(int code);

#endif //CAESIUMCLT_ERROR_H

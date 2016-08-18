#ifndef CCLT_ERROR
#define CCLT_ERROR

#include <stdbool.h>

void trigger_error(int code, bool is_critical, ...);

#endif

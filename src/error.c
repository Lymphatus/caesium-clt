#include "error.h"

#include <stdio.h>
#include <stdlib.h>
#include <stdarg.h>

/*
 * We could leave error messages where they happens,
 * but I want a more centralized way to track what went wrong
 */

#define parse_error_level(level) ((level) ? "ERROR" : "WARNING")

void trigger_error(int code, bool is_critical, ...) {
    va_list args;
    va_start(args, is_critical);

    fprintf(stderr, "%s - %d: ",
            parse_error_level(is_critical),
            code);

    switch (code) {
    case 1:
        fprintf(stderr,
                "-l option can't be used with -q. Either use one or the other.");
        break;
    case 2:
        fprintf(stderr,
                "Either -l or -q must be set.");
        break;
    case 3:
        fprintf(stderr,
                "Quality must be within a [1-100] range.");
        break;
    case 4:
        fprintf(stderr,
                "No -o option pointing to the destination folder.");
        break;
    case 5:
        fprintf(stderr,
                "Failed to create output directory. Permission issue?");
        break;
    case 6:
        vfprintf(stderr,
                 "Option -%c requires an argument.", args);
        break;
    case 9:
        fprintf(stderr,
                "No input files.");
        break;
    case 11:
        vfprintf(stderr,
                 "Failed to get input file stats: %s", args);
        break;
    case 12:
        vfprintf(stderr,
                 "Failed to get output file stats: %s", args);
        break;
    case 13:
        vfprintf(stderr,
                 "Failed to open file (markers): %s", args);
        break;
    case 16:
        vfprintf(stderr,
                 "Failed to open PNG file: %s", args);
        break;
    case 17:
        fprintf(stderr,
                "Error while optimizing PNG.");
        break;
    case 18:
        vfprintf(stderr,
                 "Error while writing PNG: %s", args);
    case 20:
        fprintf(stderr,
                "Found folder along with input files.");
    case 100:
        vfprintf(stderr,
                 "Unknown option `-%c'.", args);
        break;
    case 101:
        vfprintf(stderr,
                 "Unknown option character `\\x%x'.", args);
        break;
    case 102:
        fprintf(stderr,
                "Parameter expected.");
        break;
    case 103:
        fprintf(stderr,
                "Folder found, skipping all other inputs.");
        break;
    case 104:
        vfprintf(stderr,
                 "Unknown file type: %s", args);
        break;
    case 105:
        vfprintf(stderr,
                 "Failed to open file (input): %s", args);
        break;
    case 106:
        vfprintf(stderr,
                 "Failed to open file (output): %s", args);
        break;
    default:
        //Every unlisted code is critical
        is_critical = true;
        fprintf(stderr,
                "Cs-137 spreading out. Good luck.");
        break;
    }

    fprintf(stderr, "\n");

    va_end(args);

    if (is_critical) {
        exit(EXIT_FAILURE);
    }
}

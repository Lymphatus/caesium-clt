/*
 *
 * Copyright 2018 Matteo Paonessa
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */

#include <stdio.h>
#include <caesium.h>
#include <stdlib.h>

#include "error.h"

void display_error(error_level level, int code) {
    char *error_level = ((level) ? "[WARNING]" : "[ERROR]");
    fprintf(stderr, "%s %d: %s\n",
            error_level,
            code,
            get_error_message(code));
    if (level == ERROR) {
        exit(-code);
    }
}

const char *get_error_message(int code) {
    switch (code) {
        //Generic errors
        case 1:
            return "Invalid quality value. Must be between [0-100].";
        case 2:
            return "Unrecognized option.";
        case 3:
            return "Empty input folder.";
        case 4:
            return "Cannot keep folder structure providing multiple input files.";
        case 5:
            return "Cannot create output folder.";
        case 6:
            return "Cannot check if is a directory.";
        case 7:
            return "Cannot calculate file size";
        case 8:
            return "Input folder provided. Skipping all other inputs.";
        case 9:
            return "Input files provided. Cannot mix them with a folder.";
        case 10:
            return "-R has no effects on files.";
        case 11:
            return "-S has no effect without -R.";
        case 12:
            return "Cannot set output folder inside the input one";
        case 13:
            return "Scale factor must be between (0, 1.0]. Setting it to 1.0.";
        case 14:
            return "Scale factor parsing error.";
        case 15:
            return "Overwrite policy value is invalid. Using 'all'.";

        default:
            return "Unrecognized error.";
    }
}
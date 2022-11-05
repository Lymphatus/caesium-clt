/*
 *
 * Copyright 2019 Matteo Paonessa
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

#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <errno.h>

#ifdef _WIN32
#include <direct.h>
#endif

#include "helper.h"
#include "vendor/optparse.h"
#include "utils.h"
#include "config.h"
#include "error.h"
#include "shared.h"

cclt_options parse_arguments(char **argv, C_CSParameters *options) {
    struct optparse opts;
    //Initialize application options
    cclt_options parameters = {NULL, "", "", false, false, 0, 0, 0, false, all};

    //Parse command line args
    struct optparse_long longopts[] = {
            {"quality",        'q', OPTPARSE_REQUIRED},
            {"exif",           'e', OPTPARSE_NONE},
            {"output",         'o', OPTPARSE_REQUIRED},
            {"scale",          's', OPTPARSE_REQUIRED},
            {"recursive",      'R', OPTPARSE_NONE},
            {"keep-structure", 'S', OPTPARSE_NONE},
            {"overwrite",      'O', OPTPARSE_REQUIRED},
            {"dry-run",        'd', OPTPARSE_NONE},
            {"quiet",          'Q', OPTPARSE_NONE},
            {"version",        'v', OPTPARSE_NONE},
            {"help",           'h', OPTPARSE_NONE},
            {0}
    };
    optparse_init(&opts, argv);
    int option;
    int quality = 0;
    while ((option = optparse_long(&opts, longopts, NULL)) != -1) {
        switch (option) {
            case 'q':
                quality = (int) strtol(opts.optarg, (char **) NULL, 10);
                if (quality < 0 || quality > 100) {
                    display_error(ERROR, 1);
                }
                if (quality == 0) {
                    options->optimize = true;
                } else {
                    options->jpeg_quality = quality;
                    options->png_quality = quality;
                    options->webp_quality = quality;
                    options->gif_quality = quality;
                }
                break;
            case 'e':
                options->keep_metadata = true;
                break;
            case 'o':
                if (opts.optarg[0] == '~') {
                    snprintf(parameters.output_folder, strlen(opts.optarg) + 1, "%s", opts.optarg);
                } else {
#ifdef _WIN32
                    _fullpath(parameters.output_folder, opts.optarg, MAX_PATH_SIZE);
#else
                    char *computedPath = realpath(opts.optarg, parameters.output_folder);
                    if (computedPath == NULL) {
                        //Folder does not exists and may just fail on some systems, like Docker Alpine
                        if (errno == 2) {
                            if (mkpath(opts.optarg) == 0) {
                                computedPath = realpath(opts.optarg, parameters.output_folder);
                                if (computedPath == NULL) {
                                    //Just throw an error here
                                    display_error(ERROR, 16);
                                }
                            } else {
                                display_error(ERROR, 17);
                            }
                        } else {
                            display_error(ERROR, 16);
                        }
                    }
#endif
                }
                int pathlen = (int) strlen(parameters.output_folder);
                if (parameters.output_folder[pathlen - 1] != '/' &&
                    parameters.output_folder[pathlen - 1] != '\\') {
                    // append the extra slash/backslash
#ifdef _WIN32
                    snprintf(parameters.output_folder + pathlen, 2, "\\");
#else
                    snprintf(parameters.output_folder + pathlen, 2, "/");
#endif
                }
                break;
            case 'R':
                parameters.recursive = true;
                break;
            case 'S':
                parameters.keep_structure = true;
                break;
            case 'O':
                parameters.overwrite = parse_overwrite_policy(opts.optarg);
                break;
            case 'd':
                parameters.dry_run = true;
                break;
            case 'v':
                print_to_console(stdout, 1, "%d.%d.%d\n", VERSION_MAJOR, VERSION_MINOR, VERSION_PATCH);
                exit(EXIT_SUCCESS);
            case 'Q':
                verbose = 0;
                break;
            case 'h':
                print_help();
                break;
            case '?':
            default:
                print_to_console(stderr, verbose, "%s: %s\n", argv[0], opts.errmsg);
                display_error(ERROR, 2);
        }
    }
    //Remaining arguments
    char *arg;
    bool files_flag = false, folders_flag = false;
    char resolved_path[MAX_PATH_SIZE];

    print_to_console(stdout, verbose, "%s\n", "Collecting files...");

    while ((arg = optparse_arg(&opts))) {
        if (folders_flag) {
            display_error(WARNING, 8);
            break;
        }

        //Check if it's a directory and add its content
        if (arg[0] == '~' && is_directory(arg)) {
            if (arg[strlen(arg) - 1] == '/' || arg[strlen(arg) - 1] == '\\') {
                snprintf(resolved_path, strlen(arg), "%s", arg);
            } else {
#ifdef _WIN32
                snprintf(resolved_path, strlen(arg) + 1, "%s\\", arg);
#else
                snprintf(resolved_path, strlen(arg) + 1, "%s/", arg);
#endif
            }
        } else {
#ifdef _WIN32
            _fullpath(resolved_path, arg, MAX_PATH_SIZE);
#else
            realpath(arg, resolved_path);
#endif
        }

        if (is_directory(resolved_path)) {
            if (!files_flag) {
                folders_flag = true;
                size_t len = strlen(resolved_path);
                if (resolved_path[len - 1] != '/' && resolved_path[strlen(resolved_path) - 1] != '\\') {
#ifdef _WIN32
                    resolved_path[len] = '\\';
#else
                    resolved_path[len] = '/';
#endif
                    resolved_path[len + 1] = '\0';
                }

                snprintf(parameters.input_folder, strlen(resolved_path) + 1, "%s", resolved_path);
                scan_folder(resolved_path, &parameters, parameters.recursive);
                if (parameters.files_count == 0) {
                    display_error(WARNING, 3);
                }
            } else {
                display_error(WARNING, 9);
            }
        } else {
            files_flag = true;
            parameters.input_files = realloc(parameters.input_files, (parameters.files_count + 1) * sizeof(char *));
            parameters.input_files[parameters.files_count] = malloc((strlen(arg) + 1) * sizeof(char));
            snprintf(parameters.input_files[parameters.files_count], strlen(arg) + 1, "%s", arg);
            parameters.files_count++;
        }
    }

    //Check if the output folder is a subfolder of the input to avoid infinite loops
    //but just if the -R option is set
    //However, if the folders are the same, we can let it go as it will overwrite the files
    if (folders_flag) {
        if (strstr(parameters.output_folder, parameters.input_folder) != NULL
            && strcmp(parameters.output_folder, parameters.input_folder) != 0
            && parameters.recursive) {
            display_error(ERROR, 12);
        }
    }

    //-R and -S set warnings
    if (parameters.recursive && !folders_flag) {
        display_error(WARNING, 10);
        parameters.recursive = false;
    }
    if (!parameters.recursive && parameters.keep_structure) {
        display_error(WARNING, 11);
        parameters.keep_structure = false;
    }
    //If there are files and folders, we cannot keep the structure
    if (parameters.keep_structure && (!folders_flag && parameters.files_count > 1)) {
        display_error(WARNING, 4);
        parameters.keep_structure = false;
    }
    return parameters;
}

int start_compression(cclt_options *options, struct C_CSParameters parameters) {
    int compressed_files = 0;
    off_t input_file_size;
    off_t output_file_size;
    //Create the output folder if it does not exist
    if (mkpath(options->output_folder) == -1) {
        display_error(ERROR, 5);
    }

    for (int i = 0; i < options->files_count; i++) {
        char *filename = get_filename(options->input_files[i]);
        char *output_full_path = NULL;
        char *original_output_full_path = NULL;
        bool overwriting = false;
        off_t file_size;
        //If we don't need to keep the structure, we put all the files in one folder by just the filename
        if (!options->keep_structure) {
            output_full_path = malloc((strlen(filename) + strlen(options->output_folder) + 1) * sizeof(char));
            snprintf(output_full_path, (strlen(filename) + strlen(options->output_folder) + 1), "%s%s",
                     options->output_folder, filename);
        } else {
            /*
             * Otherwise, we need to compute the whole directory structure
             * We are sure we have a folder only as input, so that's the root
             * Just compute the subfolders without the filename, make them and append the filename
             * A piece of cake <3
            */

            size_t index = strspn(options->input_folder, options->input_files[i]);
            size_t size = strlen(options->input_files[i]) - index - strlen(filename);
            char output_full_folder[strlen(options->output_folder) + size + 1];

            snprintf(output_full_folder, strlen(options->output_folder) + size + 1, "%s%s",
                     options->output_folder, &options->input_files[i][index]);
            output_full_path = malloc((strlen(output_full_folder) + strlen(filename) + 1) * sizeof(char));
            snprintf(output_full_path, strlen(output_full_folder) + strlen(filename) + 1, "%s%s",
                     output_full_folder,
                     filename);

            mkpath(output_full_folder);
        }

        //Calculating the total input file size, ignoring if we are going to skip them later
        file_size = get_file_size(options->input_files[i]);
        if (file_size == 0) {
            //We could not open the file
            continue;
        }
        input_file_size = file_size;
        options->input_total_size += input_file_size;

        //If the file already exist, create a temporary file
        bool f_exists = file_exists(output_full_path);
        if (f_exists) {
            if (options->overwrite == none) {
                print_to_console(stdout, verbose, "[SKIPPED] %s\n", output_full_path);
                options->output_total_size += get_file_size(output_full_path);
                goto free_and_go_on_with_next_file;
            } else if (options->overwrite == prompt) {
                print_to_console(stdout, verbose, "Overwrite %s? [y/n]\n", output_full_path);
                int prompt = getchar();
                if (prompt == '\n') {
                    prompt = getchar();
                }
                if (prompt != 'y' && prompt != 'Y') {
                    print_to_console(stdout, verbose, "[SKIPPED] %s\n", output_full_path);
                    options->output_total_size += get_file_size(output_full_path);
                    goto free_and_go_on_with_next_file;
                }
            }

            original_output_full_path = strdup(output_full_path);
            output_full_path = realloc(output_full_path, (strlen(output_full_path) + 4) * sizeof(char));
            snprintf(output_full_path, (strlen(original_output_full_path) + 4), "%s.cs", original_output_full_path);
            overwriting = true;
        }

        print_to_console(stdout, verbose, "(%d/%d) %s -> %s\nCompressing...",
                         i + 1,
                         options->files_count,
                         filename,
                         f_exists ? original_output_full_path : output_full_path);
        fflush(stdout);
        //Prevent compression if running in dry mode
        if (!options->dry_run) {
            if (c_compress(options->input_files[i], output_full_path, parameters)) {
                compressed_files++;
                output_file_size = get_file_size(output_full_path);

                char *human_input_size = get_human_size(input_file_size);
                char *human_output_size = get_human_size(output_file_size);

                if (options->overwrite == bigger && get_file_size(original_output_full_path) <= output_file_size) {
                    print_to_console(stdout, verbose, "Resulting file is bigger. Skipping.\n");
                    remove(output_full_path);
                    options->output_total_size += get_file_size(original_output_full_path);
                    goto free_and_go_on_with_next_file;
                }
                options->output_total_size += output_file_size;
                print_to_console(stdout, verbose, "\r%s -> %s [%.2f%%]\n",
                                 human_input_size,
                                 human_output_size,
                                 ((float) output_file_size - (float) input_file_size) * 100 / (float) input_file_size);
            } else {
                print_to_console(stderr, verbose, "\nCompression failed\n");
                options->input_total_size -= get_file_size(options->input_files[i]);
            }
        }

        //Rename if we were overwriting
        if (overwriting && !options->dry_run) {
#ifdef _WIN32
            remove(original_output_full_path);
#endif
            rename(output_full_path, original_output_full_path);
        }

        free_and_go_on_with_next_file:
        free(original_output_full_path);
        free(output_full_path);
    }

    return compressed_files;
}



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

#ifndef CAESIUMCLT_ERROR_H
#define CAESIUMCLT_ERROR_H

typedef enum error_level
{
    ERROR = 0,
    WARNING = 1
} error_level;

void display_error(error_level level, int code);

const char *get_error_message(int code);

#endif //CAESIUMCLT_ERROR_H

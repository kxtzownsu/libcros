/*
 * Copyright 2010 Google LLC
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *    * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *    * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *    * Neither the name of Google LLC nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include "lib/flashrom.h"

/* The best choice is PATH_MAX. However, it leads portibility issue.
 * So, define a long enough length here. */
#define CMD_BUF_SIZE (4096)

static uint8_t flashrom_cmd[] = "flashrom";

/* The argument for flashrom.
 *   bus=spi: The VPD data are stored in BIOS flash, which is attached
 *            to the SPI bus.
 */
static uint8_t flashrom_arguments[] = " -p internal ";

int flashromFullRead(const char* full_file) {
  char cmd[CMD_BUF_SIZE];
  int ret = 0;

  snprintf(cmd, sizeof(cmd), "%s %s -r '%s' >/dev/null 2>&1",
           flashrom_cmd, flashrom_arguments, full_file);
  ret = system(cmd);
  if (ret == 0)
    return FLASHROM_OK;
  else
    return FLASHROM_FAIL;
}

int flashromPartialRead(const char* part_file, const char* full_file,
                        const char* partition_name) {
  char cmd[CMD_BUF_SIZE];
  int ret = 0;

  snprintf(cmd, sizeof(cmd), "%s %s -i FMAP -i '%s':'%s' "
                             "-r '%s' >/dev/null 2>&1",
           flashrom_cmd, flashrom_arguments,
           partition_name, part_file, full_file);
  ret = system(cmd);
  if (ret == 0)
    return FLASHROM_OK;
  else
    return FLASHROM_FAIL;
}

int flashromPartialWrite(const char* part_file, const char* full_file,
                         const char* partition_name) {
  char cmd[CMD_BUF_SIZE];
  int ret = 0;

  /* write it back */
  snprintf(cmd, sizeof(cmd),
           "%s %s -i '%s':'%s' -w '%s' --noverify-all >/dev/null 2>&1",
           flashrom_cmd, flashrom_arguments,
           partition_name, part_file, full_file);
  ret = system(cmd);

  if (ret == 0)
    return FLASHROM_OK;
  else
    return FLASHROM_FAIL;
}

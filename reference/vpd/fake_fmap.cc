// Copyright 2023 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include <fmap.h>
extern "C" {
#include <libflashrom.h>
}  // extern "C"
#include <stdarg.h>
#include <stddef.h>
#include <stdint.h>
#include <string.h>

#include <vector>

#include "base/check.h"
#include "base/check_op.h"
#include "base/files/file.h"
#include "base/logging.h"
#include "base/strings/stringprintf.h"

#include "vpd/fake_fmap.h"

namespace vpd {

namespace {
int FlashromLogging(enum flashrom_log_level level,
                    const char* format,
                    va_list va_args) {
#pragma GCC diagnostic ignored "-Wformat-nonliteral"
  if (level > FLASHROM_MSG_INFO)
    return 0;
  LOG(INFO) << base::StringPrintV(format, va_args);
#pragma GCC diagnostic warning "-Wformat-nonliteral"
  return 0;
}

};  // namespace

void FakeFmap::FillFlashFile(const base::FilePath& flash_path,
                             unsigned int size,
                             bool empty_fmap) {
  base::File file(flash_path, base::File::FLAG_CREATE | base::File::FLAG_WRITE);
  // NB: filled with zeroes.
  CHECK(file.SetLength(size));

  const uint8_t* flash_name = reinterpret_cast<const uint8_t*>("FAKE_FMAP");
  const uint8_t* fmap_name = reinterpret_cast<const uint8_t*>("FMAP");
  const uint8_t* ro_name = reinterpret_cast<const uint8_t*>("RO_VPD");
  const uint8_t* rw_name = reinterpret_cast<const uint8_t*>("RW_VPD");

  auto fmap = fmap_create(0, size, flash_name);
  CHECK(fmap);

  CHECK_LT(0, fmap_append_area(&fmap, 0, 16 * 1024, fmap_name, 0));
  if (!empty_fmap) {
    CHECK_LT(0, fmap_append_area(&fmap, 16 * 1024, 16 * 1024, ro_name, 0));
    CHECK_LT(0, fmap_append_area(&fmap, 32 * 1024, 16 * 1024, rw_name, 0));
  }

  // Prep flash.bin with a bare FMAP and the appropriate regions.
  struct flashrom_programmer* programmer;
  struct flashrom_flashctx* flashctx;
  struct flashrom_layout* layout;

  CHECK_EQ(0, flashrom_init(1 /* selfcheck */));
  flashrom_set_log_callback(FlashromLogging);
  CHECK_EQ(0, flashrom_programmer_init(
                  &programmer, "dummy",
                  GetProgrammerParams(flash_path, size).c_str()));
  CHECK_EQ(0, flashrom_flash_probe(&flashctx, programmer, NULL));
  CHECK_EQ(0, flashrom_layout_read_fmap_from_buffer(
                  &layout, flashctx, reinterpret_cast<uint8_t*>(fmap),
                  fmap_size(fmap)));
  flashrom_layout_set(flashctx, layout);
  CHECK_EQ(0, flashrom_layout_include_region(layout, "FMAP"));
  size_t flash_size = flashrom_flash_getsize(flashctx);
  std::vector<uint8_t> flash(flash_size, 0xff);
  unsigned int region_start, region_len;
  CHECK_EQ(0, flashrom_layout_get_region_range(layout, "FMAP", &region_start,
                                               &region_len));
  memcpy(flash.data() + region_start, reinterpret_cast<uint8_t*>(fmap),
         fmap_size(fmap));
  CHECK_EQ(0,
           flashrom_image_write(flashctx, flash.data(), flash.size(), nullptr));

  flashrom_layout_release(layout);
  flashrom_flash_release(flashctx);
  flashrom_programmer_shutdown(programmer);

  fmap_destroy(fmap);
}

}  // namespace vpd

// Copyright 2023 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include "vpd/flashrom.h"

#include <assert.h>
extern "C" {
#include <libflashrom.h>
}  // extern "C"
#include <stdarg.h>
#include <stddef.h>
#include <stdint.h>

#include <algorithm>
#include <optional>
#include <string>
#include <vector>

#include "absl/log/log.h"
#include "absl/strings/strip.h"

#include "vpd/types.h"
#include "vpd/util/string.h"

namespace vpd {

namespace {

int FlashromLog(enum flashrom_log_level level,
                const char* format,
                va_list va_args) {
  // Too noisy. Just drop.
  if (level > FLASHROM_MSG_INFO) {
    return 0;
  }

  std::string msg = vpd::util::StringPrintV(format, va_args);

  // libflashrom sends us printf()-style messages, including newlines. Strip
  // those, a LOG doesn't expect that.
  // NB: libflashrom also splits lines sometimes (and omits the newline), which
  // means we'll log a split line.
  msg = absl::StripSuffix(msg, "\n");

  if (level >= FLASHROM_MSG_INFO) {
    // Flashrom is still pretty noisy at INFO level.

    // TODO(wdzeng): Uncomment this after we migrate to use absl-20240116.
    // VLOG(1) << msg;
  } else if (level >= FLASHROM_MSG_WARN) {
    LOG(WARNING) << msg;
  } else {
    LOG(ERROR) << msg;
  }
  return 0;
}

const char* PartitionName(VpdRegion region) {
  switch (region) {
    case VpdRo:
      return "RO_VPD";
    case VpdRw:
      return "RW_VPD";
    default:
      assert(false);
  }
}

// For RAII handling of flashrom_layout_{in,ex}clude_region().
class FlashromLayoutRegionHandle {
 public:
  FlashromLayoutRegionHandle(const FlashromLayoutRegionHandle&) = delete;
  FlashromLayoutRegionHandle& operator=(const FlashromLayoutRegionHandle&) =
      delete;

  // This should be private, but std::make_optional templating doesn't seem to
  // like that below.
  FlashromLayoutRegionHandle(struct flashrom_layout* layout,
                             const std::string& name)
      : layout_(layout), name_(name) {}

  ~FlashromLayoutRegionHandle() {
    flashrom_layout_exclude_region(layout_, name_.c_str());
  }

  static std::optional<FlashromLayoutRegionHandle> IncludeRegion(
      struct flashrom_layout* layout, const std::string& name) {
    if (flashrom_layout_include_region(layout, name.c_str()))
      return {};

    return std::make_optional<FlashromLayoutRegionHandle>(layout, name);
  }

 private:
  struct flashrom_layout* layout_;
  std::string name_;
};

}  // namespace

bool Flashrom::Init() {
  if (flashrom_init(1 /* selfcheck */)) {
    LOG(ERROR) << "flashrom initialization error";
    return false;
  }
  flashrom_set_log_callback(FlashromLog);

  if (flashrom_programmer_init(&programmer_, programmer_name_.c_str(),
                               programmer_params_.c_str())) {
    LOG(ERROR) << "programmer initialization failed";
    return false;
  }

  int rc = flashrom_flash_probe(&flashctx_, programmer_, NULL);
  if (rc == 3) {
    LOG(ERROR) << "flash probe failed: multiple chips were found";
    return false;
  }
  if (rc == 2) {
    LOG(ERROR) << "flash probe failed: no chip was found";
    return false;
  }
  if (rc != 0) {
    LOG(ERROR) << "flash probe failed: unknown error: " << rc;
    return false;
  }

  size_t flash_size = flashrom_flash_getsize(flashctx_);
  rc = flashrom_layout_read_fmap_from_rom(&layout_, flashctx_, 0, flash_size);
  if (rc == 3) {
    LOG(ERROR) << "fmap not implemented";
    return false;
  }
  if (rc == 2) {
    LOG(ERROR) << "failed to read fmap";
    return false;
  }
  if (rc) {
    LOG(ERROR) << "unknown flashrom/fmap error: " << rc;
    return false;
  }

  initialized_ = true;

  return true;
}

Flashrom::~Flashrom() {
  flashrom_layout_release(layout_);
  flashrom_flash_release(flashctx_);
  flashrom_programmer_shutdown(programmer_);
}

bool Flashrom::GetPartitionDimensions(VpdRegion region,
                                      uint32_t* offset,
                                      uint32_t* len) {
  *offset = 0;
  *len = 0;

  if (!initialized_) {
    if (!Init()) {
      LOG(ERROR) << "failed to initialized flashrom context";
      return false;
    }
  }
  unsigned int region_start, region_len;
  int rc = flashrom_layout_get_region_range(layout_, PartitionName(region),
                                            &region_start, &region_len);
  if (rc) {
    LOG(ERROR) << "failed to find region " << PartitionName(region);
    return false;
  }

  *offset = region_start;
  *len = region_len;

  return true;
}

bool Flashrom::Write(VpdRegion region, const std::vector<uint8_t>& blob) {
  if (!initialized_) {
    if (!Init()) {
      LOG(ERROR) << "failed to initialized flashrom context";
      return false;
    }
  }

  auto region_handle =
      FlashromLayoutRegionHandle::IncludeRegion(layout_, PartitionName(region));
  if (!region_handle) {
    LOG(ERROR) << "can't find flash region: " << PartitionName(region);
    return false;
  }

  flashrom_layout_set(flashctx_, layout_);

  size_t flash_size = flashrom_flash_getsize(flashctx_);
  // Yes, libflashrom really requires a flash-sized buffer just to flash one
  // region.
  std::vector<uint8_t> flash(flash_size, 0xff);
  uint32_t part_offset, part_len;
  if (!GetPartitionDimensions(region, &part_offset, &part_len)) {
    LOG(ERROR) << "failed to retrieve partition dimensions";
    return false;
  }
  std::copy(blob.begin(), blob.end(), flash.begin() + part_offset);

  // Tell flashrom to read back the written region for verification.
  // NB: without this, flashrom may fail to report write protection errors.
  flashrom_flag_set(flashctx_, FLASHROM_FLAG_VERIFY_AFTER_WRITE, true);

  int rc = flashrom_image_write(flashctx_, flash.data(), flash.size(), nullptr);
  if (rc == 4) {
    LOG(ERROR) << "flashrom blob doesn't fit flash chip";
    return false;
  }
  if (rc == 3) {
    LOG(ERROR) << "flashrom write didn't take hold?";
    return false;
  }
  if (rc == 2) {
    LOG(ERROR) << "flashrom write failed; CONTENTS MAY HAVE CHANGED";
    return false;
  }
  if (rc) {
    LOG(ERROR) << "unknown flashrom error";
    return false;
  }

  return true;
}

std::optional<std::vector<uint8_t>> Flashrom::Read(VpdRegion region) {
  if (!initialized_) {
    if (!Init()) {
      LOG(ERROR) << "failed to initialized flashrom context";
      return {};
    }
  }

  auto region_handle =
      FlashromLayoutRegionHandle::IncludeRegion(layout_, PartitionName(region));
  if (!region_handle) {
    LOG(ERROR) << "can't find flash region: " << PartitionName(region);
    return {};
  }

  flashrom_layout_set(flashctx_, layout_);

  size_t flash_size = flashrom_flash_getsize(flashctx_);
  // Really? libflashrom requires constructing a full "image"??
  std::vector<uint8_t> flash(flash_size, 0xff);
  uint32_t part_offset, part_len;
  if (!GetPartitionDimensions(region, &part_offset, &part_len)) {
    LOG(ERROR) << "failed to retrieve partition dimensions";
    return {};
  }

  int rc = flashrom_image_read(flashctx_, flash.data(), flash.size());
  if (rc == 2) {
    LOG(ERROR) << "flashrom read buffer is too small";
    return {};
  }
  if (rc) {
    LOG(ERROR) << "unknown flashrom error";
    return {};
  }

  return std::vector<uint8_t>(flash.begin() + part_offset,
                              flash.begin() + part_offset + part_len);
}

}  // namespace vpd

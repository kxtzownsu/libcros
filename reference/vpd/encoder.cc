// Copyright 2023 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include "vpd/encoder.h"

#include <stddef.h>
#include <stdint.h>
#include <string.h>
#include <uuid/uuid.h>

#include <algorithm>
#include <array>
#include <map>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "absl/log/check.h"
#include "absl/log/log.h"
#include "absl/strings/escaping.h"
#include "absl/strings/string_view.h"

extern "C" {
#include "vpd/include/lib/checksum.h"
#include "vpd/include/lib/vpd.h"
#include "vpd/include/lib/vpd_tables.h"
}  // extern "C"
#include "vpd/types.h"

namespace vpd {

namespace {

// The VPD format can support very large (up to 0x7FFFFFFF) entries, but this
// places an unreasonable strain on memory size, and is frankly impossible to
// fit on the flash devices VPD tends to be stored on. Let's just bail nicely
// if we're getting too large.
constexpr unsigned int kMaxBlobSize = 1024 * 1024 * 1024;

// We only look for EPS headers at this alignment.
constexpr unsigned int kEpsOffsetStride = 16;

enum {
  VPD_TYPE_TERMINATOR = 0,
  VPD_TYPE_STRING,
  VPD_TYPE_INFO = 0xfe,
  VPD_TYPE_IMPLICIT_TERMINATOR = 0xff,
};

std::vector<uint8_t> EncodeLen(size_t len) {
  std::vector<uint8_t> data;
  unsigned int shifting = len;
  unsigned int encoded_len;
  unsigned int reversed_7bits = 0;

  /* reverse the len for every 7-bit. The little endian. */
  for (encoded_len = 0; shifting; encoded_len++) {
    reversed_7bits = (reversed_7bits << 7) | (shifting & 0x7f);
    shifting >>= 7;
  }
  if (!encoded_len)
    encoded_len = 1; /* output at least 1 byte */

  data.resize(encoded_len);

  /* Output in reverse order, now big endian. */
  for (auto& it : data) {
    /* always set MORE flag */
    it = 0x80 | (reversed_7bits & 0x7f);
    reversed_7bits >>= 7;
  }
  data.back() &= 0x7f; /* clear the MORE flag in last byte */

  return data;
}

std::vector<uint8_t> EncodePair(const KeyVal& kv) {
  std::vector<uint8_t> data;

  // Encode key.
  data.push_back(VPD_TYPE_STRING);
  auto k = EncodeLen(kv.key.size());
  data.insert(data.end(), k.begin(), k.end());
  for (const auto& b : kv.key) {
    data.push_back(b);
  }

  // Encode value.
  auto v = EncodeLen(kv.value.size());
  data.insert(data.end(), v.begin(), v.end());
  for (const auto& b : kv.value) {
    data.push_back(b);
  }

  return data;
}

bool DecodeLength(const std::vector<uint8_t> blob,
                  uint32_t* decoded_length,
                  uint32_t* entry_length) {
  *entry_length = 0;

  uint8_t more;
  uint32_t i = 0;
  do {
    if (i >= blob.size())
      return false;

    more = blob[i] & 0x80;
    *entry_length <<= 7;
    *entry_length |= blob[i] & 0x7f;
    i++;
  } while (more);

  *decoded_length = i;
  return true;
}

std::optional<KeyVal> DecodePair(const std::vector<uint8_t>& blob,
                                 uint32_t* consumed) {
  KeyVal ret;
  *consumed = 0;

  uint32_t decoded_length, entry_length;
  if (!DecodeLength(std::vector<uint8_t>(blob.begin(), blob.end()),
                    &decoded_length, &entry_length)) {
    return {};
  }
  if (decoded_length > blob.size() - *consumed) {
    return {};
  }
  *consumed += decoded_length;
  if (entry_length > blob.size() - *consumed) {
    return {};
  }

  ret.key = std::string(reinterpret_cast<const char*>(blob.data()) + *consumed,
                        entry_length);
  *consumed += entry_length;

  if (!DecodeLength(std::vector<uint8_t>(blob.begin() + *consumed, blob.end()),
                    &decoded_length, &entry_length)) {
    return {};
  }
  if (decoded_length > blob.size() - *consumed) {
    return {};
  }
  *consumed += decoded_length;
  if (entry_length > blob.size() - *consumed) {
    return {};
  }

  ret.value = std::string(
      reinterpret_cast<const char*>(blob.data()) + *consumed, entry_length);
  *consumed += entry_length;

  return ret;
}

void VectorAppendArray(std::vector<uint8_t>& v, const uint8_t* p, size_t len) {
  v.insert(v.end(), &p[0], &p[len]);
}

std::vector<uint8_t> VpdType241(uint16_t handle,
                                std::string uuid,
                                std::string vendor,
                                std::string desc,
                                std::string variant,
                                uint32_t offset,
                                uint32_t size) {
  struct vpd_header* header;
  struct vpd_table_binary_blob_pointer* data;

  std::vector<uint8_t> ret;
  ret.resize(sizeof(*header) + sizeof(*data));
  header = reinterpret_cast<struct vpd_header*>(&ret[0]);
  data = reinterpret_cast<struct vpd_table_binary_blob_pointer*>(
      &ret[sizeof(*header)]);

  *header = (struct vpd_header){
      .type = VPD_TYPE_BINARY_BLOB_POINTER,
      .length = sizeof(*header) + sizeof(*data),
      .handle = handle,
  };
  *data = (struct vpd_table_binary_blob_pointer){
      .struct_major_version = 1,
      .struct_minor_version = 0,
      .major_version = 2,
      .minor_version = 0,
      .offset = offset,
      .size = size,
  };

  CHECK_GE(uuid_parse(uuid.c_str(), &data->uuid[0]), 0);

  int string_index = 1;
  // Vector may have resized/reallocated.
  data = reinterpret_cast<struct vpd_table_binary_blob_pointer*>(
      &ret[sizeof(*header)]);
  data->vendor = string_index++;
  // Include the nul terminator.
  VectorAppendArray(ret, reinterpret_cast<const uint8_t*>(vendor.c_str()),
                    vendor.size() + 1);
  // Vector may have resized/reallocated.
  data = reinterpret_cast<struct vpd_table_binary_blob_pointer*>(
      &ret[sizeof(*header)]);
  data->description = string_index++;
  VectorAppendArray(ret, reinterpret_cast<const uint8_t*>(desc.c_str()),
                    desc.size() + 1);
  // Vector may have resized/reallocated.
  data = reinterpret_cast<struct vpd_table_binary_blob_pointer*>(
      &ret[sizeof(*header)]);
  data->variant = string_index++;
  VectorAppendArray(ret, reinterpret_cast<const uint8_t*>(variant.c_str()),
                    variant.size() + 1);
  // Structure terminator.
  ret.push_back(0);

  return ret;
}

std::vector<uint8_t> VpdType127(uint16_t handle) {
  struct vpd_table_eot data = {
      .header =
          {
              .type = 127,
              .length = sizeof(data),
              .handle = handle,
          },
  };

  std::vector<uint8_t> ret;
  VectorAppendArray(ret, reinterpret_cast<const uint8_t*>(&data), sizeof(data));
  // double terminator
  ret.push_back(0);
  ret.push_back(0);

  return ret;
}

std::vector<uint8_t> VpdCreateEps(uint16_t structure_table_len,
                                  uint16_t num_structures,
                                  uint32_t eps_base) {
  struct vpd_entry eps = {
      .entry_length = sizeof(eps),
      .major_ver = CONFIG_EPS_VPD_MAJOR_VERSION,
      .minor_ver = CONFIG_EPS_VPD_MINOR_VERSION,
      // EPS revision based on version 2.1 or later.
      .entry_rev = 0,
      .table_length = structure_table_len,
      .table_address =
          eps_base + static_cast<uint32_t>(sizeof(eps)),  // + .entry_length
      .table_entry_count = num_structures,
      .bcd_revision =
          (CONFIG_EPS_VPD_MAJOR_VERSION << 4) | CONFIG_EPS_VPD_MINOR_VERSION,
      /* note: nothing done with EPS formatted area */
  };
  memcpy(eps.anchor_string, VPD_ENTRY_MAGIC, 4);

  /* Intermediate EPS (IEPS) stuff */
  memcpy(eps.inter_anchor_string, "_DMI_", 5);

  /* calculate IEPS checksum first, then the EPS checksum */
  eps.inter_anchor_cksum = zero8_csum(&eps.inter_anchor_string[0], 0xf);
  eps.entry_cksum =
      zero8_csum(reinterpret_cast<uint8_t*>(&eps), eps.entry_length);

  std::vector<uint8_t> ret;
  VectorAppendArray(ret, reinterpret_cast<const uint8_t*>(&eps), sizeof(eps));
  return ret;
}

std::vector<uint8_t> GenerateEpsAndTables(const Encoder::EncodingParams& params,
                                          unsigned int size_blob) {
  std::vector<uint8_t> ret;

  uint32_t eps_base = params.partition_offset;
  int num_structures = 0;

  // TODO(hungte) Once most systems have been updated to support VPD_TYPE_INFO
  // record, we can remove the +sizeof(google_vpd_info) hack.

  // Generate type 241 - VPD 2.0.
  auto vpd = VpdType241(
      num_structures++, GOOGLE_VPD_2_0_UUID, GOOGLE_VPD_2_0_VENDOR,
      GOOGLE_VPD_2_0_DESCRIPTION, GOOGLE_VPD_2_0_VARIANT,
      eps_base + GOOGLE_VPD_2_0_OFFSET + sizeof(struct google_vpd_info),
      size_blob);

  // Generate type 127.
  auto type127 = VpdType127(num_structures++);

  for (const auto& v : std::array{vpd, type127}) {
    ret.insert(ret.end(), v.begin(), v.end());
  }

  auto eps = VpdCreateEps(ret.size(), num_structures, eps_base);
  // Prepend EPS header.
  ret.insert(ret.begin(), eps.begin(), eps.end());

  return ret;
}

std::vector<uint8_t> GenerateVpdBlob(
    const std::map<std::string, std::string>& dict) {
  struct google_vpd_info* info;
  std::vector<uint8_t> data(sizeof(*info));

  for (const auto& pair : dict) {
    const auto& b = EncodePair(KeyVal{
        .key = pair.first,
        .value = pair.second,
    });
    data.insert(data.end(), b.begin(), b.end());
  }

  data.push_back(VPD_TYPE_TERMINATOR);

  info = reinterpret_cast<struct google_vpd_info*>(data.data());
  memcpy(info->header.magic, VPD_INFO_MAGIC, sizeof(info->header.magic));
  info->size = data.size() - sizeof(*info);

  return data;
}

// Compare with the SMBIOS signature ("_SM_").
//
// NB: This is a pretty basic validity check. We've never tried to do more, and
// it seems to be good enough.
bool IsEps(const std::vector<uint8_t>& blob) {
  if (blob.size() < sizeof(VPD_ENTRY_MAGIC) - 1)
    return false;
  return !memcmp(VPD_ENTRY_MAGIC, blob.data(), sizeof(VPD_ENTRY_MAGIC) - 1);
}

std::optional<std::map<std::string, std::string>> DecodeBlob(
    const std::vector<uint8_t>& blob) {
  std::map<std::string, std::string> ret;

  std::vector<uint8_t> tracking(blob.begin(), blob.end());
  while (true) {
    // Legacy utility didn't require a terminator.
    if (tracking.empty()) {
      return ret;
    }

    uint8_t type = tracking[0];
    switch (type) {
      case VPD_TYPE_TERMINATOR:
      case VPD_TYPE_IMPLICIT_TERMINATOR:
        return ret;
      case VPD_TYPE_INFO:
      case VPD_TYPE_STRING: {
        uint32_t consumed;
        auto kv = DecodePair(
            std::vector<uint8_t>(tracking.begin() + 1, tracking.end()),
            &consumed);
        if (!kv) {
          LOG(ERROR) << "failed to decode key/value pair";
          return {};
        }
        ret[kv->key] = kv->value;
        // TODO(wdzeng): Uncomment this after we migrate to use absl-20240116.
        // VLOG(1) << "Parsed: key = " << kv->key << ", value = " << kv->value;
        tracking = std::vector<uint8_t>(tracking.begin() + 1 + consumed,
                                        tracking.end());
        break;
      }
      default:
        LOG(ERROR) << "unexpected VPD entry type: " << int(type);
        return {};
    }
  }
}

// VpdType241Size() - return the size of type 241 structure table.
//
// Type 241 structure contains 3 variant length of string at end of table.
bool VpdType241Size(const std::vector<uint8_t>& blob, uint32_t* length) {
  const struct vpd_header* header =
      reinterpret_cast<const struct vpd_header*>(blob.data());
  if (blob.size() < sizeof(*header)) {
    return false;
  }
  if (header->type != VPD_TYPE_BINARY_BLOB_POINTER) {
    return false;
  }
  *length = sizeof(*header) + sizeof(struct vpd_table_binary_blob_pointer);

  unsigned int offs = header->length;
  if (offs >= blob.size()) {
    return false;
  }

  // Three variant-length strings.
  for (unsigned int i = 0; i < 3; i++) {
    for (auto it = blob.begin() + offs; it != blob.end(); it++) {
      offs++;
      (*length)++;
      // We're looking for a string terminator.
      if (*it == 0) {
        break;
      }
    }
    // We should still have at least a terminator left over, so we can't have
    // reached the end.
    if (offs >= blob.size()) {
      return false;
    }
    // Didn't find a string terminator.
    if (blob[offs - 1] != 0) {
      return false;
    }
  }

  // Additional null(0) to indicate end of set, so called structure terminator.
  // Refer to SMBIOS spec 3.1.3 Text Strings.
  (*length)++;

  return true;
}

}  // namespace

// static
std::optional<std::vector<uint8_t>> Encoder::Encode(
    const EncodingParams& params,
    const std::map<std::string, std::string>& dict) {
  if (params.eps_offset % kEpsOffsetStride != 0) {
    LOG(ERROR) << "unaligned EPS offset: " << params.eps_offset;
    return {};
  }

  auto blob = GenerateVpdBlob(dict);
  auto eps = GenerateEpsAndTables(params, blob.size());

  std::vector<uint8_t> data;

  // NB: We assume EPS, SPD (optional), and VPD-2.0 are laid out in order and
  // throw errors if these offsets don't align with that. Previous
  // implementations claimed some flexibility but probably didn't really
  // support it in practice.
  data.resize(params.eps_offset, 0xff);
  data.insert(data.end(), eps.begin(), eps.end());

  // NB: old implementations pretended this was configurable, but always used a
  // fixed constant.
  const uint32_t vpd_2_0_offset = GOOGLE_VPD_2_0_OFFSET;
  if (data.size() > vpd_2_0_offset) {
    LOG(ERROR) << "EPS/SPD overlap with VPD 2.0 blob: " << data.size()
               << " vs. " << vpd_2_0_offset;
    return {};
  }
  data.resize(vpd_2_0_offset, 0xff);
  data.insert(data.end(), blob.begin(), blob.end());

  return data;
}

// static
std::optional<std::map<std::string, std::string>> Encoder::Decode(
    const std::vector<uint8_t>& blob, DecodedParams* params) {
  if (blob.size() < sizeof(struct vpd_entry)) {
    LOG(ERROR) << "vpd size is too small: " << blob.size();
    return {};
  }

  if (blob.size() > kMaxBlobSize) {
    LOG(ERROR) << "vpd size is too large: " << blob.size();
    return {};
  }

  std::map<std::string, std::string> ret;
  std::vector<uint8_t> eps;
  struct vpd_entry* eps_entry;
  // EPS may not be aligned to the beginning of the partition.
  for (unsigned int index = 0; index < blob.size(); index += kEpsOffsetStride) {
    std::vector<uint8_t> offset_blob(blob.begin() + index, blob.end());

    if (offset_blob.size() < sizeof(struct vpd_entry)) {
      break;
    }
    if (IsEps(offset_blob)) {
      eps = std::move(offset_blob);
      params->eps_offset = index;
      break;
    }
  }
  if (eps.empty()) {
    // This is an old heuristic used by VPD tooling, to treat an unformatted
    // flash as an empty key/value store.
    std::array<uint8_t, 4> empty_flash{0xff, 0xff, 0xff, 0xff};
    if (std::equal(empty_flash.begin(), empty_flash.end(), blob.begin())) {
      LOG(WARNING) << "VPD partition not formatted. It's fine.";
      return ret;
    }

    // All other contents are treated as an error.
    LOG(WARNING)
        << "could not find VPD headers; partition may not be formatted";
    return {};
  }

  eps_entry = reinterpret_cast<struct vpd_entry*>(eps.data());

  struct vpd_header* header;
  struct vpd_table_binary_blob_pointer* data;
  if (eps.size() < eps_entry->entry_length + sizeof(*header)) {
    LOG(ERROR) << "EPS header too small: " << eps.size();
    return {};
  }

  std::vector<uint8_t> tracking(eps.begin() + eps_entry->entry_length,
                                eps.end());

  uuid_t spd_uuid, vpd_2_0_uuid, vpd_1_2_uuid;
  CHECK_GE(uuid_parse(GOOGLE_SPD_UUID, spd_uuid), 0);
  CHECK_GE(uuid_parse(GOOGLE_VPD_2_0_UUID, vpd_2_0_uuid), 0);
  CHECK_GE(uuid_parse(GOOGLE_VPD_1_2_UUID, vpd_1_2_uuid), 0);

  for (unsigned int expected_handle = 0;; expected_handle++) {
    header = reinterpret_cast<struct vpd_header*>(tracking.data());
    data = reinterpret_cast<struct vpd_table_binary_blob_pointer*>(
        tracking.data() + sizeof(*header));

    if (tracking.size() < sizeof(*header)) {
      LOG(ERROR) << "ran out of room for header";
      return {};
    }

    if (header->type == VPD_TYPE_END) {
      break;
    }
    // Make sure we don't have too many handles.
    static_assert(sizeof(header->handle) == sizeof(uint16_t));
    if (expected_handle >= UINT16_MAX) {
      LOG(ERROR) << "too many handles";
      return {};
    }

    if (header->handle != expected_handle) {
      LOG(ERROR) << "unexpected handle: got " << int(header->handle)
                 << ", expected " << expected_handle;
      return {};
    }

    if (header->type != VPD_TYPE_BINARY_BLOB_POINTER) {
      LOG(ERROR)
          << "unsupported Binary Blob Pointer type; we support 241, but handle "
          << expected_handle << " is type " << header->type;
      return {};
    }

    if (tracking.size() < sizeof(*header) + sizeof(*data)) {
      LOG(ERROR) << "ran out of room for data";
      return {};
    }

    // Payload relative to eps.
    uint32_t offs = data->offset -
                    (eps_entry->table_address - sizeof(*eps_entry)) -
                    params->eps_offset;
    if (offs >= eps.size()) {
      LOG(ERROR) << "table overflows the buffer: " << offs
                 << ", eps size: " << eps.size();
      return {};
    }

    if (!memcmp(data->uuid, spd_uuid, sizeof(data->uuid))) {
      // TODO(wdzeng): Uncomment this after we migrate to use absl-20240116.
      // VLOG(1) << "Ignoring SPD";
    } else if (!memcmp(data->uuid, vpd_1_2_uuid, sizeof(data->uuid))) {
      LOG(ERROR) << "VPD 1.2 not supported";
      return {};
    } else if (!memcmp(data->uuid, vpd_2_0_uuid, sizeof(data->uuid))) {
      // TODO(wdzeng): Uncomment this after we migrate to use absl-20240116.
      // VLOG(1) << "Found VPD 2.0 data";
      auto dict =
          DecodeBlob(std::vector<uint8_t>(eps.begin() + offs, eps.end()));
      if (!dict) {
        return {};
      }
      ret.merge(*dict);
    } else {
      LOG(ERROR) << "unsupported UUID: "
                 << absl::BytesToHexString(absl::string_view(
                        reinterpret_cast<const char*>(data->uuid),
                        sizeof(data->uuid)));
      return {};
    }

    uint32_t table_len;
    if (!VpdType241Size(tracking, &table_len)) {
      LOG(ERROR) << "could not determine type 241 length";
      return {};
    }

    if (table_len >= tracking.size()) {
      LOG(ERROR) << "type 241 table overflow: " << table_len
                 << " >= " << tracking.size();
      return {};
    }

    tracking =
        std::vector<uint8_t>(tracking.begin() + table_len, tracking.end());
  }

  return ret;
}

// static
std::optional<std::map<std::string, std::string>> Encoder::DecodeRaw(
    const std::vector<uint8_t>& blob) {
  if (blob.size() > kMaxBlobSize) {
    LOG(ERROR) << "vpd size is too large: " << blob.size();
    return {};
  }

  return DecodeBlob(blob);
}

}  // namespace vpd

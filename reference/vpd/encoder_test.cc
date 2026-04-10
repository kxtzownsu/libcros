// Copyright 2023 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include <stddef.h>
#include <stdint.h>

#include <array>
#include <map>
#include <optional>
#include <string>
#include <vector>

#include "base/files/file_path.h"
#include "base/files/file_util.h"
#include "base/logging.h"
#include "gtest/gtest.h"

#include "vpd/encoder.h"
extern "C" {
#include "vpd/include/lib/vpd.h"
#include "vpd/include/lib/vpd_decode.h"
#include "vpd/include/lib/vpd_tables.h"
}  // extern "C"

namespace vpd {

class EncoderTest : public ::testing::Test {};

TEST_F(EncoderTest, Encode) {
  // C++ strings can technically hold the padding, if we're careful...
  std::string padded_string(
      {'p', 'a', 'd', 'd', 'i', 'n', 'g', '\0', '\0', '\0'});
  EXPECT_EQ(10, padded_string.size());

  auto blob = Encoder::Encode(
      Encoder::EncodingParams{
          .partition_offset = 0,
      },
      {
          {"foo", "bar"},
          {"thi/s", ""},
          {"padded", padded_string},
          {"Newlines", "are\ndiscouraged"},
          {"but", "they\nare\nsupported in the encoding"},
      });
  ASSERT_NE(std::nullopt, blob);
  ASSERT_LT(0, blob->size());

  Encoder::DecodedParams params;
  auto dict = Encoder::Decode(*blob, &params);
  ASSERT_NE(std::nullopt, dict);

  EXPECT_EQ(5, dict->size());

  EXPECT_EQ("bar", (*dict)["foo"]);
  EXPECT_EQ("", (*dict)["thi/s"]);

  // For some reason, we like binary padding for some stuff.
  EXPECT_EQ(padded_string, (*dict)["padded"]);
  EXPECT_EQ(10, (*dict)["padded"].size());

  // Newlines are disallowed, but the encoding format technically supports it.
  // Make sure it works.
  EXPECT_EQ("are\ndiscouraged", (*dict)["Newlines"]);
  EXPECT_EQ("they\nare\nsupported in the encoding", (*dict)["but"]);
}

TEST_F(EncoderTest, EncodeEmpty) {
  auto blob = Encoder::Encode(Encoder::EncodingParams{}, {});
  ASSERT_NE(std::nullopt, blob);

  Encoder::DecodedParams params;
  auto dict = Encoder::Decode(*blob, &params);
  EXPECT_NE(std::nullopt, dict);
  EXPECT_EQ(0, dict->size());
}

TEST_F(EncoderTest, EncodeParams) {
  // Quiet the noisy logs.
  auto log_level = logging::GetMinLogLevel();
  logging::SetMinLogLevel(logging::LOGGING_FATAL);

  for (uint32_t eps_offset = 0; eps_offset < 1024; eps_offset++) {
    auto blob = Encoder::Encode(
        Encoder::EncodingParams{
            .partition_offset = 0,
            .eps_offset = eps_offset,
        },
        {
            {"some", "key"},
            {"another", "key"},
        });
    // Only 16-stride offsets should be accepted.
    if (eps_offset % 16 == 0) {
      EXPECT_NE(std::nullopt, blob);
    } else {
      EXPECT_EQ(std::nullopt, blob);
    }
  }

  // Restore the old level.
  logging::SetMinLogLevel(log_level);
}

TEST_F(EncoderTest, Decode) {
  auto blob =
      base::ReadFileToBytes(base::FilePath("vpd-0.0.1/testdata/ro.bin"));
  ASSERT_NE(std::nullopt, blob);

  Encoder::DecodedParams params;
  auto dict = Encoder::Decode(*blob, &params);
  ASSERT_NE(std::nullopt, dict);

  EXPECT_EQ(dict->end(), dict->find("nonexistentkey"));

  EXPECT_NE(dict->end(), dict->find("mlb_serial_number"));
  EXPECT_EQ("ABC123DEF567", (*dict)["mlb_serial_number"]);

  EXPECT_EQ(13, dict->size());
}

TEST_F(EncoderTest, DecodeRaw) {
  auto blob =
      base::ReadFileToBytes(base::FilePath("vpd-0.0.1/testdata/ro_raw"));
  ASSERT_NE(std::nullopt, blob);

  std::map<std::string, std::string> contents = {
      {"serial_number", "NXH0BAA001122334455660"},
      {"region", "us"},
      {"mlb_serial_number", "01234AURBEEF00001"},
      {"in_accel_y_lid_calibbias", "52"},
      {"in_accel_x_lid_calibbias", "-22"},
      {"in_accel_z_lid_calibbias", "-28"},
      {"stable_device_secret_DO_NOT_SHARE",
       "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"},
  };

  auto dict = Encoder::DecodeRaw(*blob);
  ASSERT_NE(std::nullopt, dict);

  EXPECT_EQ(dict->size(), contents.size());

  for (const auto& pair : contents) {
    EXPECT_EQ((*dict)[pair.first], pair.second);
  }
}

TEST_F(EncoderTest, DecodeEmptyFlash) {
  // Empty (0xff) flash of a reasonable size. VPD tooling has always treated
  // this as empty, and not an error.
  const std::vector<uint8_t> ff_flash(16 * 1024, 0xff);

  Encoder::DecodedParams params;
  auto dict = Encoder::Decode(ff_flash, &params);
  ASSERT_NE(std::nullopt, dict);

  EXPECT_TRUE(dict->empty());
}

TEST_F(EncoderTest, DecodeCorruptFlash) {
  // Simulate flash contents of a reasonable size that are not all-0xff, and
  // don't have appropriate header signatures. VPD tooling has always treated
  // this as an error.
  std::vector<uint8_t> blob({0xd, 0xe, 0xa, 0xd, 0xb, 0xe, 0xe, 0xf});
  blob.resize(16 * 1024, 0xff);

  Encoder::DecodedParams params;
  auto dict = Encoder::Decode(blob, &params);
  EXPECT_EQ(std::nullopt, dict);
}

TEST_F(EncoderTest, EncodeLength) {
  const vpd::Encoder::EncodingParams params;
  size_t preamble = sizeof(struct google_vpd_info) + GOOGLE_VPD_2_0_OFFSET;

  {
    // 0 length.
    std::map<std::string, std::string> contents = {{"", ""}};
    auto blob = vpd::Encoder::Encode(params, contents);
    ASSERT_NE(std::nullopt, blob);

    ASSERT_LE(preamble, blob->size());

    const std::vector<uint8_t> vpd(blob->begin() + preamble, blob->end());
    const std::vector<uint8_t> expected({VPD_TYPE_STRING, 0x00 /* key length */,
                                         0x00 /* value length */,
                                         VPD_TYPE_TERMINATOR});
    EXPECT_EQ(expected, vpd);
  }
  {
    // 1-byte length (0x7f).
    std::string key;
    key.resize(0x7f);
    std::map<std::string, std::string> contents = {{key, ""}};
    auto blob = vpd::Encoder::Encode(params, contents);
    ASSERT_NE(std::nullopt, blob);

    ASSERT_LE(preamble, blob->size());

    const std::vector<uint8_t> vpd(blob->begin() + preamble, blob->end());
    std::vector<uint8_t> expected({VPD_TYPE_STRING});
    expected.push_back(0x7f);  // key length
    expected.insert(expected.end(), key.begin(), key.end());
    expected.push_back(0x00);  // value length
    expected.push_back(VPD_TYPE_TERMINATOR);
    EXPECT_EQ(expected, vpd);
  }
  {
    // 2-byte length (0x80).
    std::string key;
    key.resize(0x80);
    std::map<std::string, std::string> contents = {{key, ""}};
    auto blob = vpd::Encoder::Encode(params, contents);
    ASSERT_NE(std::nullopt, blob);

    ASSERT_LE(preamble, blob->size());

    const std::vector<uint8_t> vpd(blob->begin() + preamble, blob->end());
    std::vector<uint8_t> expected({VPD_TYPE_STRING});
    expected.push_back(0x81);  // key length
    expected.push_back(0x00);  // key length
    expected.insert(expected.end(), key.begin(), key.end());
    expected.push_back(0x00);  // value length
    expected.push_back(VPD_TYPE_TERMINATOR);
    EXPECT_EQ(expected, vpd);
  }
  {
    // 3-byte length (0x100040).
    std::string key;
    key.resize(0x100040);
    std::map<std::string, std::string> contents = {{key, ""}};
    auto blob = vpd::Encoder::Encode(params, contents);
    ASSERT_NE(std::nullopt, blob);

    ASSERT_LE(preamble, blob->size());

    const std::vector<uint8_t> vpd(blob->begin() + preamble, blob->end());
    std::vector<uint8_t> expected({VPD_TYPE_STRING});
    expected.push_back(0xc0);  // key length
    expected.push_back(0x80);  // key length
    expected.push_back(0x40);  // key length
    expected.insert(expected.end(), key.begin(), key.end());
    expected.push_back(0x00);  // value length
    expected.push_back(VPD_TYPE_TERMINATOR);
    EXPECT_EQ(expected, vpd);
  }
}

TEST_F(EncoderTest, DecodeLength) {
  {
    // Empty blob is an empty store.
    std::vector<uint8_t> encoded{};
    std::map<std::string, std::string> expected;
    EXPECT_EQ(expected, vpd::Encoder::DecodeRaw(encoded));
  }
  {
    // Decoded key length is 0, but we're missing the value and the terminator.
    std::vector<uint8_t> encoded({VPD_TYPE_STRING, 0x00});
    EXPECT_EQ(std::nullopt, vpd::Encoder::DecodeRaw(encoded));
  }
  {
    // Decoded key and value length are 0, but no terminator. That's legacy, but
    // OK.
    std::vector<uint8_t> encoded({VPD_TYPE_STRING, 0x00, 0x00});
    std::map<std::string, std::string> expected({{"", ""}});
    EXPECT_EQ(expected, vpd::Encoder::DecodeRaw(encoded));
  }
  {
    // Decoded key and value length are 0, plus a terminator. All good.
    std::vector<uint8_t> encoded(
        {VPD_TYPE_STRING, 0x00, 0x00, VPD_TYPE_TERMINATOR});
    std::map<std::string, std::string> expected({{"", ""}});
    EXPECT_EQ(expected, vpd::Encoder::DecodeRaw(encoded));
  }
  {
    // Key length 0x7f (maximum 1-byte length).
    std::vector<uint8_t> encoded({VPD_TYPE_STRING, 0x7F});
    encoded.resize(encoded.size() + 0x7F);
    // Value length 0.
    encoded.push_back(0x00);
    encoded.push_back(VPD_TYPE_TERMINATOR);

    auto dict = vpd::Encoder::DecodeRaw(encoded);
    EXPECT_NE(std::nullopt, dict);
    if (dict) {
      EXPECT_EQ(1, dict->size());
      auto pair = *dict->begin();
      EXPECT_EQ(0x7F, pair.first.size());
      EXPECT_EQ("", pair.second);
    }
  }
  {
    // 2 bytes of length.
    std::vector<uint8_t> encoded({VPD_TYPE_STRING, 0x81, 0x02});
    encoded.resize(encoded.size() + 0x82);
    // Value length 0.
    encoded.push_back(0x00);
    encoded.push_back(VPD_TYPE_TERMINATOR);

    auto dict = vpd::Encoder::DecodeRaw(encoded);
    EXPECT_NE(std::nullopt, dict);
    if (dict) {
      EXPECT_EQ(1, dict->size());
      auto pair = *dict->begin();
      EXPECT_EQ(0x82, pair.first.size());
      EXPECT_EQ("", pair.second);
    }
  }
  {
    // "Large" key length. The format can fit up to 0x7FFFFFFF, but 32-bit
    // architectures have a tough time fitting that size of buffers (and
    // possibly some scratch space) in memory, so we don't push it.
    std::vector<uint8_t> encoded({VPD_TYPE_STRING});
    std::array<uint8_t, 5> encoded_length{0x82, 0xF8, 0x80, 0x80, 0x00};
    encoded.insert(encoded.end(), encoded_length.begin(), encoded_length.end());
    encoded.resize(encoded.size() + 0x2F000000);
    // Value length 0.
    encoded.push_back(0x00);
    encoded.push_back(VPD_TYPE_TERMINATOR);

    auto dict = vpd::Encoder::DecodeRaw(encoded);
    EXPECT_NE(std::nullopt, dict);
    if (dict) {
      EXPECT_EQ(1, dict->size());
      auto pair = *dict->begin();
      EXPECT_EQ(0x2F000000, pair.first.size());
      EXPECT_EQ("", pair.second);
    }
  }
}

// Test that input (encoding) and output (decoded) params match up.
TEST_F(EncoderTest, BidirectionlParams) {
  std::array params{
      Encoder::EncodingParams{
          .partition_offset = 0,
      },
      Encoder::EncodingParams{
          .partition_offset = 0x12300,
          .eps_offset = 0x100,
      },
  };

  for (const auto& param : params) {
    auto blob = Encoder::Encode(param, {
                                           {"key1", ""},
                                           {"key", "largo"},
                                           {"a",
                                            "looooooooooooooooooooooooooooooooo"
                                            "ooooooooooooooooooooooooong one"},
                                       });
    ASSERT_NE(std::nullopt, blob);
    ASSERT_LT(0, blob->size());

    Encoder::DecodedParams decoded_params;
    auto dict = Encoder::Decode(*blob, &decoded_params);
    ASSERT_NE(std::nullopt, dict);

    EXPECT_EQ(3, dict->size());
    EXPECT_EQ("", (*dict)["key1"]);
    EXPECT_EQ("largo", (*dict)["key"]);
    EXPECT_EQ(
        "loooooooooooooooooooooooooooooooooooooooooooooooooooooooooong one",
        (*dict)["a"]);

    EXPECT_EQ(decoded_params.eps_offset, param.eps_offset);
  }
}

}  // namespace vpd

// Copyright 2023 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include <array>
#include <map>
#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "gtest/gtest.h"

#include "vpd/fake_vpd.h"
#include "vpd/util/update_rw_vpd.h"
#include "vpd/vpd.h"

namespace vpd {

class UpdateRwVpdTest : public ::testing::Test {};

TEST_F(UpdateRwVpdTest, ParseArgs) {
  const char* prog_name = "update_rw_vpd";

  struct expectation {
    std::vector<const char*> args;
    std::optional<std::map<std::string, std::optional<std::string>>> values;
  };

  std::array expectations{
      (struct expectation){
          .args = {prog_name}, .values = {{}},  // empty map
      },
      (struct expectation){
          .args = {prog_name, "key_without_value"},
          .values = std::nullopt,  // error
      },
      (struct expectation){
          .args = {prog_name, "delete_key", ""},
          .values = {{{"delete_key", std::nullopt}}},
      },
      (struct expectation){
          .args = {prog_name, "key", "value"},
          .values = {{{"key", "value"}}},
      },
      (struct expectation){
          .args = {prog_name, "key1", "value1", "key2", "value2", "key3", ""},
          .values = {{
              {"key1", "value1"},
              {"key2", "value2"},
              {"key3", std::nullopt},
          }},
      },
      (struct expectation){
          .args = {prog_name, "non@ascii", "value"},
          .values = std::nullopt,  // error
      },
      (struct expectation){
          .args = {prog_name, "goodkey", "v", "non@ascii", "value"},
          .values = std::nullopt,  // error
      },
  };

  for (auto& expectation : expectations) {
    int argc = expectation.args.size();
    const char** argv = expectation.args.data();

    EXPECT_EQ(expectation.values, update_rw_vpd::ParseArgs(argc, argv));
  }
}

TEST_F(UpdateRwVpdTest, NeedsWrite) {
  Vpd vpd(std::make_unique<FakeVpd>());

  EXPECT_TRUE(update_rw_vpd::NeedsWrite(vpd, {{"key", "value"}}));
  EXPECT_FALSE(
      update_rw_vpd::NeedsWrite(vpd, {{"delete_missing", std::nullopt}}));
  EXPECT_TRUE(
      update_rw_vpd::NeedsWrite(vpd, {
                                         {"key", "value"},
                                         {"delete_missing", std::nullopt},
                                     }));

  EXPECT_TRUE(vpd.WriteValue(VpdRw, "key", "value"));
  EXPECT_EQ("value", vpd.GetValue(VpdRw, "key"));

  EXPECT_FALSE(update_rw_vpd::NeedsWrite(vpd, {{"key", "value"}}));
  EXPECT_FALSE(
      update_rw_vpd::NeedsWrite(vpd, {{"delete_missing", std::nullopt}}));
  EXPECT_FALSE(
      update_rw_vpd::NeedsWrite(vpd, {
                                         {"key", "value"},
                                         {"delete_missing", std::nullopt},
                                     }));
  EXPECT_TRUE(update_rw_vpd::NeedsWrite(vpd, {
                                                 {"key", std::nullopt},
                                             }));
}

}  // namespace vpd

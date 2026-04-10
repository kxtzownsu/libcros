// Copyright 2024 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
//
// Outputs a filtered list of VPD key/value pairs.
//
// Some clients will care not just whether these key/value pairs exist, but
// also whether the underlying region storage was valid. We dump some free-form
// error text about their status in stderr, and we dump precise error codes on
// exit as well.
//
// Exit status expectation:
//   0 = both RO and RW VPD are valid
//   1 = only RO is invalid
//   2 = only RW is invalid
//   3 = both RO and RW are invalid
//
// All other exit statuses are unexpected.

#include <array>
#include <iostream>
#include <set>

#include "absl/log/initialize.h"
#include "vpd/vpd.h"

namespace {

constexpr std::array kVpdKeyAllowlist{
    "ActivateDate",
    "Product_S/N",
    "attested_device_id",
    "block_devmode",
    "check_enrollment",
    "customization_id",
    "display_profiles",
    "gbind_attribute",
    "initial_locale",
    "initial_timezone",
    "keyboard_layout",
    "model_name",
    "oem_device_requisition",
    "oem_name",
    "panel_backlight_max_nits",
    "region",
    "rlz_brand_code",
    "rlz_embargo_end_date",
    "serial_number",
    "should_send_rlz_ping",
    "sku_number",
    "ubind_attribute",
};

enum ExitStatus {
  Valid = 0,
  RoInvalid = 1,
  RwInvalid = 2,
  BothInvalid = RoInvalid | RwInvalid,
};

}  // namespace

int main(int argc, char* argv[]) {
  absl::InitializeLog();
  vpd::Vpd vpd;

  auto ro = vpd.GetValues(vpd::VpdRo);
  auto rw = vpd.GetValues(vpd::VpdRw);

  std::set<std::string> allowlist(std::begin(kVpdKeyAllowlist),
                                  std::end(kVpdKeyAllowlist));

  for (const auto& dict : std::array{ro, rw}) {
    for (const auto& [key, value] : dict) {
      if (allowlist.contains(key)) {
        std::cout << "\"" << key << "\"=\"" << value << "\"\n";
      }
    }
  }

  int ret = Valid;

  if (!vpd.Valid(vpd::VpdRo)) {
    std::cerr << "RO VPD is invalid\n";
    ret |= RoInvalid;
  }

  if (!vpd.Valid(vpd::VpdRw)) {
    std::cerr << "RW VPD is invalid\n";
    ret |= RwInvalid;
  }

  return ret;
}

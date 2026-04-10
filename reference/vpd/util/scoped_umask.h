// Copyright 2024 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef UTIL_SCOPED_UMASK_H_
#define UTIL_SCOPED_UMASK_H_

#include <sys/types.h>

#include "vpd/export.h"

namespace vpd {
namespace util {

// ScopedUmask is a helper class for temporarily setting the umask before a
// set of operations. umask(2) is never expected to fail.
class EXPORT ScopedUmask {
 public:
  explicit ScopedUmask(mode_t new_umask);
  ScopedUmask(const ScopedUmask&) = delete;
  ScopedUmask& operator=(const ScopedUmask&) = delete;

  ~ScopedUmask();

 private:
  mode_t saved_umask_;

  // Avoid reusing ScopedUmask for multiple masks. We delete the copy
  // constructor and operator=, but there are other situations where
  // reassigning a new ScopedUmask to an existing ScopedUmask object is
  // problematic:
  //
  // /* starting umask: default_value
  // auto a = std::make_unique<ScopedUmask>(first_value);
  // ... code here ...
  // a.reset(ScopedUmask(new_value));
  //
  // Here, the order of destruction of the old object and the construction of
  // the new object is inverted. The recommended usage would be:
  //
  // {
  //    ScopedUmask a(old_value);
  //    ... code here ...
  // }
  //
  // {
  //    ScopedUmask a(new_value);
  //    ... code here ...
  // }
};

}  // namespace util
}  // namespace vpd

#endif  // UTIL_SCOPED_UMASK_H_

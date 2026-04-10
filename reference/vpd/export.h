// Copyright 2024 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef EXPORT_H_
#define EXPORT_H_

// Explicitly opt in to making symbols visible, in case we build with
// -fvisibility=internal or similar.
#define EXPORT __attribute__((__visibility__("default")))

#endif  // EXPORT_H_

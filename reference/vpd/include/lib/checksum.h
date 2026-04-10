/*
 * Copyright 2010 Google LLC
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * as published by the Free Software Foundation; either version 2
 * of the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301, USA
 *
 * Ported from mosys project (http://code.google.com/p/mosys/).
 */

#ifndef INCLUDE_LIB_CHECKSUM_H_
#define INCLUDE_LIB_CHECKSUM_H_

#ifdef __cplusplus
extern "C" {
#endif /* __cplusplus */

#include <inttypes.h>
#include <sys/types.h>

/*
 * zero8_csum - Calculates 8-bit zero-sum checksum
 *
 * @buf:  input buffer
 * @len:  length of buffer
 *
 * The summation of the bytes in the array and the csum will equal zero
 * for 8-bit data size.
 *
 * returns checksum to indicate success
 */
extern uint8_t zero8_csum(uint8_t* buf, size_t len);

#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif /* INCLUDE_LIB_CHECKSUM_H_ */

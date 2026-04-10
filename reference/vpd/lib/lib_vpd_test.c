/*
 * Copyright 2013 The ChromiumOS Authors
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 *
 */
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "lib/lib_vpd.h"

#ifndef NDEBUG
enum {
  TEST_OK = 0,
  TEST_FAIL = 1,
};


#define CU8 (const uint8_t *)  /* for compiler warning on sign bit */


int testEncodeLen() {
  unsigned char output[10];
  int generated;

  /* fail cases */
  assert(VPD_ERR_INVALID == encodeLen(-1, output, 0, &generated));
  assert(VPD_ERR_OVERFLOW == encodeLen(0x7f, output, 0, &generated));

  /* success case - 1 byte output, all zeros */
  assert(VPD_OK == encodeLen(0x00, output, 1, &generated));
  assert(1 == generated);
  assert(0x00 == output[0]);

  /* success case - 1 byte output */
  assert(VPD_OK == encodeLen(0x7f, output, 1, &generated));
  assert(1 == generated);
  assert(0x7f == output[0]);

  /* 2 bytes of output */
  assert(VPD_ERR_OVERFLOW == encodeLen(0x80, output, 1, &generated));
  /* success */
  assert(VPD_OK == encodeLen(0x80, output, 2, &generated));
  assert(2 == generated);
  assert(0x81 == output[0]);
  assert(0x00 == output[1]);

  /* 3 bytes of output */
  assert(VPD_ERR_OVERFLOW == encodeLen(0x100040, output, 0, &generated));
  assert(VPD_ERR_OVERFLOW == encodeLen(0x100040, output, 1, &generated));
  assert(VPD_ERR_OVERFLOW == encodeLen(0x100040, output, 2, &generated));
  /* success */
  assert(VPD_OK == encodeLen(0x100040, output, 3, &generated));
  assert(3 == generated);
  assert(0xc0 == output[0]);
  assert(0x80 == output[1]);
  assert(0x40 == output[2]);

  printf("[PASS] %s()\n", __FUNCTION__);
  return TEST_OK;
}


int testDecodeLen() {
  uint32_t length;
  uint32_t consumed;

  { /* max_len is 0. No more char in string. */
    uint8_t encoded[] = { 0x00 };
    assert(VPD_ERR_DECODE == decodeLen(0, encoded, &length, &consumed));
  }
  { /* just decode one byte */
    uint8_t encoded[] = { 0x00 };
    assert(VPD_OK == decodeLen(sizeof(encoded), encoded, &length, &consumed));
    assert(consumed == sizeof(encoded));
    assert(length == 0);
  }
  { /* just decode one byte */
    uint8_t encoded[] = { 0x7F };
    assert(VPD_OK == decodeLen(sizeof(encoded), encoded, &length, &consumed));
    assert(consumed == sizeof(encoded));
    assert(length == 0x7F);
  }
  { /* more bit is set, but reachs end of string. */
    uint8_t encoded[] = { 0x80 };
    assert(VPD_ERR_DECODE == decodeLen(
        sizeof(encoded), encoded, &length, &consumed));
  }
  { /* decode 2 bytes, but reachs end of string. */
    uint8_t encoded[] = { 0x81, 0x02 };
    assert(VPD_ERR_DECODE == decodeLen(1, encoded, &length, &consumed));
  }
  { /* more bit is set, but reachs end of string. */
    uint8_t encoded[] = { 0x81, 0x82 };
    assert(VPD_ERR_DECODE == decodeLen(sizeof(encoded), encoded, &length, &consumed));
  }
  { /* decode 2 bytes, normal case */
    uint8_t encoded[] = { 0x81, 0x02 };
    assert(VPD_OK == decodeLen(sizeof(encoded), encoded, &length, &consumed));
    assert(consumed == sizeof(encoded));
    assert(length == 0x82);
  }
  { /* decode 2 bytes, normal case (bot reach end of string). */
    uint8_t encoded[] = { 0xFF, 0x7F, 0xFF };
    assert(VPD_OK == decodeLen(sizeof(encoded), encoded, &length, &consumed));
    assert(consumed == 2);
    assert(length == 0x3FFF);
  }
  { /* weird case, but still valid. */
    uint8_t encoded[] = { 0x80, 0x00 };
    assert(VPD_OK == decodeLen(sizeof(encoded), encoded, &length, &consumed));
    assert(consumed == sizeof(encoded));
    assert(length == 0);
  }
  { /* test max length */
    uint8_t encoded[] = { 0x87, 0xFF, 0xFF, 0xFF, 0x7F };
    assert(VPD_OK == decodeLen(sizeof(encoded), encoded, &length, &consumed));
    assert(consumed == sizeof(encoded));
    assert(length == 0x7FFFFFFF);
  }

  printf("[PASS] %s()\n", __FUNCTION__);
  return TEST_OK;
}


int testEncodeVpdString() {
  unsigned char expected[] = {
    VPD_TYPE_STRING,
    0x03, 'K', 'E', 'Y',
    0x05, 'V', 'A', 'L', 'U', 'E',
  };
  unsigned char buf[256];
  int generated = 0;

  assert(VPD_OK ==
         encodeVpdString(CU8"KEY", CU8"VALUE",
                         VPD_AS_LONG_AS, sizeof(buf), buf, &generated));
  assert(sizeof(expected) == generated);
  assert(!memcmp(expected, buf, generated));

  printf("[PASS] %s()\n", __FUNCTION__);
  return TEST_OK;
}


int testEncodeVpdStringPadding() {
  unsigned char expected[] = {
    VPD_TYPE_STRING,
    0x03, 'K', 'E', 'Y',
    0x08, 'V', 'A', 'L', 'U', 'E', '\0', '\0', '\0',
  };
  unsigned char buf[256];
  int generated = 0;

  assert(VPD_OK == encodeVpdString(CU8"KEY", CU8"VALUE",
                                   8, sizeof(buf), buf, &generated));
  assert(sizeof(expected) == generated);
  assert(!memcmp(expected, buf, generated));

  printf("[PASS] %s()\n", __FUNCTION__);
  return TEST_OK;
}


int testEncodeMultiStrings() {
  unsigned char expected[] = {
    VPD_TYPE_STRING,
    0x03, 'M', 'A', 'C',
    0x08, '0', '1', '2', '3', '4', '5', '6', '7',
    VPD_TYPE_STRING,
    0x07, 'P', 'r', 'o', 'd', '/', 'I', 'd',
    0x0c, 'M', 'a', 'r', 'i', 'o', '0', '9', '2', '8', '4', '\0', '\0',
  };
  unsigned char buf[256];
  int generated = 0;

  assert(VPD_OK == encodeVpdString(CU8"MAC", CU8"01234567",
                                   0x08, sizeof(buf), buf, &generated));
  assert(VPD_OK == encodeVpdString(CU8"Prod/Id", CU8"Mario09284",
                                   0x0c, sizeof(buf), buf, &generated));
  assert(sizeof(expected) == generated);
  assert(!memcmp(expected, buf, generated));

  printf("[PASS] %s()\n", __FUNCTION__);
  return TEST_OK;
}


int testContainer() {
  unsigned char expected[] = {
    VPD_TYPE_STRING,
    0x03, 'K', 'E', 'Y',
    0x08, 'V', 'A', 'L', 'U', 'E', '\0', '\0', '\0',
  };
  unsigned char buf[256];
  int generated = 0;
  struct PairContainer container;

  initContainer(&container);
  setString(&container, CU8"KEY", CU8"VALUE", 8);
  encodeContainer(&container, sizeof(buf), buf, &generated);

  assert(sizeof(expected) == generated);
  assert(!memcmp(expected, buf, generated));

  destroyContainer(&container);

  printf("[PASS] %s()\n", __FUNCTION__);
  return TEST_OK;
}


/* Based on previous test cases:
 *
 * KEY=VALUE --> encode --> decode --> expected KEY=VALUE
 */
int testDecodeVpdString() {
  unsigned char expected[] = {
    VPD_TYPE_STRING,
    0x03, 'K', 'E', 'Y',
    0x08, 'V', 'A', 'L', 'U', 'E', '\0', '\0', '\0',
  };
  unsigned char buf[256];
  uint32_t consumed = 0;
  struct PairContainer container;
  int encode_consumed = 0;

  initContainer(&container);
  assert(VPD_OK == decodeToContainer(&container, sizeof(expected), expected,
                                     &consumed));
  assert(sizeof(expected) == consumed);

  encodeContainer(&container, sizeof(buf), buf, &encode_consumed);
  assert(sizeof(expected) == encode_consumed);
  assert(!memcmp(expected, buf, encode_consumed));

  destroyContainer(&container);

  printf("[PASS] %s()\n", __FUNCTION__);
  return TEST_OK;
}


int testDeleteEmptyContainer() {
  struct PairContainer container;

  initContainer(&container);
  assert(VPD_FAIL == deleteKey(&container, CU8"NON_EXISTED_KEY"));

  /* still good for add */
  setString(&container, CU8"FIRST", CU8"1", 8);
  assert(NULL != findString(&container, CU8"FIRST", NULL));

  destroyContainer(&container);

  printf("[PASS] %s()\n", __FUNCTION__);
  return TEST_OK;
}


int testDeleteFirstOfOne() {
  struct PairContainer container;

  initContainer(&container);

  /* test the case that only one string in container. */
  setString(&container, CU8"FIRST", CU8"1", 8);
  assert(VPD_FAIL == deleteKey(&container, CU8"NON_EXISTED_KEY"));
  assert(VPD_OK == deleteKey(&container, CU8"FIRST"));
  assert(NULL == findString(&container, CU8"FIRST", NULL));

  /* still good for add */
  setString(&container, CU8"SECOND", CU8"2", 12);
  assert(NULL != findString(&container, CU8"SECOND", NULL));

  destroyContainer(&container);

  printf("[PASS] %s()\n", __FUNCTION__);
  return TEST_OK;
}


int testDeleteFirstOfTwo() {
  struct PairContainer container;

  initContainer(&container);

  /* add 2 and remove the first one */
  setString(&container, CU8"FIRST", CU8"1", 8);
  setString(&container, CU8"SECOND", CU8"2", 8);
  assert(VPD_FAIL == deleteKey(&container, CU8"NON_EXISTED_KEY"));
  assert(VPD_OK == deleteKey(&container, CU8"FIRST"));
  assert(NULL == findString(&container, CU8"FIRST", NULL));
  assert(NULL != findString(&container, CU8"SECOND", NULL));

  /* still good for add */
  setString(&container, CU8"FIRST", CU8"1", 9);
  assert(NULL != findString(&container, CU8"FIRST", NULL));

  destroyContainer(&container);

  printf("[PASS] %s()\n", __FUNCTION__);
  return TEST_OK;
}


int testDeleteSecondOfTwo() {
  struct PairContainer container;

  initContainer(&container);

  /* add 2 and remove the last one */
  setString(&container, CU8"FIRST", CU8"1", 8);
  setString(&container, CU8"SECOND", CU8"2", 8);
  assert(VPD_FAIL == deleteKey(&container, CU8"NON_EXISTED_KEY"));
  assert(VPD_OK == deleteKey(&container, CU8"SECOND"));
  assert(NULL != findString(&container, CU8"FIRST", NULL));
  assert(NULL == findString(&container, CU8"SECOND", NULL));

  /* still good for add */
  setString(&container, CU8"SECOND", CU8"2", 5);
  assert(NULL != findString(&container, CU8"SECOND", NULL));

  destroyContainer(&container);

  printf("[PASS] %s()\n", __FUNCTION__);
  return TEST_OK;
}


int testDeleteSecondOfThree() {
  unsigned char expected[] = {
    VPD_TYPE_STRING,
    0x05, 'F', 'I', 'R', 'S', 'T',
    0x03, '1', '\0', '\0',
    VPD_TYPE_STRING,
    0x05, 'T', 'H', 'I', 'R', 'D',
    0x04, '3', '\0', '\0', '\0',
  };
  unsigned char buf[256];
  int generated = 0;
  struct PairContainer container;

  initContainer(&container);

  /* add 3 and remove the middle one */
  setString(&container, CU8"FIRST", CU8"1", 3);
  setString(&container, CU8"SECOND", CU8"2", 2);
  setString(&container, CU8"THIRD", CU8"3", 4);
  assert(VPD_FAIL == deleteKey(&container, CU8"NON_EXISTED_KEY"));
  assert(VPD_OK == deleteKey(&container, CU8"SECOND"));
  assert(NULL != findString(&container, CU8"FIRST", NULL));
  assert(NULL == findString(&container, CU8"SECOND", NULL));
  assert(NULL != findString(&container, CU8"THIRD", NULL));

  /* expect the middle one is removed. */
  encodeContainer(&container, sizeof(buf), buf, &generated);
  assert(sizeof(expected) == generated);
  assert(!memcmp(expected, buf, generated));

  /* still good if we delete all */
  assert(VPD_OK == deleteKey(&container, CU8"THIRD"));
  assert(VPD_OK == deleteKey(&container, CU8"FIRST"));

  /* still good for add */
  setString(&container, CU8"FORTH", CU8"4", 4);
  assert(NULL != findString(&container, CU8"FORTH", NULL));
  setString(&container, CU8"FIFTH", CU8"5", 5);
  assert(NULL != findString(&container, CU8"FIFTH", NULL));

  destroyContainer(&container);

  printf("[PASS] %s()\n", __FUNCTION__);
  return TEST_OK;
}
#endif


int main() {
#ifndef NDEBUG
  assert(TEST_OK == testEncodeLen());
  assert(TEST_OK == testDecodeLen());
  assert(TEST_OK == testEncodeVpdString());
  assert(TEST_OK == testEncodeVpdStringPadding());
  assert(TEST_OK == testEncodeMultiStrings());
  assert(TEST_OK == testContainer());
  assert(TEST_OK == testDecodeVpdString());
  assert(TEST_OK == testDeleteEmptyContainer());
  assert(TEST_OK == testDeleteFirstOfOne());
  assert(TEST_OK == testDeleteFirstOfTwo());
  assert(TEST_OK == testDeleteSecondOfTwo());
  assert(TEST_OK == testDeleteSecondOfThree());

  printf("SUCCESS!\n");
#endif
  return 0;
}

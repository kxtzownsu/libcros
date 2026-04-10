/*
 * Copyright 2012 The ChromiumOS Authors
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

#include <assert.h>
#include <ctype.h>
#include <fmap.h>
#include <getopt.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <sys/types.h>
#include <unistd.h>

#include <iostream>
#include <map>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "absl/log/initialize.h"
#include "absl/log/log.h"
#include "absl/strings/str_format.h"

#include "vpd/cache.h"
#include "vpd/encoder.h"
#include "vpd/flashrom.h"
extern "C" {
#include "vpd/include/lib/lib_vpd.h"
}  // extern "C"
#include "vpd/types.h"
#include "vpd/util/file.h"

namespace {

using std::string_literals::operator""s;

/* The comment shown in the begin of --sh output */
#define SH_COMMENT                                                     \
  "#\n"                                                                \
  "# Prepend 'vpd -O' before this text to always reset VPD content.\n" \
  "# Append more -s at end to set additional key/values.\n"            \
  "# Or an empty line followed by other commands.\n"                   \
  "#\n"

/* The EPS base address used to fill the EPS table entry.
 * If the VPD partition can be found in fmap, this points to the starting
 * offset of VPD partition. If not found, this is used to be the base address
 * to increase the VPD 2.0 offset field.
 */
#define UNKNOWN_EPS_BASE ((uint32_t)-1)
uint32_t eps_base = UNKNOWN_EPS_BASE;

/* If found_vpd, replace the VPD partition when saveFile().
 * If not found, always create new file when saveFile(). */
bool found_vpd = false;

/* The VPD partition offset and size in buffer. The whole partition includes:
 *
 *   SMBIOS EPS
 *   SMBIOS tables[]
 *   VPD 2.0 data
 *
 */
uint32_t vpd_offset = 0, vpd_size; /* The whole partition */

int isbase64(uint8_t c) {
  return isalnum(c) || (c == '+') || (c == '/') || (c == '=');
}

std::optional<std::string> read_string_from_file(const std::string& file_name) {
  uint32_t i, j;

  auto file_buffer = vpd::util::ReadFileToBytes(file_name);
  if (!file_buffer) {
    PLOG(ERROR) << "Failed to read file: " << file_name;
    return {};
  }

  /*
   * Are the contents a proper base64 blob? Verify it and drop EOL characters
   * along the way. This will help when displaying the contents.
   */
  for (i = 0, j = 0; i < file_buffer->size(); i++) {
    uint8_t c = (*file_buffer)[i];

    if ((c == 0xa) || (c == 0xd))
      continue; /* Skip EOL characters */

    if (!isbase64(c)) {
      LOG(ERROR) << "file " << file_name << " is not in base64 format: " << c
                 << " at " << i;
      return {};
    }
    (*file_buffer)[j++] = c;
  }
  (*file_buffer)[j] = '\0';

  return std::string(reinterpret_cast<const char*>(file_buffer->data()));
}

/*
 * Check if given key name is compliant to recommended format.
 */
vpd_err_t checkKeyName(const std::string& name) {
  for (const auto& c : name) {
    if (!(isalnum(c) || c == '_' || c == '.')) {
      LOG(ERROR) << "VPD key name does not allow char: " << c;
      return VPD_ERR_PARAM;
    }
  }
  return VPD_OK;
}

/*
 * Check if key and value are compliant to recommended format.
 * For the checker of the key, see the function |checkKeyName|.
 * If key is "serial_number" or "mlb_serial_number", the value should only
 * contain characters a-z, A-Z, 0-9 or dash (-).
 * TODO(b/288327526): subsume into libvpd?
 */
vpd_err_t checkKeyValuePair(const std::string& key, const std::string& value) {
  vpd_err_t retval = checkKeyName(key);
  if (retval != VPD_OK)
    return retval;

  if (key == "serial_number" || key == "mlb_serial_number") {
    // For serial numbers, we only allow a-z, A-Z, 0-9 and dash (-).
    for (const auto& c : value) {
      if (!(isalnum(c) || c == '-')) {
        LOG(ERROR) << "serial number does not allow char: " << c;
        return VPD_ERR_PARAM;
      }
    }

    if (value.empty()) {
      LOG(ERROR) << "serial number cannot be empty";
      return VPD_ERR_PARAM;
    }

    if (value[0] == '-') {
      LOG(ERROR) << "serial number cannot start with [-]";
      return VPD_ERR_PARAM;
    }

    if (value.back() == '-') {
      LOG(ERROR) << "serial number cannot end with [-]";
      return VPD_ERR_PARAM;
    }
  }

  if (value.find_first_of("\0"s) != std::string::npos) {
    LOG(ERROR) << "value must not contain \\0";
    return VPD_ERR_PARAM;
  }

  return VPD_OK;
}

/*
 * Given a key=value string, this function parses it and adds to arugument
 * pair container. The 'value' can be stored in a base64 format file, in this
 * case the value field is the file name.
 */
vpd_err_t parseString(const std::string& string,
                      bool read_from_file,
                      std::map<std::string, std::string>* set_argument) {
  vpd_err_t retval = VPD_OK;

  if (string.empty() || string[0] == '\0' || string[0] == '=') {
    return VPD_ERR_SYNTAX;
  }

  size_t delim_pos = string.find('=');
  std::string key = string.substr(0, delim_pos);
  std::string value;
  if (delim_pos != std::string::npos) {
    value = string.substr(delim_pos + 1);
  }

  if (read_from_file) {
    /* 'value' in fact is a file name */
    const auto file_contents = read_string_from_file(value);
    if (!file_contents) {
      return VPD_ERR_SYNTAX;
    }
    value = *file_contents;
  }

  retval = checkKeyValuePair(key, value);
  if (retval != VPD_OK) {
    return retval;
  }

  (*set_argument)[key] = value;
  return VPD_OK;
}

/* There are two possible file content appearng here:
 *   1. a full and complete BIOS file
 *   2. a full but only VPD partition area is valid. (no fmap)
 *   3. a full BIOS, but VPD partition is blank.
 *
 * The first case is easy. Just lookup the fmap and find out the VPD partition.
 * The second is harder. We try to search the SMBIOS signature (since others
 * are blank). For the third, we just return and leave caller to read full
 * content, including fmap info.
 *
 * If found, vpd_offset and vpd_size are updated.
 */
vpd_err_t findVpdPartition(const std::vector<uint8_t>& read_buf,
                           const std::string& region_name,
                           uint32_t* vpd_offset,
                           uint32_t* vpd_size) {
  assert(vpd_offset);
  assert(vpd_size);

  /* scan the file and find out the VPD partition. */
  const off_t sig_offset = fmap_find(read_buf.data(), read_buf.size());
  if (sig_offset < 0) {
    return VPD_ERR_NOT_FOUND;
  }

  const struct fmap* fmap;
  if (sig_offset + sizeof(*fmap) > read_buf.size()) {
    LOG(ERROR) << "Bad FMAP at: " << sig_offset;
    return VPD_FAIL;
  }
  /* FMAP signature is found, try to search the partition name in table. */
  fmap = (const struct fmap*)(read_buf.data() + sig_offset);

  const struct fmap_area* area = fmap_find_area(fmap, region_name.c_str());
  if (!area) {
    LOG(ERROR) << "The VPD partition [" << region_name << "] is not found.";
    return VPD_ERR_NOT_FOUND;
  }
  *vpd_offset = area->offset;
  *vpd_size = area->size;
  /* Mark found here then saveFile() knows where to write back (vpd_offset,
   * vpd_size). */
  found_vpd = true;
  return VPD_OK;
}

/* Load VPD into kv_dict. kv_dict will be unchanged if overwrite_it is set. */
vpd_err_t loadFile(const std::vector<uint8_t>& vpd_blob,
                   const std::string& region_name,
                   vpd::VpdRegion region,
                   vpd::Flashrom& flashrom,
                   int overwrite_it,
                   std::map<std::string, std::string>* kv_dict) {
  vpd_err_t retval = VPD_OK;

  if (0 == findVpdPartition(vpd_blob, region_name, &vpd_offset, &vpd_size)) {
    eps_base = vpd_offset;
  } else {
    /* We cannot parse out the VPD partition address from given file.
     * Then, try to read the whole BIOS chip. */
    uint32_t offset, size;
    if (!flashrom.GetPartitionDimensions(region, &offset, &size)) {
      if (overwrite_it) {
        retval = VPD_OK;
      } else {
        retval = VPD_FAIL;
        LOG(ERROR) << "GetPartitionDimensions failed";
      }
      return retval;
    }
    eps_base = offset;
    vpd_size = size;
  }

  /* Update the following variables:
   *   eps_base: integer, the VPD EPS address in ROM.
   *   vpd_offset: integer, the VPD partition offset in file (formerly
   *     read_buf[]).
   *   vpd_buf: uint8_t*, points to the VPD partition.
   */
  const uint8_t* vpd_buf = vpd_blob.data() + vpd_offset;

  if (eps_base == UNKNOWN_EPS_BASE) {
    LOG(ERROR) << "Cannot determine eps_base. Cannot go on. Ensure you have a "
                  "valid FMAP.";
    return VPD_ERR_INVALID;
  }

  /* In overwrite mode, we don't care the content inside. Stop parsing. */
  if (overwrite_it) {
    return VPD_OK;
  }

  // Decoder wants to export params, but we don't really want to do anything
  // with them.
  // TODO(b/288327526): remove.
  vpd::Encoder::DecodedParams ignored_params;
  auto dict = vpd::Encoder::Decode(
      std::vector<uint8_t>(vpd_buf, vpd_buf + vpd_size), &ignored_params);
  if (!dict) {
    return VPD_ERR_DECODE;
  }

  *kv_dict = std::move(*dict);

  return VPD_OK;
}

vpd_err_t saveFile(const std::vector<uint8_t>& blob, const char* filename) {
  FILE* fp;
  vpd_err_t retval = VPD_OK;
  uint32_t file_seek;

  if (found_vpd) {
    /* We found VPD partition in -f file, which means file is existed.
     * Instead of truncating the whole file, open to write partial. */
    if (!(fp = fopen(filename, "r+"))) {
      PLOG(ERROR) << "File cannot be opened for write: " << filename;
      return VPD_ERR_SYSTEM;
    }
  } else {
    /* VPD is not found, which means the file is pure VPD data.
     * Always creates the new file and overwrites the original content. */
    if (!(fp = fopen(filename, "w+"))) {
      PLOG(ERROR) << "File cannot be opened for write: " << filename;
      return VPD_ERR_SYSTEM;
    }
  }

  file_seek = vpd_offset;

  /* write entire blob */
  fseek(fp, file_seek, SEEK_SET);
  if (fwrite(blob.data(), blob.size(), 1, fp) != 1) {
    PLOG(ERROR) << "fwrite() error";
    retval = VPD_ERR_SYSTEM;
    goto teardown;
  }

teardown:
  fclose(fp);

  return retval;
}

// We're just escaping single quotes within a single-quote string. i.e.,
// 'foo'bar' becomes 'foo'"'"'bar'.
std::string ShellSingleQuoteEscape(std::string s) {
  size_t pos = 0;
  std::string search("'");
  std::string replace("'\"'\"'");

  while ((pos = s.find(search, pos)) != std::string::npos) {
    s.replace(pos, search.size(), replace);
    pos += replace.size();
  }

  return s;
}

void usage(const char* progname) {
  std::cout
      << "Chrome OS VPD 2.0 utility --\n"
#ifdef VPD_VERSION
      << VPD_VERSION "\n"
      <<
#endif
      "\n"
      "Usage: "
      << progname << " [OPTION] ...\n"
      << "   OPTIONs include:\n"
         "      -h               This help page and version.\n"
         "      -f <filename>    The output file name.\n"
         "      -E <address>     EPS base address default:0x240000).\n"
         "      -S <key=file>    To add/change a string value, reading its\n"
         "                       base64 contents from a file.\n"
         "      -s <key=value>   To add/change a string value.\n"
         "      -i <partition>   Specify VPD partition name in fmap.\n"
         "      -l               List content in the file.\n"
         "      --sh             Dump content for shell script.\n"
         "      --raw            Parse from a raw blob (without headers).\n"
         "      -0/--null-terminated\n"
         "                       Dump content in null terminate format.\n"
         "      -O               Overwrite and re-format VPD partition.\n"
         "      -g <key>         Print value string only.\n"
         "      -d <key>         Delete a key.\n"
         "      --no-cache       Do not use the VPD cache; access the flash\n"
         "                       directly. Only applies to non-filebacked\n"
         "                       operations.\n"
         "\n"
         "   Notes:\n"
         "      You can specify multiple -s and -d. However, vpd always\n"
         "         applies -s first, then -d.\n"
         "      You can specify -O along with multiple -s and -d. However,\n"
         "          vpd always applies -O first, followed by -s, and then -d.\n"
         "      -g and -l must be mutually exclusive.\n";
}

}  // namespace

int main(int argc, char* argv[]) {
  int opt;
  int option_index = 0;
  vpd_err_t retval = VPD_OK;
  int export_type = VPD_EXPORT_KEY_VALUE;
  const char* optstring = "hf:s:S:i:lOg:d:0";
  static struct option long_options[] = {
      {"help", 0, 0, 'h'},
      {"file", 0, 0, 'f'},
      {"string", 0, 0, 's'},
      {"base64file", 0, 0, 'S'},
      {"partition", 0, 0, 'i'},
      {"list", 0, 0, 'l'},
      {"overwrite", 0, 0, 'O'},
      {"filter", 0, 0, 'g'},
      {"sh", 0, &export_type, VPD_EXPORT_AS_PARAMETER},
      {"raw", 0, 0, 'R'},
      {"null-terminated", 0, 0, '0'},
      {"delete", 0, 0, 'd'},
      {"no-cache", 0, 0, 'n'},
      {0, 0, 0, 0}};
  vpd::VpdRegion region = vpd::VpdRo;
  std::string region_name = "RO_VPD";
  std::optional<std::string> filename;
  std::optional<std::string> save_file;
  std::optional<std::string> key_to_export;
  /* Stores parsed pairs from command "set" arguments. */
  std::map<std::string, std::string> set_argument;
  /* Stores parsed keys (and empty value) from command "delete"
   * arguments. */
  std::map<std::string, std::string> del_argument;

  vpd::Flashrom flashrom;
  bool write_back_to_flash = false;
  bool list_it = false;
  bool overwrite_it = false;
  int modified = 0;
  bool read_from_file = false;
  bool raw_input = false;
  bool use_cache = true;

  absl::InitializeLog();

  while ((opt = getopt_long(argc, argv, optstring, long_options,
                            &option_index)) != EOF) {
    switch (opt) {
      case 'h':
        usage(argv[0]);
        return retval;

      case 'f':
        filename = std::string(optarg);
        break;

      case 'S':
        read_from_file = true;
        /* Fall through into the next case */
      case 's':
        retval =
            parseString(std::string(optarg), read_from_file, &set_argument);
        if (VPD_OK != retval) {
          LOG(ERROR) << "The string [" << optarg << "] cannot be parsed.";
          return retval;
        }
        read_from_file = false;
        break;

      case 'i':
        region_name = std::string(optarg);

        if (region_name == "RO_VPD") {
          region = vpd::VpdRo;
        } else if (region_name == "RW_VPD") {
          region = vpd::VpdRw;
        } else {
          LOG(ERROR) << "Invalid VPD partition name: " << region_name;
          return VPD_ERR_SYNTAX;
        }
        break;

      case 'l':
        list_it = true;
        break;

      case 'O':
        overwrite_it = true;
        /* This option forces to write empty data back even no new pair is
         * given. */
        modified = 1;
        break;

      case 'g':
        key_to_export = std::string(optarg);
        break;

      case 'd':
        /* Add key into container for delete. Since value is nonsense,
         * keep it empty. */
        del_argument[std::string(optarg)] = "";
        break;

      case '0':
        export_type = VPD_EXPORT_NULL_TERMINATE;
        break;

      case 'R':
        raw_input = true;
        break;

      case 'n':
        use_cache = false;
        break;

      case 0:
        break;

      default:
        LOG(ERROR) << absl::StrFormat("Invalid option '{%c}' ({%s})", opt,
                                      optarg ? optarg : "<no argument>");
        usage(argv[0]);
        return VPD_ERR_SYNTAX;
    }
  }

  if (optind < argc) {
    LOG(ERROR) << "unexpected argument: " << argv[optind];
    usage(argv[0]);
    return VPD_ERR_SYNTAX;
  }

  if (list_it && key_to_export) {
    LOG(ERROR) << "-l and -g must be mutually exclusive";
    return VPD_ERR_SYNTAX;
  }

  if (use_cache && (raw_input || filename)) {
    /* use_cache is default, but doesn't apply to raw or file-backed. Silently
     * disable. */
    use_cache = false;
  }

  if (VPD_EXPORT_KEY_VALUE != export_type && !list_it) {
    LOG(ERROR) << "--sh/--null-terminated can be set only if -l is set";
    return VPD_ERR_SYNTAX;
  }

  if (raw_input && !filename) {
    LOG(ERROR) << "Needs -f FILE for raw input";
    return VPD_ERR_SYNTAX;
  }

  if (raw_input &&
      (!set_argument.empty() || !del_argument.empty() || overwrite_it)) {
    LOG(ERROR) << "Changing in raw mode is not supported";
    return VPD_ERR_SYNTAX;
  }

  vpd::Cache cache;
  /* The original VDP blob got from either file or flashrom. It will not be used
   * if we enable caching. */
  std::vector<uint8_t> vpd_blob;
  /* kv_dict stores decoded pairs from VPD. When using caching, key-values pairs
   * are got from tmpfs cache. Otherwise key-values pairs are parsed from
   * vpd_blob.
   */
  std::map<std::string, std::string> kv_dict;
  if (filename) {
    auto blob = vpd::util::ReadFileToBytes(filename.value());
    if (!blob) {
      return VPD_FAIL;
    }
    vpd_blob = std::move(*blob);
    save_file = filename;
  } else if (use_cache) {
    if (key_to_export) {
      auto value = cache.GetValue(region, *key_to_export);
      if (value) {
        kv_dict[*key_to_export] = *value;
      }
    } else if (!overwrite_it) {
      /* Leave kv_dict empty if we want to overwrite. */
      kv_dict = cache.GetValues(region);
    }
  } else {
    /*
     * fall back to flashrom if we're not loading from a file, and we weren't
     * allowed to use the cache.
     */
    LOG(WARNING) << "Falling back to flashrom for reading the flash.";
    auto blob = flashrom.Read(region);
    if (!blob) {
      LOG(ERROR) << "flashrom read error";
      return VPD_ERR_ROM_READ;
    }
    vpd_blob = std::move(*blob);

    write_back_to_flash = true;
  }

  if (!use_cache) {
    if (raw_input) {
      auto res = vpd::Encoder::DecodeRaw(vpd_blob);
      if (!res) {
        return VPD_ERR_DECODE;
      }
      kv_dict = std::move(*res);
    } else {
      retval = loadFile(vpd_blob, region_name, region, flashrom, overwrite_it,
                        &kv_dict);
      if (VPD_OK != retval) {
        LOG(ERROR) << "loadFile() error: " << retval;
        return retval;
      }
    }
  }

  /* Do -s */
  if (!set_argument.empty()) {
    for (auto& pair : set_argument) {
      kv_dict[pair.first] = pair.second;
    }
    modified++;
  }

  /* Do -d */
  for (auto& pair : del_argument) {
    if (!kv_dict.erase(pair.first)) {
      LOG(ERROR) << "At least one of the keys to delete does not exist. "
                    "Command ignored.";
      return VPD_ERR_PARAM;
    }
  }
  if (!del_argument.empty()) {
    modified++;
  }

  /* Do -g */
  if (key_to_export) {
    const auto& pair = kv_dict.find(*key_to_export);
    if (pair == kv_dict.end()) {
      LOG(ERROR) << "key lookup: Vpd data '" << *key_to_export
                 << "' was not found";
      return VPD_FAIL;
    } else {
      fwrite(pair->second.data(), strlen(pair->second.data()), 1, stdout);
    }
  }

  /* Do -l */
  if (list_it) {
    std::string list;

    for (const auto& pair : kv_dict) {
      switch (export_type) {
        case VPD_EXPORT_AS_PARAMETER:
          list += "    -s '" + ShellSingleQuoteEscape(pair.first) + "'='" +
                  ShellSingleQuoteEscape(pair.second) + "'\\\n";
          break;
        case VPD_EXPORT_KEY_VALUE:
          list += "\"" + pair.first + "\"=\"" + pair.second + "\"\n";
          break;
        case VPD_EXPORT_NULL_TERMINATE:
          list += pair.first + "=" + pair.second + std::string({'\0'});
          break;
        default:
          // Shouldn't be reached.
          assert(0);
      }
    }

    /* Export necessary program parameters */
    if (VPD_EXPORT_AS_PARAMETER == export_type) {
      std::cout << SH_COMMENT << argv[0] << " -i " << region_name << " \\\n";

      if (filename)
        std::cout << "    -f " << *filename << " \\\n";
    }

    fwrite(list.data(), list.size(), 1, stdout);
  }

  if (modified && use_cache) {
    std::map<std::string, std::optional<std::string>> write_values;

    auto cached_pair = cache.GetValues(region);
    if (overwrite_it) {
      /* Marshall -O arguments first */
      for (const auto& pair : cached_pair) {
        /* nullopt means delete. */
        write_values[pair.first] = std::nullopt;
      }
    }

    /* Marshall -s arguments */
    for (const auto& pair : set_argument) {
      write_values[pair.first] = pair.second;
    }

    /* Marshall -d at the end, in case they cancel out -s. */
    for (const auto& pair : del_argument) {
      /* nullopt means delete. */
      write_values[pair.first] = std::nullopt;
    }

    /* Avoid attempting to delete pairs that do not exist when -s and -d used
     * on the same new keys. */
    for (auto write_it = write_values.begin();
         write_it != write_values.end();) {
      if (!write_it->second.has_value() &&
          cached_pair.find(write_it->first) == cached_pair.end()) {
        write_it = write_values.erase(write_it);
      } else {
        ++write_it;
      }
    }

    if (!cache.WriteValues(region, write_values)) {
      LOG(ERROR) << "Writeback failed";
      return VPD_ERR_ROM_WRITE;
    }

    /* If we used the cache, no need to go any further. */
    return VPD_OK;
  }

  if (modified) {
    auto blob = vpd::Encoder::Encode(
        vpd::Encoder::EncodingParams{.partition_offset = eps_base}, kv_dict);
    if (!blob) {
      LOG(ERROR) << "Encode() error";
      return VPD_FAIL;
    }
    if (save_file) {
      retval = saveFile(*blob, save_file->c_str());
      if (VPD_OK != retval) {
        LOG(ERROR) << "saveFile " << *filename << ", error: " << retval;
        return retval;
      }
    }

    if (write_back_to_flash) {
      if (!flashrom.Write(region, *blob)) {
        LOG(ERROR) << "flashrom write error";
        return VPD_ERR_ROM_WRITE;
      }
    }
  }

  return VPD_OK;
}

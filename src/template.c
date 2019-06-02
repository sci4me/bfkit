typedef unsigned char u8;
typedef int s32;
typedef unsigned long long u64;

static void assert(u8 b, char *msg);

#ifdef _WIN32
    #include <windows.h>

    static inline s32 get_page_size() {
        SYSTEM_INFO system_info;
        GetSystemInfo(&system_info);
        return system_info.dwPageSize;
    }

    static void* __alloc(u64 size) {
        s32 page_size = get_page_size();
        u64 total_size = (size + page_size - 1) & -page_size;
        total_size += page_size * 2;

        void *ptr = VirtualAlloc(0, total_size, MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);

        DWORD old;
        assert(VirtualProtect(ptr, page_size, PAGE_NOACCESS, &old), "Failed to protect underflow page");
        assert(VirtualProtect(ptr + total_size - page_size, page_size, PAGE_NOACCESS, &old), "Failed to protect overflow page");

        return ptr + page_size;
    }

    static void __free(void* ptr, u64 size) {
        s32 page_size = get_page_size();
        assert(VirtualFree(ptr - page_size, 0, MEM_RELEASE), "Failed to free memory");
    }

    #define __scan_left(tape, dp) while(**dp) { *dp -= 1; }
#else
    #define _GNU_SOURCE
    #include <string.h>
    #include <sys/mman.h>

    static void* __alloc(u64 size) {
        void *ptr = mmap(0, size, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
        assert(ptr != MAP_FAILED, "Failed to allocate memory");
        return ptr;
    }

    static void __free(void* ptr, u64 size) {
        assert(!munmap(ptr, size), "Failed to free memory");
    }

    #define __scan_left(tape, dp) *dp -= (u64)((void*) *dp - memrchr(tape, 0, (*dp - tape + 1)));
#endif

#include <stdio.h>
#include <stdlib.h>

static void assert(u8 b, char *msg) {
    if(!b) {
        fprintf(stderr, "Assertion failed: %s\n", msg);
        exit(1);
    }
}

static const u64 tape_size = __TAPE_SIZE__;

#define ADJUST(base_offset, delta) *(dp + base_offset) += delta;
#define SELECT(delta) dp += delta;
#define READ(base_offset) *(dp + base_offset) = getchar();
#define WRITE(base_offset) putchar(*(dp + base_offset)); fflush(stdout);
#define OPEN() while(*dp) {
#define CLOSE() }
#define SET(base_offset, value) *(dp + base_offset) = value;
#define MADD(offset, factor) *(dp + offset) += *dp * factor;
#define SCAN_LEFT() __scan_left(tape, &dp);
#define SCAN_RIGHT() dp += (u64)(memchr(dp, 0, tape_size - (dp - tape)) - (void*) dp);

int main() {
    u8 *tape = (u8*) __alloc(tape_size);
    u8 *dp = tape;

    __CODE__

    __free(tape, tape_size);
    return 0;
}